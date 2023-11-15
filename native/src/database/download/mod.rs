use crate::database::connect_db;
use crate::udto::UIQueryDownloadComic;
use once_cell::sync::OnceCell;
use sea_orm::{DatabaseConnection, DbErr, TransactionTrait};
use std::ops::Deref;
use tokio::sync::Mutex;

pub(crate) mod download_comic;
pub(crate) mod download_comic_chapter;
pub(crate) mod download_comic_group;
pub(crate) mod download_comic_page;

pub(crate) static DOWNLOAD_DATABASE: OnceCell<Mutex<DatabaseConnection>> = OnceCell::new();

pub(crate) async fn init() {
    let db = connect_db("download.db").await;
    DOWNLOAD_DATABASE.set(Mutex::new(db)).unwrap();
    // init tables
    download_comic::init().await;
    download_comic_group::init().await;
    download_comic_chapter::init().await;
    download_comic_page::init().await;
}

pub(crate) async fn save_chapter_images(
    comic_path_word: String,
    chapter_uuid: String,
    images: Vec<download_comic_page::Model>,
) -> anyhow::Result<()> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    db.transaction(|db| {
        Box::pin(async move {
            download_comic::add_image_count(db, comic_path_word.as_str(), images.len() as i64)
                .await?;
            for image in images {
                download_comic_page::save(db, image).await?;
            }
            download_comic_chapter::update_status(
                db,
                chapter_uuid.as_str(),
                download_comic_chapter::STATUS_FETCH_SUCCESS,
            )
            .await?;
            Ok::<(), DbErr>(())
        })
    })
    .await?;
    Ok(())
}

pub(crate) async fn chapter_fetch_error(chapter_uuid: String) -> anyhow::Result<()> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    download_comic_chapter::update_status(
        db.deref(),
        chapter_uuid.as_str(),
        download_comic_chapter::STATUS_FETCH_FAILED,
    )
    .await?;
    Ok(())
}

pub(crate) async fn download_page_success(
    comic_path_word: String,
    chapter_uuid: String,
    idx: i32,
    width: u32,
    height: u32,
    format: String,
) -> anyhow::Result<()> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    db.transaction(|db| {
        Box::pin(async move {
            download_comic_page::update_status(
                db,
                chapter_uuid.as_str(),
                idx,
                download_comic_page::STATUS_DOWNLOAD_SUCCESS,
                width,
                height,
                format,
            )
            .await?;
            download_comic::success_image_count(db, comic_path_word.as_str()).await?;
            Ok::<(), DbErr>(())
        })
    })
    .await?;
    Ok(())
}

pub async fn download_page_failed(chapter_uuid: String, idx: i32) -> anyhow::Result<()> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    download_comic_page::update_status(
        db.deref(),
        chapter_uuid.as_str(),
        idx,
        download_comic_page::STATUS_DOWNLOAD_FAILED,
        0,
        0,
        "".to_string(),
    )
    .await?;
    Ok(())
}

pub(crate) async fn remove_all(comic_path_word: String) -> anyhow::Result<()> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    db.transaction(|db| {
        Box::pin(async move {
            download_comic::delete_by_comic_path_word(db, comic_path_word.as_str()).await?;
            download_comic_group::delete_by_comic_path_word(db, comic_path_word.as_str()).await?;
            download_comic_chapter::delete_by_comic_path_word(db, comic_path_word.as_str()).await?;
            download_comic_page::delete_by_comic_path_word(db, comic_path_word.as_str()).await?;
            Ok::<(), DbErr>(())
        })
    })
    .await?;
    Ok(())
}

pub async fn append_download(data: UIQueryDownloadComic) -> anyhow::Result<()> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    db.transaction(|db| {
        Box::pin(async move {
            download_comic::insert_or_update_info(
                db,
                download_comic::Model {
                    cover_cache_key: crate::downloading::url_to_cache_key(data.cover.as_str()),
                    path_word: data.path_word,
                    alias: data.alias,
                    author: data.author,
                    b_404: data.b_404,
                    b_hidden: data.b_hidden,
                    ban: data.ban,
                    brief: data.brief,
                    close_comment: data.close_comment,
                    close_roast: data.close_roast,
                    cover: data.cover,
                    datetime_updated: data.datetime_updated,
                    females: data.females,
                    free_type: data.free_type,
                    img_type: data.img_type,
                    males: data.males,
                    name: data.name,
                    popular: data.popular,
                    reclass: data.reclass,
                    region: data.region,
                    restrict: data.restrict1,
                    seo_baidu: data.seo_baidu,
                    status: data.status,
                    theme: data.theme,
                    uuid: data.uuid,
                    append_time: chrono::Local::now().timestamp(),
                    cover_download_status: 0,
                    cover_format: "".to_string(),
                    cover_width: 0,
                    cover_height: 0,
                    image_count: 0,
                    image_count_success: 0,
                    download_status: 0,
                },
            )
            .await?;
            for g in data.groups {
                download_comic_group::insert_or_update_info(
                    db,
                    download_comic_group::Model {
                        comic_path_word: g.comic_path_word,
                        group_path_word: g.group_path_word,
                        count: g.count,
                        name: g.name,
                        group_rank: g.group_rank,
                    },
                )
                .await?;
            }
            for c in data.chapters {
                download_comic_chapter::insert_or_update_info(
                    db,
                    download_comic_chapter::Model {
                        comic_path_word: c.comic_path_word,
                        uuid: c.uuid,
                        comic_id: c.comic_id,
                        count: c.count,
                        datetime_created: c.datetime_created,
                        group_path_word: c.group_path_word,
                        img_type: c.img_type,
                        index: c.index,
                        is_long: c.is_long,
                        name: c.name,
                        news: c.news,
                        next: c.next,
                        ordered: c.ordered,
                        prev: None,
                        size: c.size,
                        type_field: c.type_field,
                        download_status: 0,
                    },
                )
                .await?;
            }
            Ok::<(), DbErr>(())
        })
    })
    .await?;
    Ok(())
}

pub async fn reset_fail_downloads() -> anyhow::Result<()> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    db.transaction(|db| {
        Box::pin(async move {
            download_comic::reset_failed(db).await?;
            download_comic_chapter::reset_failed(db).await?;
            download_comic_page::reset_failed(db).await?;
            Ok::<(), DbErr>(())
        })
    })
    .await?;
    Ok(())
}
