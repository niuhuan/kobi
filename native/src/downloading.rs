use crate::database::download;
use crate::database::download::{download_comic, download_comic_chapter, download_comic_page};
use crate::utils::join_paths;
use crate::{get_download_dir, get_image_cache_dir, CLIENT};
use lazy_static::lazy_static;
use std::ops::Deref;
use tokio::sync::Mutex;

pub(crate) fn get_image_path(model: &download_comic_page::Model) -> String {
    join_paths(vec![
        get_download_dir().as_str(),
        model.comic_path_word.as_str(),
        model.chapter_uuid.as_str(),
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

pub(crate) async fn start_download() {
    loop {
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
            if need_restart().await {
                return Ok(());
            }
        }
    }
    Ok(())
}

async fn fetch_chapter(chapter: &download_comic_chapter::Model) -> anyhow::Result<()> {
    match CLIENT
        .comic_chapter_data(chapter.comic_path_word.as_str(), chapter.uuid.as_str())
        .await
    {
        Ok(data) => {
            let mut idx = 0;
            let mut images = vec![];
            for x in data.chapter.contents {
                images.push(download_comic_page::Model {
                    comic_path_word: chapter.group_path_word.clone(),
                    chapter_uuid: chapter.uuid.clone(),
                    image_index: {
                        let tmp = idx;
                        idx += 1;
                        tmp
                    },
                    cache_key: url_to_cache_key(x.url.as_str()),
                    url: x.url,
                    ..Default::default()
                });
            }
            download::save_chapter_images(chapter.uuid.clone(), images)
                .await
                .expect("save_chapter_images")
        }
        Err(_) => download::chapter_fetch_error(chapter.uuid.clone())
            .await
            .expect("chapter_fetch_error"),
    };
    Ok(())
}

fn url_to_cache_key(url_str: &str) -> String {
    let u = url::Url::parse(url_str);
    if let Ok(u) = u {
        u.path().to_string()
    } else {
        "".to_string()
    }
}
