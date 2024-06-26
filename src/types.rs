use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct BookBox {
    pub title: String,
    pub url: String,
    pub image_url: String,
    pub description: String,
    pub rating_avg: Option<f32>,
    pub rating_count: Option<i32>,
    pub genre: Option<String>
}

impl BookBox {
    pub fn new(title: String, url: String, image_url: String, description: String, rating_avg: Option<f32>, rating_count: Option<i32>, genre: Option<String>) -> Self {
        BookBox {
            title,
            url,
            image_url,
            description,
            rating_avg,
            rating_count,
            genre
        }
    }

    pub fn fields_list() -> Vec<String> {
        let fields = std::any::type_name::<BookBox>().to_string();
        return fields.split(">::").map(|f| f.to_string()).collect();
    }
}