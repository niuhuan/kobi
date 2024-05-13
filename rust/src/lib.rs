pub mod api;
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
use crate::database::init_database;
use base64::Engine;
use copy_client::Client;
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
mod udto;
mod utils;
mod exports;

const API_URL: &str = "aHR0cHM6Ly9hcGkubWFuZ2Fjb3B5LmNvbQ==";
// const API_URL_ORIGIN: &str = "aHR0cHM6Ly9hcGkuY29weW1hbmdhLm5ldA==";

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
