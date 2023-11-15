use crate::copy_client::{Author, ErrorInfo};
use crate::database::active::comic_view_log;
use crate::database::cache::{image_cache, web_cache};
use crate::database::download::{download_comic, download_comic_chapter, download_comic_page};
use crate::database::properties::property;
use crate::udto::{
    UICacheImage, UIChapterData, UIComicData, UIComicQuery, UIDownloadComic, UILoginState,
    UIPageComicChapter, UIPageComicInExplore, UIPageRankItem, UIPageUIComicInList, UIPageUIViewLog,
    UIQueryDownloadComic, UITags, UIViewLog,
};
use crate::utils::{hash_lock, join_paths};
use crate::{downloading, get_image_cache_dir, CLIENT, RUNTIME};
use anyhow::Result;
use image::EncodableLayout;
use reqwest::Proxy;
use std::future::Future;
use std::time::Duration;

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

pub fn init_login_state() -> Result<UILoginState> {
    block_on(async {
        let token = property::load_property("token".to_owned()).await?;
        if token.is_empty() {
            Ok(UILoginState {
                state: 0,
                message: "".to_string(),
                member: Default::default(),
            })
        } else {
            CLIENT.set_token(token).await;
            match CLIENT.member_info().await {
                Ok(member) => Ok(UILoginState {
                    state: 1,
                    message: "".to_string(),
                    member: Some(member),
                }),
                Err(err) => {
                    match err.info {
                        ErrorInfo::Network(e) => Ok(UILoginState {
                            state: 2,
                            message: e.to_string(),
                            member: Default::default(),
                        }),
                        ErrorInfo::Message(_) => {
                            // token 已经失效
                            // todo : 用来token过期重新登录
                            // property::load_property("username".to_owned()).await?;
                            // property::load_property("password".to_owned()).await?;
                            property::save_property("token".to_owned(), "".to_owned()).await?;
                            Ok(UILoginState {
                                state: 0,
                                message: "".to_string(),
                                member: None,
                            })
                        }
                        ErrorInfo::Convert(e) => Ok(UILoginState {
                            state: 2,
                            message: e.to_string(),
                            member: None,
                        }),
                        ErrorInfo::Other(e) => Ok(UILoginState {
                            state: 2,
                            message: e.to_string(),
                            member: None,
                        }),
                    }
                }
            }
        }
    })
}

pub fn rank(date_type: String, offset: u64, limit: u64) -> Result<UIPageRankItem> {
    let key = format!("COMIC_RANK${}${}${}", date_type, offset, limit);
    block_on(web_cache::cache_first_map(
        key,
        Duration::from_secs(60 * 60 * 2),
        Box::pin(async move { CLIENT.comic_rank(date_type.as_str(), offset, limit).await }),
    ))
}

pub fn recommends(offset: u64, limit: u64) -> Result<UIPageUIComicInList> {
    let key = format!("COMIC_RECOMMENDS${}${}", offset, limit);
    block_on(web_cache::cache_first_map(
        key,
        Duration::from_secs(60 * 60 * 2),
        Box::pin(async move { CLIENT.recommends(offset, limit).await }),
    ))
}

pub fn comic(path_word: String) -> Result<UIComicData> {
    let key = format!("COMIC${}", path_word);
    block_on(web_cache::cache_first_map(
        key,
        Duration::from_secs(60 * 60 * 2),
        Box::pin(async move { CLIENT.comic(path_word.as_str()).await }),
    ))
}

pub fn comic_chapters(
    comic_path_word: String,
    group_path_word: String,
    limit: u64,
    offset: u64,
) -> Result<UIPageComicChapter> {
    let key = format!("COMIC_CHAPTERS${comic_path_word}${group_path_word}${limit}${offset}");
    block_on(web_cache::cache_first_map(
        key,
        Duration::from_secs(60 * 60 * 2),
        Box::pin(async move {
            CLIENT
                .comic_chapter(
                    comic_path_word.as_str(),
                    group_path_word.as_str(),
                    limit,
                    offset,
                )
                .await
        }),
    ))
}

pub fn comic_query(path_word: String) -> Result<UIComicQuery> {
    let key = format!("COMIC_QUERY${path_word}");
    block_on(web_cache::cache_first_map(
        key,
        Duration::from_secs(60 * 60 * 2),
        Box::pin(async move { CLIENT.comic_query(path_word.as_str()).await }),
    ))
}

