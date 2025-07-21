use super::client::Client;
use anyhow::Result;
use base64::Engine;
use serde_json::json;

const API_URL: &str = "aHR0cHM6Ly9hcGkuY29weTIwMDAub25saW5l";

fn api_url() -> String {
    String::from_utf8(base64::prelude::BASE64_STANDARD.decode(API_URL).unwrap()).unwrap()
}

fn client() -> Client {
    Client::new(reqwest::Client::builder().build().unwrap(), api_url())
}

#[tokio::test]
async fn test_request() -> Result<()> {
    let value = client()
        .request(
            reqwest::Method::GET,
            "/api/v3/comics",
            json!({
                "_update": true,
                "limit": 21,
                "offset": 42,
                "platform": 3,
            }),
        )
        .await?;
    println!("{}", serde_json::to_string(&value).unwrap());
    Ok(())
}

#[tokio::test]
async fn test_comic() -> Result<()> {
    let value = client().comic("dokunidakareteoboreteitai").await?;
    println!("{}", serde_json::to_string(&value).unwrap());
    Ok(())
}

#[tokio::test]
async fn test_chapters() -> Result<()> {
    let value = client()
        .comic_chapter("fxzhanshijiuliumei", "default", 100, 0)
        .await?;
    println!("{}", serde_json::to_string(&value).unwrap());
    Ok(())
}

#[tokio::test]
async fn test_recommends() -> Result<()> {
    let value = client().recommends(0, 21).await?;
    println!("{}", serde_json::to_string(&value).unwrap());
    Ok(())
}

#[tokio::test]
async fn test_explore() -> Result<()> {
    let value = client()
        .explore(Some("-datetime_updated"), None, None, 0, 21)
        .await?;
    println!("{}", serde_json::to_string(&value).unwrap());
    Ok(())
}

#[tokio::test]
async fn test_collect() -> Result<()> {
    let client = client();
    client.set_token("token").await;
    let value = client
        .collect("9581bff2-3892-11ec-8e8b-024352452ce0", true)
        .await?;
    println!("{}", serde_json::to_string(&value).unwrap());
    Ok(())
}

#[tokio::test]
async fn test_collected_comics() -> Result<()> {
    let client = client();
    client.set_token("token").await;
    let value = client
        .collected_comics(1, "-datetime_modifier", 0, 21)
        .await?;
    println!("{}", serde_json::to_string(&value).unwrap());
    Ok(())
}

#[tokio::test]
async fn test_login() -> Result<()> {
    let client = client();
     client.login("", "").await?;
    let token = client.get_token().await;
    println!("{}", token.as_str());
    Ok(())
}
