pub mod api;
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
use crate::api::api::save_property;
use crate::database::init_database;
use base64::Engine;
use copy_client::Client;
use database::properties::property;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use tokio::runtime;
use utils::create_dir_if_not_exists;
use utils::join_paths;

pub mod copy_client;
mod database;
pub mod downloading;
mod exports;
mod udto;
mod utils;

const OLD_API_URL: [&str; 2] = [
    "aHR0cHM6Ly93d3cuY29weS1tYW5nYS5jb20=",
    "aHR0cHM6Ly93d3cuY29weTIwLmNvbQ==",
];
const API_URL: &str = "aHR0cHM6Ly9hcGkuY29weTIwMDAub25saW5l";

fn api_url() -> String {
    String::from_utf8(base64::prelude::BASE64_STANDARD.decode(API_URL).unwrap()).unwrap()
}

lazy_static! {
    pub(crate) static ref RUNTIME: runtime::Runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_keep_alive(tokio::time::Duration::new(60, 0))
        .worker_threads(30)
        .max_blocking_threads(30)
        .build()
        .unwrap();
    pub(crate) static ref CLIENT: Arc<Client> = Arc::new(Client::new(
        reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(60))
            .connect_timeout(Duration::from_secs(30))
            .read_timeout(Duration::from_secs(30))
            .build()
            .unwrap(),
        api_url()
    ));
    static ref INIT_ED: Mutex<bool> = Mutex::new(false);
}

static ROOT: OnceCell<String> = OnceCell::new();
static IMAGE_CACHE_DIR: OnceCell<String> = OnceCell::new();
static DATABASE_DIR: OnceCell<String> = OnceCell::new();
static DOWNLOAD_DIR: OnceCell<String> = OnceCell::new();

pub fn init_root(path: &str) {
    let mut lock = INIT_ED.lock().unwrap();
    if *lock {
        return;
    }
    *lock = true;
    println!("Init application with root : {}", path);
    ROOT.set(path.to_owned()).unwrap();
    IMAGE_CACHE_DIR
        .set(join_paths(vec![path, "image_cache"]))
        .unwrap();
    DATABASE_DIR
        .set(join_paths(vec![path, "database"]))
        .unwrap();
    DOWNLOAD_DIR
        .set(join_paths(vec![path, "download"]))
        .unwrap();
    create_dir_if_not_exists(ROOT.get().unwrap());
    create_dir_if_not_exists(IMAGE_CACHE_DIR.get().unwrap());
    create_dir_if_not_exists(DATABASE_DIR.get().unwrap());
    create_dir_if_not_exists(DOWNLOAD_DIR.get().unwrap());
    RUNTIME.block_on(init_database());
    RUNTIME.block_on(reset_api());
    RUNTIME.block_on(load_api());
    RUNTIME.block_on(init_device());
    RUNTIME.block_on(init_header());
    RUNTIME.spawn(sync_header());
    RUNTIME.block_on(async {
        *downloading::DOWNLOAD_AND_EXPORT_TO.lock().await =
            database::properties::property::load_property("download_and_export_to".to_owned())
                .await
                .unwrap()
    });
    RUNTIME.block_on(async {
        *downloading::PAUSE_FLAG.lock().await =
            database::properties::property::load_property("download_pause".to_owned())
                .await
                .unwrap()
                == "true"
    });
    RUNTIME.spawn(downloading::start_download());
}

#[allow(dead_code)]
pub(crate) fn get_root() -> &'static String {
    ROOT.get().unwrap()
}

pub(crate) fn get_image_cache_dir() -> &'static String {
    IMAGE_CACHE_DIR.get().unwrap()
}

pub(crate) fn get_database_dir() -> &'static String {
    DATABASE_DIR.get().unwrap()
}

pub(crate) fn get_download_dir() -> &'static String {
    DOWNLOAD_DIR.get().unwrap()
}

async fn reset_api() {
    let api = property::load_property("api".to_owned()).await.unwrap();
    if api.is_empty() {
        return;
    }
    let replace_from_string = OLD_API_URL
        .iter()
        .map(|s| String::from_utf8(base64::prelude::BASE64_STANDARD.decode(s).unwrap()).unwrap())
        .collect::<Vec<String>>();

    let replace_from = replace_from_string
        .iter()
        .map(|e| e.as_str())
        .collect::<Vec<&str>>();

    if replace_from.contains(&api.as_str()) {
        let replace_to =
            String::from_utf8(base64::prelude::BASE64_STANDARD.decode(API_URL).unwrap()).unwrap();
        property::save_property("old_api".to_owned(), api)
            .await
            .unwrap();
        property::save_property("api".to_owned(), replace_to)
            .await
            .unwrap();
    }
}

