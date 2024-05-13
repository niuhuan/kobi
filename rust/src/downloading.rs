use crate::database::download;
use crate::database::download::{download_comic, download_comic_chapter, download_comic_page};
use crate::udto::UIQueryDownloadComic;
use crate::utils::{create_dir_if_not_exists, join_paths};
use crate::{get_download_dir, CLIENT};
use anyhow::Context;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) fn get_image_path(model: &download_comic_page::Model) -> String {
    join_paths(vec![
        get_download_dir().as_str(),
        model.comic_path_word.as_str(),
        model.chapter_uuid.as_str(),
        model.image_index.to_string().as_str(),
    ])
}

pub(crate) fn get_cover_path(model: &download_comic::Model) -> String {
    join_paths(vec![
        get_download_dir().as_str(),
        model.path_word.as_str(),
        "cover",
    ])
}

lazy_static! {
    pub(crate) static ref RESTART_FLAG: Mutex<bool> = Mutex::new(false);
    pub(crate) static ref DOWNLOAD_AND_EXPORT_TO: Mutex<String> = Mutex::new("".to_owned());
    pub(crate) static ref DOWNLOAD_THREAD: Mutex<i32> = Mutex::new(3);
    pub(crate) static ref PAUSE_FLAG: Mutex<bool> = Mutex::new(false);
}

async fn need_restart() -> bool {
    *RESTART_FLAG.lock().await.deref()
}

async fn set_restart() {
    let mut restart_flag = RESTART_FLAG.lock().await;
    if *restart_flag.deref() {
        *restart_flag = false;
    }
    drop(restart_flag);
}

