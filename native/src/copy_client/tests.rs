use super::client::Client;
use anyhow::Result;
use base64::Engine;

const API_URL: &str = "aHR0cHM6Ly9hcGkuY29weW1hbmdhLm5ldA==";

fn api_url() -> String {
    String::from_utf8(base64::prelude::BASE64_STANDARD.decode(API_URL).unwrap()).unwrap()
}

#[tokio::test]
async fn test_client() -> Result<()> {
    let client = Client::new(
        reqwest::Client::builder()
            .proxy(reqwest::Proxy::all("http://127.0.0.1:1087").unwrap())
            .build()
            .unwrap(),
        api_url(),
    );
    let value: serde_json::Value = client
        .request(
            reqwest::Method::GET,
            "/api/v3/comic/fxzhanshijiuliumei/chapter2/d59724f2-d432-11eb-84f6-00163e0ca5bd",
            serde_json::json!({
                "platform": 3,
            }),
        )
        .await?;
    // page * 21, query, type
    println!("{}", serde_json::to_string(&value).unwrap());
    Ok(())
}
