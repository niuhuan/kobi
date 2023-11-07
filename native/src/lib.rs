use crate::database::init_database;
use copy_client::Client;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::runtime;
use tokio::sync::RwLock;
use utils::create_dir_if_not_exists;
use utils::join_paths;

mod api;
mod bridge_generated;
mod copy_client;
mod database;
mod udto;
mod utils;

lazy_static! {
    pub(crate) static ref RUNTIME: runtime::Runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_keep_alive(tokio::time::Duration::new(60, 0))
        .worker_threads(30)
        .max_blocking_threads(30)
        .build()
        .unwrap();
    pub(crate) static ref CLIENT: Arc<RwLock<Client>> = Arc::new(RwLock::new(Client::new(
        reqwest::Client::new(),
        "https://www.mangacopy.com",
    )));
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
    // RUNTIME.block_on(async {
    //     *DOWNLOAD_AND_EXPORT_TO.lock().await =
    //         property::load_property("download_and_export_to".to_owned())
    //             .await
    //             .unwrap()
    // });
    // RUNTIME.spawn(download_thread::start_download());
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
