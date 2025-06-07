pub use super::types::*;
use super::{Comment, Roast};
use crate::copy_client::{
    ChapterData, CollectedComic, ComicChapter, ComicData, ComicInExplore, ComicInSearch,
    ComicQuery, LoginResult, MemberInfo, Page, RankItem, RecommendItem, RegisterResult, Response,
    Tags,
};
use base64::Engine;
use chrono::Datelike;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Client {
    agent: Mutex<Arc<reqwest::Client>>,
    api_host: Mutex<Arc<String>>,
    token: Mutex<Arc<String>>,
}

impl Client {
    pub fn new(agent: impl Into<Arc<reqwest::Client>>, api_host: impl Into<String>) -> Self {
        Self {
            agent: Mutex::new(agent.into()),
            api_host: Mutex::new(Arc::new(api_host.into())),
            token: Mutex::new(Arc::new(String::new())),
        }
    }

    pub async fn set_agent(&self, agent: impl Into<Arc<reqwest::Client>>) {
        let mut lock = self.agent.lock().await;
        *lock = agent.into();
    }

    pub async fn set_api_host(&self, api_host: impl Into<String>) {
        let mut lock = self.api_host.lock().await;
        *lock = Arc::new(api_host.into());
    }

    pub async fn api_host_string(&self) -> Arc<String> {
        let api_host = self.api_host.lock().await;
        api_host.clone()
    }

    pub async fn set_token(&self, token: impl Into<String>) {
        let mut lock = self.token.lock().await;
        *lock = Arc::new(token.into());
    }

    pub async fn get_token(&self) -> Arc<String> {
        let token = self.token.lock().await;
        token.clone()
    }