async fn load_api() {
    let api = property::load_property("api".to_owned()).await.unwrap();
    if api.is_empty() {
        return;
    }
    CLIENT.set_api_host(api).await;
}

async fn init_device() {
    let mut device = property::load_property("device".to_owned()).await.unwrap();
    if device.is_empty() {
        device = copy_client::random_device();
        property::save_property("device".to_owned(), device.clone())
            .await
            .unwrap();
    }
    let mut device_info = property::load_property("device_info".to_owned())
        .await
        .unwrap();
    if device_info.is_empty() {
        device_info = copy_client::random_device();
        property::save_property("device_info".to_owned(), device_info.clone())
            .await
            .unwrap();
    }
    CLIENT.set_device(device, device_info).await;
}

pub(crate) async fn init_header() {
    let headers = database::properties::header::Entity::get_all()
        .await
        .unwrap();
    let mut headers_vec = Vec::with_capacity(headers.len());
    for header in headers {
        headers_vec.push(copy_client::CopyHeader {
            key: header.k,
            value: header.v,
        });
    }
    CLIENT.set_headers(headers_vec).await;
}

pub async fn sync_header() {
    let sync_time = property::load_property("sync_time".to_owned())
        .await
        .unwrap();
    let sync_time = sync_time.parse::<u64>().unwrap_or(0);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let need_sync = now - sync_time > 60 * 10;
    if !need_sync {
        return;
    }
    let fetch_url_vec = vec![
        "https://ghfast.top/https://raw.githubusercontent.com/caolib/doki/main/docs/config/headers.json",
        "https://raw.githubusercontent.com/caolib/doki/main/docs/config/headers.json",
    ];
    for url in fetch_url_vec {
        if let Ok(_) = sync_raw(url).await {
            save_sync_time(now).await;
            init_header().await;
            return;
        }
    }
}

async fn save_sync_time(now: u64) {
    property::save_property("sync_time".to_owned(), now.to_string())
        .await
        .unwrap();
}

pub(crate) async fn sync_api_host() -> anyhow::Result<String> {
    let mut new_api_host = Option::<String>::None;
    let fetch_url_vec = vec![
        "https://ghfast.top/https://raw.githubusercontent.com/caolib/doki/main/docs/config/headers.json",
        "https://raw.githubusercontent.com/caolib/doki/main/docs/config/headers.json",
    ];
    for url in fetch_url_vec {
        if let Ok(api_host) = load_api_host(url).await {
            new_api_host = Some(api_host);
            break;
        }
    }
    if new_api_host.is_none() {
        return Err(anyhow::anyhow!("同步失败"));
    }
    let new_api_host = new_api_host.unwrap();
    if new_api_host.is_empty() {
        return Err(anyhow::anyhow!("同步失败"));
    }
    CLIENT.set_api_host(new_api_host.clone()).await;
    property::save_property("api".to_owned(), new_api_host.clone()).await?;
    Ok(new_api_host)
}

async fn sync_raw(url: &str) -> anyhow::Result<()> {
    let client = CLIENT.clone_agent().await;
    let rsp = client.get(url).send().await?;
    let data: HashMap<String, String> = rsp.json().await?;
    for (key, value) in data {
        if "device".eq(&key) || "device_info".eq(&key) || "deviceinfo".eq(&key) || "host".eq(&key) {
            continue;
        }
        database::properties::header::Entity::set_value(key, value).await?;
    }
    Ok(())
}

async fn load_api_host(url: &str) -> anyhow::Result<String> {
    let client = CLIENT.clone_agent().await;
    let rsp = client.get(url).send().await?;
    let data: HashMap<String, String> = rsp.json().await?;
    for (key, value) in data.clone() {
        if "device".eq(&key) || "device_info".eq(&key)  || "deviceinfo".eq(&key) || "host".eq(&key) {
            continue;
        }
        database::properties::header::Entity::set_value(key, value).await?;
    }
    if let Some(api_host) = data.get("host") {
        Ok(api_host.to_string())
    } else {
        Err(anyhow::anyhow!("同步失败"))
    }
}
