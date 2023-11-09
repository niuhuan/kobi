use super::client::Client;
use anyhow::Result;
use base64::Engine;

const API_URL: &str = "aHR0cHM6Ly9hcGkuY29weW1hbmdhLm5ldA==";

fn api_url() -> String {
    String::from_utf8(base64::prelude::BASE64_STANDARD.decode(API_URL).unwrap()).unwrap()
}

fn client() -> Client {
    Client::new(
        reqwest::Client::builder()
            .proxy(reqwest::Proxy::all("http://127.0.0.1:1087").unwrap())
            .build()
            .unwrap(),
        api_url(),
    )
}

#[tokio::test]
async fn test_chapters() -> Result<()> {
    let value = client
        .comic_chapter("fxzhanshijiuliumei", "default", 100, 0)
        .await?;
    println!("{}", serde_json::to_string(&value).unwrap());
    Ok(())
}
