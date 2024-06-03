pub mod parse {
    use scraper::{Html, Selector};
    use crate::types::BookBox;
    use regex::Regex;

    const GOODREADS_BASENAME: &str = "https://goodreads.com";

    pub fn get_books_from_html(body: String) -> Vec<BookBox> {
        // document.querySelectorAll('.bigBoxContent .bookBox');
        let book_box_selector = Selector::parse(".bigBoxContent .bookBox").unwrap();
        let image_element = Selector::parse(r#"a img[alt]"#).unwrap();
        let a_element = Selector::parse("a[href]").unwrap();
        let rating_avg_regex = Regex::new(r#"([0-5]{1}\.[0-9]+)\savg\srating"#).unwrap();
        let rating_count_regex = Regex::new(r#"([0-9,]{3,})\sratings"#).unwrap();
        let canonical_element = Selector::parse("link[rel=canonical]").unwrap();

        let document = Html::parse_document(&body);
        let book_boxes_elements = document.select(&book_box_selector);
        let mut book_boxes:Vec<BookBox> = Vec::new();
        let mut genre:Option<String> = None;
        
        if let Some(canonical_href) = document.select(&canonical_element).next().unwrap().value().attr("href") {
            let canonical_split = canonical_href.split("/");
            if let Some(canonical_last) = canonical_split.last() {
                genre = Some(canonical_last.to_string());
            }

        }

        for book_box_element in book_boxes_elements {
            let mut title = String::from("");
            let mut url = String::from("");
            let mut image_url = String::from("");
            let mut rating_avg: Option<f32> = None;
            let mut rating_count: Option<i32> = None;

            if let Some(alt) = book_box_element.select(&image_element).next().unwrap().value().attr("alt") {
                title = alt.to_string();
            }

            if let Some(href) = book_box_element.select(&a_element).next().unwrap().value().attr("href") {
                url = href.to_string();
                let url_first_6: String = url.chars().take(6).collect();
    
                if url.len() > 5 && url_first_6 != "https" {
                    url = GOODREADS_BASENAME.to_string() + &url;
                }
            }


            if let Some(src) = book_box_element.select(&image_element).next().unwrap().value().attr("src") {
                image_url = src.to_string();
            }

            if let Some(rating_avg_matches) = rating_avg_regex.captures(&book_box_element.html()) {
                if let Some(rating_avg_match) = rating_avg_matches.get(1) {
                    rating_avg = Some(rating_avg_match.as_str().parse::<f32>().unwrap());
                }
            }

            if let Some(rating_count_matches) = rating_count_regex.captures(&book_box_element.html()) {
                if let Some(rating_count_match) = rating_count_matches.get(1) {
                    rating_count = Some(rating_count_match.as_str().replace(",", "").parse::<i32>().unwrap());
                }
            }


            let book_box = BookBox::new(title, url, image_url, rating_avg, rating_count, genre.clone());
            book_boxes.push(book_box);
        }

        return book_boxes;
    }

    #[tokio::test]
    pub async fn can_get_book_boxes() {
        use crate::wget::wget::fetch_url;

        let body = fetch_url("https://storage.googleapis.com/ejangi-public-files/goodreads-groups-example.html".to_string()).await;
        let books = self::get_books_from_html(body.unwrap());

        assert!(books.len() > 0);

        for book in books {
            assert!(book.title.len() > 0);
            assert!(book.url.len() > 0);
            assert!(book.image_url.len() > 0);
            assert!(book.rating_avg.unwrap_or(0.0) > 0.0);
            assert!(book.rating_count.unwrap_or(0) > 0);
            assert!(book.genre.unwrap_or("".to_string()).len() > 0);
        }
    }
}