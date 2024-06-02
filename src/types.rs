pub struct BookBox {
    pub title: String,
    pub url: String,
    pub image_url: String,
    pub rating_avg: Option<f32>,
    pub rating_count: Option<i32>
}

impl BookBox {
    pub fn new(title: String, url: String, image_url: String, rating_avg: Option<f32>, rating_count: Option<i32>) -> Self {
        BookBox {
            title,
            url,
            image_url,
            rating_avg,
            rating_count
        }
    }
}