use lazy_static::lazy_static;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::path::{Path, PathBuf};
use tokio::sync::{Mutex, MutexGuard};

#[allow(dead_code)]
pub(crate) fn join_paths<P: AsRef<Path>>(paths: Vec<P>) -> String {
    match paths.len() {
        0 => String::default(),
        _ => {
            let mut path: PathBuf = PathBuf::new();
            for x in paths {
                path = path.join(x);
            }
            return path.to_str().unwrap().to_string();
        }
    }
}

pub(crate) fn create_dir_if_not_exists(path: &str) {
    if !Path::new(path).exists() {
        std::fs::create_dir_all(path).unwrap();
    }
}

lazy_static! {
    static ref HASH_LOCK: Vec<Mutex::<()>> = {
        let mut mutex_vec: Vec<Mutex<()>> = vec![];
        for _ in 0..64 {
            mutex_vec.push(Mutex::<()>::new(()));
        }
        mutex_vec
    };
}

pub(crate) async fn hash_lock(url: &String) -> MutexGuard<'static, ()> {
    let mut s = DefaultHasher::new();
    s.write(url.as_bytes());
    HASH_LOCK[s.finish() as usize % HASH_LOCK.len()]
        .lock()
        .await
}

pub(crate) fn allowed_file_name(title: &str) -> String {
    title
        .replace("#", "_")
        .replace("'", "_")
        .replace("/", "_")
        .replace("\\", "_")
        .replace(":", "_")
        .replace("*", "_")
        .replace("?", "_")
        .replace("\"", "_")
        .replace(">", "_")
        .replace("<", "_")
        .replace("|", "_")
        .replace("&", "_")
}

