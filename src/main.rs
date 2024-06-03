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

#[get("/?<groups>")]
async fn index(groups: Option<String>) -> Json<Vec<BookBox>> {
    // let urls = vec!["https://storage.googleapis.com/ejangi-public-files/goodreads-groups-example.html".to_string()];
    let mut urls: Vec<String> = Vec::new();
    let mut books = Vec::new();

    match groups {
        Some(groups) => {
            let splits = groups.split(",");

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