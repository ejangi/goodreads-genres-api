#[cfg(test)]
mod test {
    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{Status, ContentType};
    //use rocket::serde::{Serialize, Deserialize};
    use rocket::serde::json;
    use crate::types::BookBox;

    #[test]
    fn index() {
        let client = Client::tracked(rocket()).unwrap();
        let res = client.get("/?genres=science-fiction-fantasy&min_avg=4&min_count=6000").header(ContentType::JSON).dispatch();
        assert_eq!(res.status(), Status::Ok);
        let res_string = res.into_string().unwrap_or("".to_string());
        let res_first2: String = res_string.chars().take(2).collect();
        assert_eq!("[{".to_string(), res_first2);
        let books: Vec<BookBox> = json::from_str(&res_string).unwrap();
        // println!("Books: {}", books.len());
        assert_eq!(36, books.len());
        // println!("{}", res_string);
    }

    #[test]
    fn no_groups_sent() {
        let client = Client::tracked(rocket()).unwrap();
        let res = client.get("/").header(ContentType::JSON).dispatch();
        assert_eq!(res.status(), Status::Ok);
        let res_string = res.into_string().unwrap_or("".to_string());
        let res_first2: String = res_string.chars().take(2).collect();
        assert_eq!("[]".to_string(), res_first2);
    }

    #[test]
    fn four_o_four() {
        let client = Client::tracked(rocket()).unwrap();
        let res = client.get("/doesntexist").header(ContentType::JSON).dispatch();
        assert_eq!(res.status(), Status::NotFound);
        let res_string = res.into_string().unwrap_or("".to_string());
        println!("{}", res_string);
    }
}