async fn download_pause() -> bool {
    let pause_flag = PAUSE_FLAG.lock().await;
    let pausing = *pause_flag.deref();
    drop(pause_flag);
    if pausing {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
    pausing
}

pub(crate) async fn download_is_pause() -> bool {
    let pause_flag = PAUSE_FLAG.lock().await;
    return *pause_flag;
}

pub(crate) async fn download_set_pause(pause: bool) {
    let mut pause_flag = PAUSE_FLAG.lock().await;
    *pause_flag = pause;
    drop(pause_flag);
    set_restart().await;
    crate::database::properties::property::save_property(
        "download_pause".to_owned(),
        pause.to_string(),
    )
    .await
    .expect("save download_pause");
}

pub async fn start_download() {
    loop {
        process_deleting().await;
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        // 检测是否暂停
        while download_pause().await {}
        // 检测重启flag, 已经重启, 赋值false
        set_restart().await;
        // 下载下一个漫画
        let _ = down_next_comic().await;
        if need_restart().await {
            continue;
        }
    }
}

async fn process_deleting() {
    while let Some(comic) = download_comic::next_deleting_comic()
        .await
        .expect("next_deleting_comic")
    {
        let comic_dir = join_paths(vec![get_download_dir().as_str(), comic.path_word.as_str()]);
        let _ = tokio::fs::remove_dir_all(comic_dir.as_str()).await;
        download::remove_all(comic.path_word)
            .await
            .expect("remove_all");
    }
}

async fn down_next_comic() -> anyhow::Result<()> {
    // 检测重启flag
    if need_restart().await {
        return Ok(());
    }
    //
    if let Some(comic) = download_comic::next_comic(download_comic::STATUS_INIT)
        .await
        .expect("next_comic")
    {
        let comic_dir = join_paths(vec![get_download_dir().as_str(), comic.path_word.as_str()]);
        create_dir_if_not_exists(comic_dir.as_str());
        if comic.cover_download_status == download_comic::STATUS_INIT {
            down_cover(&comic).await;
        }
        if need_restart().await {
            return Ok(());
        }
        let chapters = download_comic_chapter::all_chapter(
            comic.path_word.as_str(),
            download_comic_chapter::STATUS_INIT,
        )
        .await
        .expect("all_chapter");
        for chapter in &chapters {
            if need_restart().await {
                return Ok(());
            }
            let _ = fetch_chapter(&chapter).await;
        }
        download_images(comic.path_word.clone()).await;
        if need_restart().await {
            return Ok(());
        }
        // sum
        setup_download_status(comic.path_word).await;
    }
    Ok(())
}

async fn down_cover(comic: &download_comic::Model) {
    if let Ok(data) = CLIENT.download_image(comic.cover.as_str()).await {
        if let Ok(format) = image::guess_format(&data) {
            let format = if let Some(format) = format.extensions_str().first() {
                format.to_string()
            } else {
                "".to_string()
            };
            if let Ok(image_) = image::load_from_memory(&data) {
                let width = image_.width();
                let height = image_.height();
                let path = get_cover_path(comic);
                tokio::fs::write(path.as_str(), data)
                    .await
                    .expect("write image");
                download_comic::download_cover_success(
                    comic.path_word.as_str(),
                    width,
                    height,
                    format.as_str(),
                )
                .await
                .expect("download_cover_success");
                return;
            }
        }
    }
    download_comic::download_cover_failed(comic.path_word.as_str())
        .await
        .expect("download_cover_failed");
}

async fn fetch_chapter(chapter: &download_comic_chapter::Model) -> anyhow::Result<()> {
    match CLIENT
        .comic_chapter_data(chapter.comic_path_word.as_str(), chapter.uuid.as_str())
        .await
    {
        Ok(data) => {
            let mut urls = Vec::with_capacity(data.chapter.contents.len());
            for _ in 0..data.chapter.words.len() {
                urls.push("".to_owned());
            }
            for i in 0..data.chapter.words.len() {
                let idx = *data.chapter.words.get(i).with_context(|| "words")? as usize;
                let url = data.chapter.contents.get(i).with_context(|| "contents")?;
                urls[idx] = url.url.clone();
            }
            let mut idx = 0;
            let mut images = vec![];
            for _ in data.chapter.contents {
                images.push(download_comic_page::Model {
                    comic_path_word: chapter.comic_path_word.clone(),
                    chapter_uuid: chapter.uuid.clone(),
                    cache_key: url_to_cache_key(urls[idx].as_str()),
                    url: urls[idx].clone(),
                    image_index: {
                        let tmp = idx;
                        idx += 1;
                        tmp
                    } as i32,
                    ..Default::default()
                });
            }
            download::save_chapter_images(
                chapter.comic_path_word.clone(),
                chapter.uuid.clone(),
                images,
            )
            .await
            .expect("save_chapter_images")
        }
        Err(_) => download::chapter_fetch_error(chapter.uuid.clone())
            .await
            .expect("chapter_fetch_error"),
    };
    Ok(())
}

async fn download_images(comic_path_word: String) {
    let comic_dir = join_paths(vec![get_download_dir().as_str(), comic_path_word.as_str()]);
    loop {
        if need_restart().await {
            break;
        }
        // 拉取
        let pages = download_comic_page::fetch(
            comic_path_word.as_str(),
            download_comic_chapter::STATUS_INIT,
            100,
        )
        .await
        .expect("pages");
        if pages.is_empty() {
            break;
        }
        //
        let mut chapters = vec![];
        for page in &pages {
            if !chapters.contains(&page.chapter_uuid) {
                chapters.push(page.chapter_uuid.clone());
            }
        }
        for x in chapters {
            let chapter_dir = join_paths(vec![comic_dir.as_str(), x.as_str()]);
            create_dir_if_not_exists(&chapter_dir);
        }
        // 获得线程数
        let dtl = DOWNLOAD_THREAD.lock().await;
        let d = *dtl;
        drop(dtl);
        // 多线程下载
        let pages = Arc::new(Mutex::new(VecDeque::from(pages)));
        let results = futures_util::future::join_all(
            num_iter::range(0, d)
                .map(|_| download_line(pages.clone()))
                .collect_vec(),
        )
        .await;
        //
        for x in results {
            if let Err(e) = x {
                println!("download_line error: {:?}", e);
            }
        }
    }
}

async fn download_line(
    deque: Arc<Mutex<VecDeque<download_comic_page::Model>>>,
) -> anyhow::Result<()> {
    loop {
        if need_restart().await {
            break;
        }
        let mut model_stream = deque.lock().await;
        let model = model_stream.pop_back();
        drop(model_stream);
        if let Some(image) = model {
            let _ = download_image(image).await;
        } else {
            break;
        }
    }
    Ok(())
}

async fn download_image(image: download_comic_page::Model) {
    if let Ok(data) = CLIENT.download_image(image.url.as_str()).await {
        if let Ok(format) = image::guess_format(&data) {
            let format = if let Some(format) = format.extensions_str().first() {
                format.to_string()
            } else {
                "".to_string()
            };
            if let Ok(image_) = image::load_from_memory(&data) {
                let width = image_.width();
                let height = image_.height();
                let path = get_image_path(&image);
                tokio::fs::write(path.as_str(), data)
                    .await
                    .expect("write image");
                download::download_page_success(
                    image.comic_path_word,
                    image.chapter_uuid,
                    image.image_index,
                    width,
                    height,
                    format,
                )
                .await
                .expect("download_page_success");
                return;
            }
        }
    }
    download::download_page_failed(image.chapter_uuid.clone(), image.image_index)
        .await
        .expect("download_page_failed");
}

async fn setup_download_status(comic_path_word: String) {
    let comic_status = if download_comic::is_cover_download_success(comic_path_word.as_str())
        .await
        .expect("is_cover_download_success")
        && download_comic_chapter::is_all_chapter_fetched(comic_path_word.as_str())
            .await
            .expect("is_all_chapter_fetched")
        && download_comic_page::is_all_page_downloaded(comic_path_word.as_str())
            .await
            .expect("is_all_page_downloaded")
    {
        download_comic::STATUS_DOWNLOAD_SUCCESS
    } else {
        download_comic::STATUS_DOWNLOAD_FAILED
    };
    download_comic::update_status(comic_path_word.as_str(), comic_status)
        .await
        .expect("update_status");
}

pub(crate) fn url_to_cache_key(url_str: &str) -> String {
    let u = url::Url::parse(url_str);
    if let Ok(u) = u {
        u.path().to_string()
    } else {
        "".to_string()
    }
}

pub(crate) async fn delete_download_comic(comic_path_word: String) -> anyhow::Result<()> {
    download_comic::update_status(
        comic_path_word.as_str(),
        download_comic::STATUS_DOWNLOAD_DELETING,
    )
    .await
    .expect("update_status");
    set_restart().await;
    Ok(())
}

pub async fn append_download(data: UIQueryDownloadComic) -> anyhow::Result<()> {
    download::append_download(data.clone()).await?;
    Ok(())
}

pub async fn reset_fail_downloads() -> anyhow::Result<()> {
    download::reset_fail_downloads().await?;
    set_restart().await;
    Ok(())
}
