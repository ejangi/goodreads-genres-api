
use crate::types::BookBox;
use regex::Regex;
use scraper::{Html, Selector};

const GOODREADS_BASENAME: &str = "https://goodreads.com";

pub fn get_books_from_html(body: String) -> Vec<BookBox> {
    let canonical_element = Selector::parse("link[rel=canonical]").unwrap();
    let book_box_selector = Selector::parse(".bigBoxContent .bookBox").unwrap();
    let image_element = Selector::parse(r#"a img[alt]"#).unwrap();
    let a_element = Selector::parse("a[href]").unwrap();
    let rating_avg_regex = Regex::new(r#"([0-5]{1}\.[0-9]+)\savg\srating"#).unwrap();
    let rating_count_regex = Regex::new(r#"([0-9,]{3,})\sratings"#).unwrap();
    let description_regex =
        Regex::new(r#"<span id=\\"freeText[0-9]+\\" style=\\"display:none\\">(.*)<\\/span>"#)
            .unwrap();

    let document = Html::parse_document(&body);
    let genre = document
        .select(&canonical_element)
        .next()
        .and_then(|element| element.value().attr("href"))
        .and_then(|canonical_href| canonical_href.rsplit('/').next())
        .filter(|genre| !genre.is_empty())
        .map(str::to_owned);

    let mut book_boxes: Vec<BookBox> = Vec::new();

    for book_box_element in document.select(&book_box_selector) {
        let title = match book_box_element
            .select(&image_element)
            .next()
            .and_then(|image_element| image_element.value().attr("alt"))
        {
            Some(title) => title.to_owned(),
            None => continue,
        };
        let href = match book_box_element
            .select(&a_element)
            .next()
            .and_then(|anchor| anchor.value().attr("href"))
        {
            Some(href) => href,
            None => continue,
        };
        let url = if href.starts_with("https") {
            href.to_owned()
        } else {
            format!("{GOODREADS_BASENAME}{href}")
        };
        let image_url = book_box_element
            .select(&image_element)
            .next()
            .and_then(|image_element| image_element.value().attr("src"))
            .unwrap_or_default()
            .to_owned();
        let mut description = String::from("");
        let mut rating_avg: Option<f32> = None;
        let mut rating_count: Option<i32> = None;

        if let Some(rating_avg_matches) = rating_avg_regex.captures(&book_box_element.html()) {
            if let Some(rating_avg_match) = rating_avg_matches.get(1) {
                rating_avg = Some(rating_avg_match.as_str().parse::<f32>().unwrap());
            }
        }

        if let Some(rating_count_matches) = rating_count_regex.captures(&book_box_element.html()) {
            if let Some(rating_count_match) = rating_count_matches.get(1) {
                rating_count = Some(
                    rating_count_match
                        .as_str()
                        .replace(",", "")
                        .parse::<i32>()
                        .unwrap(),
                );
            }
        }

        if let Some(description_matches) = description_regex.captures(&book_box_element.html()) {
            if let Some(description_match) = description_matches.get(1) {
                description = description_match.as_str().to_string();
            }
        }

        let book_box = BookBox::new(
            title,
            url,
            image_url,
            description,
            rating_avg,
            rating_count,
            genre.clone(),
        );
        book_boxes.push(book_box);
    }

    book_boxes
}

#[tokio::test]
pub async fn can_get_book_boxes() {
    use crate::wget::fetch_url;

    let body = fetch_url(
        "https://storage.googleapis.com/ejangi-public-files/goodreads-groups-example.html"
            .to_string(),
    )
    .await;
    let books = self::get_books_from_html(body.unwrap());

    assert!(!books.is_empty());

    for book in books {
        assert!(!book.title.is_empty());
        assert!(!book.url.is_empty());
        assert!(!book.image_url.is_empty());
        assert!(book.rating_avg.unwrap_or(0.0) > 0.0);
        assert!(book.rating_count.unwrap_or(0) > 0);
        assert!(book.genre.is_some_and(|genre| !genre.is_empty()));
    }
}