    pub async fn request<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        path: &str,
        mut params: serde_json::Value,
    ) -> Result<T> {
        let obj = params.as_object_mut().expect("query must be object");
        if !path.ends_with("/login") && !path.ends_with("/register") {
            if let reqwest::Method::POST = method {
                obj.insert(
                    "authorization".to_string(),
                    serde_json::Value::String(format!("Token {}", self.get_token().await.as_str())),
                );
                obj.insert(
                    "referer".to_string(),
                    serde_json::Value::String("com.copymanga.app-2.3.0".to_string()),
                );
                obj.insert(
                    "userAgent".to_string(),
                    serde_json::Value::String("COPY/2.3.0".to_string()),
                );
                obj.insert(
                    "source".to_string(),
                    serde_json::Value::String("copyApp".to_string()),
                );
                obj.insert(
                    "webp".to_string(),
                    serde_json::Value::String("1".to_string()),
                );
                obj.insert(
                    "version".to_string(),
                    serde_json::Value::String("2.3.0".to_string()),
                );
                obj.insert(
                    "region".to_string(),
                    serde_json::Value::String("1".to_string()),
                );
                obj.insert(
                    "accept".to_string(),
                    serde_json::Value::String("application/json".to_string()),
                );
                obj.insert(
                    "device".to_string(),
                    serde_json::Value::String("QSR1.210802.001".to_string()),
                );
                obj.insert(
                    "umString".to_string(),
                    serde_json::Value::String("b4c89ca4104ea9a97750314d791520ac".to_string()),
                );
                obj.insert(
                    "deviceInfo".to_string(),
                    serde_json::Value::String(
                        "Android SDK built for arm64-emulator64_arm64".to_string(),
                    ),
                );
                obj.insert(
                    "isGoogle".to_string(),
                    serde_json::Value::String("false".to_string()),
                );
                obj.insert(
                    "platform".to_string(),
                    serde_json::Value::String("3".to_string()),
                );
            }
        }
        let agent_lock = self.agent.lock().await;
        let agent = agent_lock.clone();
        drop(agent_lock);
        let request = agent.request(
            method.clone(),
            format!("{}{}", &self.api_host_string().await.as_str(), path),
        );
        let request = request
            .header(
                "authorization",
                format!("Token {}", self.get_token().await.as_str()),
            )
            .header("referer", "com.copymanga.app-2.3.0")
            .header("user-agent", "COPY/2.3.0")
            .header("source", "copyApp")
            .header("webp", "1")
            .header("version", "2.3.0")
            .header("region", "1")
            .header("platform", "3")
            .header("accept", "application/json")
            .header("device", "QSR1.210802.001")
            .header("umstring", "b4c89ca4104ea9a97750314d791520ac")
            .header("deviceinfo", "Android SDK built for arm64-emu64a")
            .header("dt", Self::dt());
        let request = match method {
            reqwest::Method::GET => request.query(&obj),
            _ => request.form(&obj),
        };
        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;
        if status.as_u16() == 404 {
            return Err(Error::message("404 Not found"));
        }
        println!("RESPONSE : {} {}", status, text);
        let value = serde_json::from_str(text.as_str())?;
        if let serde_json::Value::Object(value) = value {
            if value.len() == 1 {
                if let Some(serde_json::Value::String(detal)) = value.get("detail") {
                    return Err(Error::message(detal.to_string()));
                }
            }
        }
        let response: Response = serde_json::from_str(text.as_str())?;
        if response.code != 200 {
            return Err(Error::message(response.message));
        }
        Ok(serde_json::from_value(response.results)?)
    }

    fn dt() -> String {
        let now = chrono::Local::now();
        format!("{}.{}.{}", now.year(), now.month(), now.day(),)
    }

    pub async fn register(&self, username: &str, password: &str) -> Result<RegisterResult> {
        self.request(
            reqwest::Method::POST,
            "/api/v3/register",
            serde_json::json!({
                "username": username,
                "password": password,
                "source": "freeSite",
                "version": "2023.08.14",
                "platform": 3,
            }),
        )
        .await
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<LoginResult> {
        let salt = chrono::Local::now().timestamp_millis() % (u16::MAX as i64);
        let password_b64 =
            base64::prelude::BASE64_STANDARD.encode(format!("{}-{}", password, salt).as_bytes());
        self.request(
            reqwest::Method::POST,
            "/api/v3/login",
            serde_json::json!({
                "username": username,
                "password": password_b64,
                "salt": salt,
                "source": "freeSite",
                "version": "2023.08.14",
                "platform": 3,
            }),
        )
        .await
    }

    pub async fn member_info(&self) -> Result<MemberInfo> {
        self.request(
            reqwest::Method::GET,
            "/api/v3/member/info",
            serde_json::json!({
                "platform": 3,
            }),
        )
        .await
    }

    pub async fn tags(&self) -> Result<Tags> {
        self.request(
            reqwest::Method::GET,
            "/api/v3/h5/filter/comic/tags",
            serde_json::json!({
                "platform": 3,
            }),
        )
        .await
    }

    pub async fn comic_search(
        &self,
        q_type: &str,
        q: &str,
        offset: u64,
        limit: u64,
    ) -> Result<Page<ComicInSearch>> {
        self.request(
            reqwest::Method::GET,
            "/api/v3/search/comic",
            serde_json::json!({
                "platform": 3,
                "limit": limit,
                "offset": offset,
                "q": q,
                "q_type": q_type,
            }),
        )
        .await
    }

    pub async fn comic_rank(
        &self,
        date_type: &str,
        offset: u64,
        limit: u64,
    ) -> Result<Page<RankItem>> {
        self.request(
            reqwest::Method::GET,
            "/api/v3/ranks",
            serde_json::json!({
                "platform": 3,
                "date_type": date_type,
                "offset": offset,
                "limit": limit,
            }),
        )
        .await
    }

    pub async fn comic(&self, path_word: &str) -> Result<ComicData> {
        self.request(
            reqwest::Method::GET,
            format!("/api/v3/comic2/{path_word}").as_str(),
            serde_json::json!({
                 "platform": 3,
            }),
        )
        .await
    }

    pub async fn comic_chapter(
        &self,
        comic_path_word: &str,
        group_path_word: &str,
        limit: u64,
        offset: u64,
    ) -> Result<Page<ComicChapter>> {
        self.request(
            reqwest::Method::GET,
            format!("/api/v3/comic/{comic_path_word}/group/{group_path_word}/chapters").as_str(),
            serde_json::json!({
                "offset": offset,
                "limit": limit,
                "platform": 3,
            }),
        )
        .await
    }

    pub async fn comic_query(&self, path_word: &str) -> Result<ComicQuery> {
        self.request(
            reqwest::Method::GET,
            format!("/api/v3/comic2/{path_word}/query").as_str(),
            serde_json::json!({
                 "platform": 3,
            }),
        )
        .await
    }

    pub async fn comic_chapter_data(
        &self,
        comic_path_word: &str,
        chapter_uuid: &str,
    ) -> Result<ChapterData> {
        self.request(
            reqwest::Method::GET,
            format!("/api/v3/comic/{comic_path_word}/chapter2/{chapter_uuid}").as_str(),
            serde_json::json!({
                 "platform": 3,
            }),
        )
        .await
    }

    pub async fn recommends(&self, offset: u64, limit: u64) -> Result<Page<RecommendItem>> {
        self.request(
            reqwest::Method::GET,
            "/api/v3/recs",
            serde_json::json!({
                "pos": 3200102,
                "limit": limit,
                "offset": offset,
                "platform": 3,
            }),
        )
        .await
    }

    pub async fn explore_by_author_name(
        &self,
        author_name: &str,
        ordering: Option<&str>,
        offset: u64,
        limit: u64,
    ) -> Result<Page<ComicInExplore>> {
        let mut params = serde_json::json!({
            "offset": offset,
            "limit": limit,
            "q": author_name,
            "free_type": 1,
            "platform": 3,
        });
        if let Some(ordering) = ordering {
            params["ordering"] = serde_json::json!(ordering);
        }
        self.request(reqwest::Method::GET, "/api/v3/comics", params)
            .await
    }

    pub async fn explore_by_author(
        &self,
        author: &str,
        ordering: Option<&str>,
        offset: u64,
        limit: u64,
    ) -> Result<Page<ComicInExplore>> {
        let mut params = serde_json::json!({
            "offset": offset,
            "limit": limit,
            "author": author,
            "free_type": 1,
            "platform": 3,
        });
        if let Some(ordering) = ordering {
            params["ordering"] = serde_json::json!(ordering);
        }
        self.request(reqwest::Method::GET, "/api/v3/comics", params)
            .await
    }

    pub async fn explore(
        &self,
        ordering: Option<&str>,
        top: Option<&str>,
        theme: Option<&str>,
        offset: u64,
        limit: u64,
    ) -> Result<Page<ComicInExplore>> {
        let mut params = serde_json::json!({
            "offset": offset,
            "limit": limit,
            "platform": 3,
            "_update": true,
        });
        if let Some(ordering) = ordering {
            params["ordering"] = serde_json::json!(ordering);
        }
        if let Some(top) = top {
            params["top"] = serde_json::json!(top);
        }
        if let Some(theme) = theme {
            params["theme"] = serde_json::json!(theme);
        }
        self.request(reqwest::Method::GET, "/api/v3/comics", params)
            .await
    }

    pub async fn collect(&self, comic_id: &str, is_collect: bool) -> Result<()> {
        self.request(
            reqwest::Method::POST,
            format!("/api/v3/member/collect/comic").as_str(),
            serde_json::json!({
                "comic_id": comic_id,
                "is_collect": if is_collect { 1 } else { 0 },
            }),
        )
        .await
    }

    pub async fn collected_comics(
        &self,
        free_type: i64,
        ordering: &str,
        offset: u64,
        limit: u64,
    ) -> Result<Page<CollectedComic>> {
        self.request(
            reqwest::Method::GET,
            "/api/v3/member/collect/comics",
            serde_json::json!({
                "free_type": free_type,
                "limit": limit,
                "offset": offset,
                "_update": true,
                "ordering": ordering,
                "platform": 3,
            }),
        )
        .await
    }

    pub async fn download_image(&self, url: &str) -> Result<bytes::Bytes> {
        let agent_lock = self.agent.lock().await;
        let agent = agent_lock.clone();
        drop(agent_lock);
        Ok(agent.get(url).send().await?.bytes().await?)
    }

    pub async fn roasts(&self, chapter_id: &str) -> Result<Page<Roast>> {
        self.request(
            reqwest::Method::GET,
            "/api/v3/roasts",
            serde_json::json!({
                "chapter_id": chapter_id,
                "limit": 10,
                "offset": 0,
                "platform": 3,
            }),
        )
        .await
    }

    pub async fn comments(
        &self,
        comic_id: &str,
        reply_id: Option<&str>,
        offset: u64,
        limit: u64,
    ) -> Result<Page<Comment>> {
        self.request(
            reqwest::Method::GET,
            "/api/v3/comments",
            serde_json::json!({
                "comic_id": comic_id,
                "reply_id": if let Some(reply_id) = reply_id { reply_id } else { "" },
                "limit": limit,
                "offset": offset,
                "platform": 3,
            }),
        )
        .await
    }

    pub async fn comment(
        &self,
        comic_id: &str,
        comment: &str,
        reply_id: Option<&str>,
    ) -> Result<()> {
        self.request(
            reqwest::Method::POST,
            "/api/v3/member/comment",
            serde_json::json!({
                "comic_id": comic_id,
                "comment": comment,
                "reply_id": if let Some(reply_id) = reply_id { reply_id } else { "" },
                "platform": 3,
            }),
        )
        .await
    }
}