pub fn comic_chapter_data(comic_path_word: String, chapter_uuid: String) -> Result<UIChapterData> {
    let key = format!("COMIC_CHAPTER_DATA${comic_path_word}${chapter_uuid}");
    block_on(web_cache::cache_first_map(
        key,
        Duration::from_secs(60 * 60 * 2),
        Box::pin(async move {
            CLIENT
                .comic_chapter_data(comic_path_word.as_str(), chapter_uuid.as_str())
                .await
        }),
    ))
}

pub fn tags() -> Result<UITags> {
    let key = format!("COMIC_TAGS");
    block_on(web_cache::cache_first_map(
        key,
        Duration::from_secs(60 * 60 * 15),
        Box::pin(async move { CLIENT.tags().await }),
    ))
}

pub fn explorer(
    ordering: Option<String>,
    top: Option<String>,
    theme: Option<String>,
    offset: u64,
    limit: u64,
) -> Result<UIPageComicInExplore> {
    let key = format!(
        "COMIC_EXPLORER${:?}${:?}${:?}${}${}",
        ordering, top, theme, limit, offset
    );
    block_on(web_cache::cache_first_map(
        key,
        Duration::from_secs(60 * 60 * 2),
        Box::pin(async move {
            CLIENT
                .explore(
                    ordering.as_deref(),
                    top.as_deref(),
                    theme.as_deref(),
                    offset,
                    limit,
                )
                .await
        }),
    ))
}

pub fn comic_search(
    q_type: String,
    q: String,
    offset: u64,
    limit: u64,
) -> Result<UIPageUIComicInList> {
    let key = format!("COMIC_SEARCH${}${}${}${}", q_type, q, limit, offset);
    block_on(web_cache::cache_first_map(
        key,
        Duration::from_secs(60 * 60 * 2),
        Box::pin(async move {
            CLIENT
                .comic_search(q_type.as_str(), q.as_str(), offset, limit)
                .await
        }),
    ))
}

pub fn view_comic_info(
    comic_path_word: String,
    comic_name: String,
    comic_authors: Vec<Author>,
    comic_cover: String,
) -> Result<()> {
    block_on(comic_view_log::view_info(comic_view_log::Model {
        comic_path_word,
        comic_name,
        comic_authors: serde_json::to_string(&comic_authors)?,
        comic_cover,
        ..Default::default()
    }))
}

pub fn view_chapter_page(
    comic_path_word: String,
    chapter_uuid: String,
    chapter_name: String,
    chapter_ordered: i64,
    chapter_size: i64,
    chapter_count: i64,
    page_rank: i32,
) -> Result<()> {
    block_on(comic_view_log::view_page(comic_view_log::Model {
        comic_path_word,
        chapter_uuid,
        chapter_name,
        chapter_ordered,
        chapter_size,
        chapter_count,
        page_rank,
        ..Default::default()
    }))
}

pub fn find_comic_view_log(path_word: String) -> Result<Option<UIViewLog>> {
    block_on(async move {
        Ok(
            if let Some(model) = comic_view_log::view_log_by_comic_path_word(path_word).await? {
                Some(UIViewLog::from(model))
            } else {
                None
            },
        )
    })
}

pub fn list_comic_view_logs(offset: i64, limit: i64) -> Result<UIPageUIViewLog> {
    block_on(async move {
        let count = comic_view_log::count().await?;
        let list = comic_view_log::load_view_logs(offset as u64, limit as u64).await?;
        Ok(UIPageUIViewLog {
            total: count as i64,
            limit,
            offset,
            list: list.into_iter().map(UIViewLog::from).collect(),
        })
    })
}

