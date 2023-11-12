use crate::database::connect_db;
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
