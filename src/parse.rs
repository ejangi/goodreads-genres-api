pub mod parse {
    use scraper::{Html, Selector};
    use crate::types::BookBox;
    use crate::wget::wget::fetch_url;

    pub fn get_books_from_html(body: String) -> Vec<BookBox> {
        // document.querySelectorAll('.bigBoxContent .bookBox');
        let book_box_selector = Selector::parse(".bigBoxContent .bookBox").unwrap();
        let image_element = Selector::parse(r#"a img[alt]"#).unwrap();
        let document = Html::parse_document(&body);
        let book_boxes_elements = document.select(&book_box_selector);
        let mut book_boxes:Vec<BookBox> = Vec::new();

        for book_box_element in book_boxes_elements {
            let mut title = "";
            let mut image_url = "";

            if let Some(alt) = book_box_element.select(&image_element).next().unwrap().value().attr("alt") {
                title = alt;
            }

            if let Some(src) = book_box_element.select(&image_element).next().unwrap().value().attr("src") {
                image_url = src;
            }

            let book_box = BookBox::new(title.to_string(), image_url.to_string(), "".to_string(), None, None);
            book_boxes.push(book_box);

            println!("Image: {}", image_url);
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