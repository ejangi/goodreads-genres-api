#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;
pub mod wget;
pub mod parse;
pub mod types;
use crate::wget::wget::fetch_all_urls;
use crate::parse::parse::get_books_from_html;
use crate::types::BookBox;
use rocket::serde::json::{Json, Value,json};

const GOODREADS_GENRES_BASEURL: &str = "https://www.goodreads.com/genres/most_read";

#[get("/?<genres>&<sort_by>&<min_avg>&<min_count>")]
async fn index(genres: Option<String>, sort_by: Option<String>, min_avg: Option<f32>, min_count: Option<i32>) -> Json<Vec<BookBox>> {
    // let urls = vec!["https://storage.googleapis.com/ejangi-public-files/goodreads-groups-example.html".to_string()];
    let mut urls: Vec<String> = Vec::new();
    let mut books = Vec::new();

    match genres {
        Some(genres) => {
            let splits = genres.split(",");

            for g in splits {
                if g.len() > 0 {
                    let url: String = GOODREADS_GENRES_BASEURL.to_string() + "/" + g;
                    urls.push(url);
                }
            }

        },
        None => {
            json!(books);
        }
    }

    if urls.len() == 0 {
        json!(books);
    }

    let responses = fetch_all_urls(urls).await;
    
    for response in &responses {
        let book_boxes = get_books_from_html(response.to_string());
        books.extend(book_boxes);
    }

    match sort_by {
        Some(sort_by) => {
            if sort_by == "avg".to_string() {
                books.sort_by(|a, b| b.rating_avg.unwrap_or(0.0).partial_cmp(&a.rating_avg.unwrap_or(0.0)).unwrap());
            }
            else if sort_by == "count".to_string() {
                books.sort_by(|a, b| b.rating_count.unwrap_or(0).cmp(&a.rating_count.unwrap_or(0)));
            }
        },
        None => {
            books.sort_by(|a, b| b.rating_count.unwrap_or(0).cmp(&a.rating_count.unwrap_or(0)));
        }
    }

    match min_avg {
        Some(min_avg) => {
            if min_avg > 0.0 && min_avg < 5.0 {
                books = books.into_iter().filter(|b| b.rating_avg.unwrap_or(0.0) > min_avg).collect();
            }
        },
        None => {}
    }

    match min_count {
        Some(min_count) => {
            if min_count > 0 {
                books = books.into_iter().filter(|b| b.rating_count.unwrap_or(0) > min_count).collect();
            }
        },
        None => {}
    }

    Json(books)
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![not_found])
}