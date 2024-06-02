pub mod wget;
use crate::wget::wget::fetch_all_urls;

#[tokio::main]
async fn main() {
    let urls = vec!["https://storage.googleapis.com/ejangi-public-files/goodreads-groups-example.html".to_string()];

    let responses = fetch_all_urls(urls).await;

    for response in &responses {
        if let Some(first_line) = response.lines().next() {
            println!("{}", first_line);
        }
    }
}