pub fn cache_image(
    cache_key: String,
    url: String,
    useful: String,
    extends_field_first: Option<String>,
    extends_field_second: Option<String>,
    extends_field_third: Option<String>,
) -> Result<UICacheImage> {
    block_on(async {
        let _ = hash_lock(&url).await;
        if let Some(model) = image_cache::load_image_by_cache_key(cache_key.as_str()).await? {
            image_cache::update_cache_time(cache_key.as_str()).await?;
            Ok(UICacheImage::from(model))
        } else if let Some(model) = download_comic::has_download_cover(cache_key.clone()).await? {
            // check downloads images has the same key
            Ok(UICacheImage::from(model))
        } else if let Some(model) = download_comic_page::has_download_pic(cache_key.clone()).await?
        {
            // check downloads images has the same key
            Ok(UICacheImage::from(model))
        } else {
            let local_path = hex::encode(md5::compute(&url).as_slice());
            let abs_path = join_paths(vec![get_image_cache_dir().as_str(), &local_path]);
            let bytes = CLIENT.download_image(url.as_str()).await?;
            let format = image::guess_format(bytes.as_bytes())?;
            let format = if let Some(format) = format.extensions_str().first() {
                format.to_string()
            } else {
                "".to_string()
            };
            let image = image::load_from_memory(&bytes)?;
            let model = image_cache::Model {
                cache_key,
                url,
                useful,
                extends_field_first,
                extends_field_second,
                extends_field_third,
                local_path,
                cache_time: chrono::Local::now().timestamp_millis(),
                image_format: format,
                image_width: image.width(),
                image_height: image.height(),
            };
            let model = image_cache::insert(model.clone()).await?;
            tokio::fs::write(&abs_path, &bytes).await?;
            Ok(UICacheImage::from(model))
        }
    })
}

pub fn clean_cache(time: i64) -> Result<()> {
    block_on(async move {
        let time = chrono::Local::now().timestamp() - time;
        clean_web(time).await?;
        clean_image(time).await?;
        crate::database::cache::vacuum().await?;
        Ok(())
    })
}

async fn clean_web(time: i64) -> Result<()> {
    web_cache::clean_web_cache_by_time(time).await
}

async fn clean_image(time: i64) -> Result<()> {
    let dir = get_image_cache_dir();
    loop {
        let caches: Vec<image_cache::Model> = image_cache::take_100_cache(time).await?;
        if caches.is_empty() {
            break;
        }
        for cache in caches {
            let local = join_paths(vec![dir.as_str(), cache.local_path.as_str()]);
            image_cache::delete_by_cache_key(cache.cache_key).await?; // 不管有几条被作用
            let _ = std::fs::remove_file(local); // 不管成功与否
        }
    }
    Ok(())
}

pub fn delete_download_comic(comic_path_word: String) -> Result<()> {
    block_on(downloading::delete_download_comic(comic_path_word))
}

pub fn append_download(data: UIQueryDownloadComic) -> Result<()> {
    block_on(downloading::append_download(data))
}

pub fn in_download_chapter_uuid(comic_path_word: String) -> Result<Vec<String>> {
    block_on(download_comic_chapter::in_download_chapter_uuid(
        comic_path_word,
    ))
}

pub fn reset_fail_downloads() -> Result<()> {
    block_on(downloading::reset_fail_downloads())
}

pub fn download_comics() -> Result<Vec<UIDownloadComic>> {
    Ok(block_on(download_comic::all())?
        .into_iter()
        .map(UIDownloadComic::from)
        .collect())
}

pub fn download_is_pause() -> Result<bool> {
    Ok(block_on(downloading::download_is_pause()))
}

pub fn download_set_pause(pause: bool) -> Result<()> {
    Ok(block_on(downloading::download_set_pause(pause)))
}

pub fn desktop_root() -> Result<String> {
    #[cfg(target_os = "windows")]
    {
        use anyhow::Context;
        Ok(join_paths(vec![
            std::env::current_exe()?
                .parent()
                .with_context(|| "error")?
                .to_str()
                .with_context(|| "error")?,
            "data",
        ]))
    }
    #[cfg(target_os = "macos")]
    {
        use anyhow::Context;
        let home = std::env::var_os("HOME")
            .with_context(|| "error")?
            .to_str()
            .with_context(|| "error")?
            .to_string();
        Ok(join_paths(vec![
            home.as_str(),
            "Library",
            "Application Support",
            "opensource",
            "kobi",
        ]))
    }
    #[cfg(target_os = "linux")]
    {
        use anyhow::Context;
        let home = std::env::var_os("HOME")
            .with_context(|| "error")?
            .to_str()
            .with_context(|| "error")?
            .to_string();
        Ok(join_paths(vec![home.as_str(), ".opensource", "kobi"]))
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    panic!("未支持的平台")
}
