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
    chapter_uuid: String,
    images: Vec<download_comic_page::Model>,
) -> anyhow::Result<()> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    db.transaction(|db| {
        Box::pin(async move {
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
