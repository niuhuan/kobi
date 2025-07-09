pub mod api;
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
use crate::api::api::save_property;
use crate::database::init_database;
use base64::Engine;
use copy_client::Client;
use database::properties::property;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use std::sync::Mutex;
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
    pub(crate) static ref CLIENT: Arc<Client> =
        Arc::new(Client::new(reqwest::Client::new(), api_url()));
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
    let mut device_info = property::load_property("device_info".to_owned()).await.unwrap();
    if device_info.is_empty() {
        device_info = copy_client::random_device();
        property::save_property("device_info".to_owned(), device_info.clone())
            .await
            .unwrap();
    }
    CLIENT.set_device(device, device_info).await;
}

pub(crate) async fn init_header() {
    let headers = database::properties::header::Entity::get_all().await.unwrap();
    let mut headers_vec = Vec::with_capacity(headers.len());
    for header in headers {
        headers_vec.push(copy_client::CopyHeader {
            key: header.k,
            value: header.v,
        });
    }
    // todo: set headers
    // CLIENT.set_headers(headers_vec).await;
}