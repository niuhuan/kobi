use super::client::Client;
use anyhow::Result;

#[tokio::test]
async fn test_client() -> Result<()> {
    let client = Client::new(
        reqwest::Client::builder()
            .proxy(reqwest::Proxy::all("http://127.0.0.1:1087").unwrap())
            .build()
            .unwrap(),
        "https://api.copymanga.net",
    );
    // let value: serde_json::Value = client
    //     .request(
    //         reqwest::Method::GET,
    //         "/api/v3/h5/filter/comic/tags",
    //         serde_json::json!({
    //             "platform": 3,
    //         }),
    //     )
    //     .await?;
    let value: serde_json::Value = client
        .request(
            reqwest::Method::GET,
            "/api/v3/ranks",
            serde_json::json!({
                "platform": 3,
                "limit": 21,
                "offset": 21,
                "date_type": "month",
            }),
        )
        .await?;
    // page * 21, query, type
    println!("{}", serde_json::to_string(&value).unwrap());
    Ok(())
}
