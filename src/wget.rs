use reqwest::{Client, Error};
use tokio;

const USER_AGENT: &str =
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 Chrome/131.0.0.0 Safari/537.36";

pub async fn fetch_all_urls(urls: Vec<String>) -> Vec<String> {
    let client = client();
    let mut handles = Vec::new();

    for url in urls {
        let client = client.clone();
        let handle = tokio::spawn(async move {
            match fetch_url_with_client(client, url).await {
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
    fetch_url_with_client(client(), url).await
}

fn client() -> Client {
    Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .expect("the static HTTP client configuration should be valid")
}

async fn fetch_url_with_client(client: Client, url: String) -> Result<String, Error> {
    let response = client.get(url).send().await?.error_for_status()?;
    let body = response.bytes().await?;

    Ok(String::from_utf8_lossy(&body).into_owned())
}

#[tokio::test]
async fn get_urls() {
    let urls = vec![
        "https://example.com".to_string(),
        "https://example.com".to_string(),
    ];

    let responses = fetch_all_urls(urls).await;

    for response in &responses {
        assert!(!response.is_empty());

        if let Some(first_line) = response.lines().next() {
            let first_9: String = first_line.trim().to_lowercase().chars().take(9).collect();

            assert_eq!("<!doctype".to_string(), first_9);
            // println!("{}", first_line);
        }
    }

    assert_eq!(2, responses.len());
}

#[tokio::test]
async fn client_sends_goodreads_user_agent() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut buffer = vec![0; 4096];
        let bytes_read = stream.read(&mut buffer).await.unwrap();
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .await
            .unwrap();

        String::from_utf8_lossy(&buffer[..bytes_read]).into_owned()
    });

    fetch_url(format!("http://{address}")).await.unwrap();
    let request = server.await.unwrap();

    assert!(request
        .lines()
        .any(|line| line.eq_ignore_ascii_case(&format!("user-agent: {USER_AGENT}"))));
}

#[tokio::test]
async fn fetch_url_decodes_mislabelled_goodreads_html_as_utf8() {
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        stream
            .write_all(
                b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=iso-8859-1\r\nContent-Length: 7\r\nConnection: close\r\n\r\nwon\xe2\x80\x99t",
            )
            .await
            .unwrap();
    });

    let body = fetch_url(format!("http://{address}")).await.unwrap();
    server.await.unwrap();

    assert_eq!(body, "won’t");
}
