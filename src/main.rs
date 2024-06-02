pub mod wget;
pub mod parse;
pub mod types;
use crate::wget::wget::fetch_all_urls;
use crate::parse::parse::get_books_from_html;

#[tokio::main]
async fn main() {
    let urls = vec!["https://storage.googleapis.com/ejangi-public-files/goodreads-groups-example.html".to_string()];
    let responses = fetch_all_urls(urls).await;

    for response in &responses {
        assert!(response.len() > 0);

        if let Some(first_line) = response.lines().next() {
            let first_9: String = first_line.trim().to_lowercase().chars().take(9).collect();
            assert_eq!("<!doctype".to_string(), first_9);
            // println!("{}", first_line);
        }
    }
}
