pub mod parse {
    use scraper::{Html, Selector};
    use crate::types::BookBox;
    use crate::wget::wget::fetch_url;

    pub fn get_books_from_html(body: String) -> Vec<BookBox> {
        // document.querySelectorAll('.bigBoxContent .bookBox');
        let book_box_selector = Selector::parse(".bigBoxContent .bookBox").unwrap();
        let image_alt = Selector::parse(r#"a img[alt]"#).unwrap();
        let document = Html::parse_document(&body);
        let book_boxes_elements = document.select(&book_box_selector);
        let mut book_boxes:Vec<BookBox> = Vec::new();

        for book_box_element in book_boxes_elements {
            let mut title = "";

            if let Some(alt) = book_box_element.select(&image_alt).next().unwrap().value().attr("alt") {
                title = alt;
            }

            let book_box = BookBox::new(title.to_string(), "".to_string(), "".to_string(), None, None);
            book_boxes.push(book_box);

            println!("Title: {}", title);
        }

        return book_boxes;
    }

    #[tokio::test]
    pub async fn can_get_book_boxes() {
        let body = fetch_url("https://storage.googleapis.com/ejangi-public-files/goodreads-groups-example.html".to_string()).await;
        let books = self::get_books_from_html(body.unwrap());

        assert!(books.len() > 0);

        for book in books {
            assert!(book.title.len() > 0);
        }
    }
}