pub mod wget {
    use tokio;
    use reqwest::Error;
    
    pub async fn fetch_all_urls(urls: Vec<String>) -> Vec<String> {
        let mut handles = Vec::new();
        
        for url in urls {
            let handle = tokio::spawn(async move {
                match fetch_url(url).await {
                    Ok(body) => body,
                    Err(_) => String::from("Error fetching URL"),
                }
            });
            handles.push(handle);
        }
    
        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(_) => results.push(String::from("Task failed")),
            }
        }
    
        results
    }

    pub async fn fetch_url(url: String) -> Result<String, Error> {
        let response = reqwest::get(&url).await?;
        let body = response.text().await?;
        Ok(body)
    }
    
    
    #[tokio::test]
    async fn get_urls() {
        let urls = vec!["https://ejangi.com".to_string(), "https://soulution.cloud".to_string()];

        let responses = fetch_all_urls(urls).await;

        for response in &responses {
            assert!(response.len() > 0);

            if let Some(first_line) = response.lines().next() {
                let first_9: String = first_line.trim().to_lowercase().chars().take(9).collect();

                assert_eq!("<!doctype".to_string(), first_9);
                // println!("{}", first_line);
            }
        }

        assert_eq!(2, responses.len());
    }
}