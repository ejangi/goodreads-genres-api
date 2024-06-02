pub mod parse {
    use scraper::{Html, Selector};
    use crate::types::BookBox;

    const GOODREADS_BASENAME: &str = "https://goodreads.com";

    pub fn get_books_from_html(body: String) -> Vec<BookBox> {
        // document.querySelectorAll('.bigBoxContent .bookBox');
        let book_box_selector = Selector::parse(".bigBoxContent .bookBox").unwrap();
        let image_element = Selector::parse(r#"a img[alt]"#).unwrap();
        let a_element = Selector::parse("a[href]").unwrap();
        let document = Html::parse_document(&body);
        let book_boxes_elements = document.select(&book_box_selector);
        let mut book_boxes:Vec<BookBox> = Vec::new();

        for book_box_element in book_boxes_elements {
            let mut title = String::from("");
            let mut url = String::from("");
            let mut image_url = String::from("");

            if let Some(alt) = book_box_element.select(&image_element).next().unwrap().value().attr("alt") {
                title = alt.to_string();
            }

            if let Some(href) = book_box_element.select(&a_element).next().unwrap().value().attr("href") {
                url = href.to_string();
            }

            let url_first_6: String = url.chars().take(6).collect();

            if url.len() > 5 && url_first_6 != "https" {
                url = GOODREADS_BASENAME.to_string() + &url;
            }

            if let Some(src) = book_box_element.select(&image_element).next().unwrap().value().attr("src") {
                image_url = src.to_string();
            }

            let book_box = BookBox::new(title.to_string(), image_url.to_string(), "".to_string(), None, None);
            book_boxes.push(book_box);

            println!("URL: {}", url);
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
        }
    }
}