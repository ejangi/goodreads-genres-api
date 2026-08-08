#[macro_use]
extern crate rocket;
pub mod parse;
#[cfg(test)]
mod tests;
pub mod types;
pub mod wget;
use crate::parse::get_books_from_html;
use crate::types::BookBox;
use crate::wget::fetch_all_urls;
use rocket::serde::json::{json, Json, Value};

const GOODREADS_GENRES_BASEURL: &str = "https://www.goodreads.com/genres/most_read";

#[get("/?<genres>&<sort_by>&<min_avg>&<min_count>")]
async fn index(
    genres: Option<String>,
    sort_by: Option<String>,
    min_avg: Option<f32>,
    min_count: Option<i32>,
) -> Json<Vec<BookBox>> {
    let mut books = Vec::new();

    let Some(genres) = genres else {
        return Json(books);
    };

    let urls = genres
        .split(',')
        .filter(|genre| !genre.is_empty())
        .map(|genre| format!("{GOODREADS_GENRES_BASEURL}/{genre}"))
        .collect::<Vec<_>>();

    if urls.is_empty() {
        return Json(books);
    }

    for response in fetch_all_urls(urls).await {
        let book_boxes = get_books_from_html(response);
        books.extend(book_boxes);
    }

    if sort_by.as_deref() == Some("avg") {
        books.sort_by(|a, b| {
            b.rating_avg
                .unwrap_or(0.0)
                .total_cmp(&a.rating_avg.unwrap_or(0.0))
        });
    } else {
        books.sort_by(|a, b| {
            b.rating_count
                .unwrap_or(0)
                .cmp(&a.rating_count.unwrap_or(0))
        });
    }

    if let Some(min_avg) = min_avg {
        if min_avg > 0.0 && min_avg < 5.0 {
            books.retain(|book| book.rating_avg.unwrap_or(0.0) > min_avg);
        }
    }

    if let Some(min_count) = min_count {
        if min_count > 0 {
            books.retain(|book| book.rating_count.unwrap_or(0) > min_count);
        }
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
