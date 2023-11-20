use anyhow::Result;
use std::collections::HashMap;
use std::process::exit;

const UA: &str = "actions ci";

#[tokio::main]
async fn main() -> Result<()> {
    let gh_token = std::env::var("GITHUB_TOKEN")?;
    if gh_token.is_empty() {
        panic!("Please set GITHUB_TOKEN");
    }

    let repo = std::env::var("GITHUB_REPOSITORY")?;
    if repo.is_empty() {
        panic!("Can't got repo path");
    }

    let branch = std::env::var("GITHUB_HEAD_REF")?;
    if repo.is_empty() {
        panic!("Can't got repo branch");
    }

    let vs_code_txt = tokio::fs::read_to_string("version.code.txt").await?;
    let vs_info_txt = tokio::fs::read_to_string("version.info.txt").await?;

    let code = vs_code_txt.trim();
    let info = vs_info_txt.trim();

    let client = reqwest::ClientBuilder::new().user_agent(UA).build()?;

    let release_url = format!("https://api.github.com/repos/{repo}/releases/tags/{code}");
    let check_response = client.get(release_url).send().await?;

    match check_response.status().as_u16() {
        200 => {
            println!("release exists");
            exit(0);
        }
        404 => (),
        code => {
            let text = check_response.text().await?;
            panic!("error for check release : {} : {}", code, text);
        }
    }
    drop(check_response);

    // 404

    let releases_url = format!("https://api.github.com/repos/{repo}/releases");
    let check_response = client
        .post(releases_url)
        .header("Authorization", format!("token {}", gh_token))
        .json(&{
            let mut params = HashMap::<String, String>::new();
            params.insert("tag_name".to_string(), code.to_string());
            params.insert("target_commitish".to_string(), branch);
            params.insert("name".to_string(), code.to_string());
            params.insert("body".to_string(), info.to_string());
            params
        })
        .send()
        .await?;

    match check_response.status().as_u16() {
        201 => (),
        code => {
            let text = check_response.text().await?;
            panic!("error for create release : {} : {}", code, text);
        }
    }
    Ok(())
}
