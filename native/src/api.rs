use crate::database::cache::web_cache;
use crate::database::properties::property;
use crate::udto::{UIComicData, UIPageRankItem};
use crate::{CLIENT, RUNTIME};
use anyhow::Result;
use reqwest::Proxy;
use std::future::Future;
use std::time::Duration;
use tokio::io::AsyncReadExt;

pub fn init(root: String) {
    crate::init_root(&root);
    set_proxy(get_proxy().unwrap()).unwrap();
}

fn block_on<T>(f: impl Future<Output = T>) -> T {
    RUNTIME.block_on(f)
}

pub fn save_property(k: String, v: String) -> Result<()> {
    block_on(property::save_property(k, v))
}

pub fn load_property(k: String) -> Result<String> {
    block_on(property::load_property(k))
}

pub fn get_proxy() -> Result<String> {
    block_on(property::load_property("proxy".to_owned()))
}

pub fn set_proxy(proxy: String) -> Result<()> {
    block_on(async move {
        CLIENT
            .read()
            .await
            .set_agent(
                if proxy.is_empty() {
                    reqwest::Client::builder()
                } else {
                    reqwest::Client::builder().proxy(Proxy::all(proxy.as_str())?)
                }
                .build()?,
            )
            .await;
        property::save_property("proxy".to_owned(), proxy).await?;
        Ok(())
    })
}

pub fn rank(date_type: String, offset: u64, limit: u64) -> Result<UIPageRankItem> {
    let key = format!("COMIC_RANK${}${}${}", date_type, offset, limit);
    block_on(web_cache::cache_first_map(
        key,
        Duration::from_secs(60 * 60 * 2),
        Box::pin(async move {
            CLIENT
                .read()
                .await
                .comic_rank(date_type.as_str(), offset, limit)
                .await
        }),
    ))
}

pub fn comic(path_word: String) -> Result<UIComicData> {
    let key = format!("COMIC${}", path_word);
    block_on(web_cache::cache_first_map(
        key,
        Duration::from_secs(60 * 60 * 2),
        Box::pin(async move { CLIENT.read().await.comic(path_word.as_str()).await }),
    ))
}
