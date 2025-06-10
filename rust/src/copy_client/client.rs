pub use super::types::*;
use super::{Browse, Comment, Roast};
use crate::copy_client::{
    BrowseComic, ChapterData, CollectedComic, ComicChapter, ComicData, ComicInExplore,
    ComicInSearch, ComicQuery, LoginResult, MemberInfo, Page, RankItem, RecommendItem,
    RegisterResult, Response, Tags,
};
use base64::Engine;
use chrono::Datelike;
use rand::prelude::IndexedRandom;
use rand::Rng;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Client {
    agent: Mutex<Arc<reqwest::Client>>,
    api_host: Mutex<Arc<String>>,
    token: Mutex<Arc<String>>,
    device: Mutex<Arc<String>>,
    device_info: Mutex<Arc<String>>,
}

impl Client {
    pub fn new(agent: impl Into<Arc<reqwest::Client>>, api_host: impl Into<String>) -> Self {
        Self {
            agent: Mutex::new(agent.into()),
            api_host: Mutex::new(Arc::new(api_host.into())),
            token: Mutex::new(Arc::new(String::new())),
            device: Mutex::new(Arc::new("".to_string())),
            device_info: Mutex::new(Arc::new("".to_string())),
        }
    }

    pub async fn set_device(&self, device: impl Into<String>, device_info: impl Into<String>) {
        let mut lock = self.device.lock().await;
        *lock = Arc::new(device.into());
        let mut info_lock = self.device_info.lock().await;
        *info_lock = Arc::new(device_info.into());
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
        let device_lock = self.device.lock().await;
        let device = device_lock.deref().deref().clone();
        drop(device_lock);
        let device_info_lock = self.device_info.lock().await;
        let device_info = device_info_lock.deref().deref().clone();
        drop(device_info_lock);
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
                    serde_json::Value::String(device.clone()),
                );
                obj.insert(
                    "umString".to_string(),
                    serde_json::Value::String("b4c89ca4104ea9a97750314d791520ac".to_string()),
                );
                obj.insert(
                    "deviceInfo".to_string(),
                    serde_json::Value::String(device_info.clone()),
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
            .header("device", device)
            .header("umstring", "b4c89ca4104ea9a97750314d791520ac")
            .header("deviceinfo", device_info)
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

    pub async fn browser(&self, offset: u64, limit: u64) -> Result<Page<BrowseComic>> {
        self.request(
            reqwest::Method::GET,
            "/api/v3/member/browse/comics",
            serde_json::json!({
                "limit": limit,
                "offset": offset,
                "platform": 3,
            }),
        )
        .await
    }
}

pub fn random_device() -> String {
    format!(
        "{}{}{}{}.{}{}{}{}{}{}.{}{}{}",
        (b'A' + rand::random::<u8>() % 26) as char,
        (b'A' + rand::random::<u8>() % 26) as char,
        (b'0' + rand::random::<u8>() % 10) as char,
        (b'A' + rand::random::<u8>() % 26) as char,
        (b'0' + rand::random::<u8>() % 10) as char,
        (b'0' + rand::random::<u8>() % 10) as char,
        (b'0' + rand::random::<u8>() % 10) as char,
        (b'0' + rand::random::<u8>() % 10) as char,
        (b'0' + rand::random::<u8>() % 10) as char,
        (b'0' + rand::random::<u8>() % 10) as char,
        (b'0' + rand::random::<u8>() % 10) as char,
        (b'0' + rand::random::<u8>() % 10) as char,
        (b'0' + rand::random::<u8>() % 10) as char,
    )
}

fn random_device_info() -> String {
    random_android_ua()
}

const ANDROID_VERSIONS: &[&str] = &[
    "4.4", "5.0", "5.1", "6.0", "7.0", "7.1", "8.0", "8.1", "9", "10", "11", "12", "12.1", "13",
    "14", "15",
];

// 常见设备名，包括模拟器和主流品牌型号
const DEVICES: &[&str] = &[
    "Android SDK built for arm64",
    "Android SDK built for x86",
    "Pixel 7 Pro",
    "Pixel 7",
    "Pixel 6 Pro",
    "Pixel 6",
    "Pixel 5",
    "Pixel 4 XL",
    "Pixel 4a",
    "Pixel 3",
    "Redmi Note 12 Pro",
    "Redmi Note 11",
    "Redmi K60",
    "Redmi 10X",
    "MI 13",
    "MI 12",
    "MI 11 Ultra",
    "MI 10",
    "MI 9",
    "HUAWEI Mate 60 Pro",
    "HUAWEI P60",
    "HUAWEI nova 12",
    "HUAWEI Mate 40",
    "HUAWEI P40",
    "HUAWEI Mate X5",
    "OPPO Find X7",
    "OPPO Reno11",
    "OPPO A78",
    "Vivo X100",
    "Vivo S18",
    "Vivo Y100",
    "OnePlus 12",
    "OnePlus 11",
    "OnePlus 9 Pro",
    "realme GT5",
    "realme 12 Pro",
    "Samsung Galaxy S24",
    "Samsung Galaxy S23 Ultra",
    "Samsung Galaxy S22",
    "Samsung Galaxy Note10+",
    "Meizu 21 Pro",
    "Meizu 20",
    "Lenovo Legion Y70",
    "Lenovo K12",
    "Sony Xperia 1V",
    "Sony Xperia 10V",
];

// 常见 Build 前缀（按 Android 版本/厂商编译习惯）
const BUILD_PREFIXES: &[&str] = &[
    "AE3A",
    "TP1A",
    "UP1A",
    "SP1A",
    "RQ2A",
    "QQ3A",
    "RP1A",
    "QP1A",
    "RKQ1",
    "PKQ1",
    "SQ3A",
    "TQ3A",
    "UQ1A",
    "VQ1A",
    "WW",
    "HMKQ1",
    "V12.5.2.0",
    "V13.0.1.0",
    "V14.0.4.0",
];

fn random_build_id() -> String {
    let mut rng = rand::rng();
    let prefix = BUILD_PREFIXES.choose(&mut rng).unwrap();
    let year = rng.random_range(20..=25);
    let month = rng.random_range(1..=12);
    let day = rng.random_range(1..=28);
    format!(
        "{}.{}{:02}{:02}.{:03}",
        prefix,
        year,
        month,
        day,
        rng.random_range(1..=999)
    )
}

fn random_fire_fox_version() -> String {
    let mut rng = rand::rng();
    let version = rng.random_range(85..=140);
    format!("{}", version)
}

fn random_android_ua() -> String {
    let mut rng = rand::rng();
    let android_version = ANDROID_VERSIONS.choose(&mut rng).unwrap();
    let device = DEVICES.choose(&mut rng).unwrap();
    let build_id = random_build_id();
    let firefox_version = random_fire_fox_version();
    format!(
        "Android {}; {} Build/{}/{}.0",
        android_version, device, build_id, firefox_version
    )
}
