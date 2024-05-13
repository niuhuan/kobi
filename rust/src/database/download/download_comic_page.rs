use crate::database::download::DOWNLOAD_DATABASE;
use crate::database::{create_index, create_table_if_not_exists, index_exists};
use sea_orm::entity::prelude::*;
use sea_orm::{
    DeleteResult, InsertResult, IntoActiveModel, Order, QueryOrder, QuerySelect,
    UpdateResult,
};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;

pub(crate) const STATUS_INIT: i64 = 0;
pub(crate) const STATUS_DOWNLOAD_SUCCESS: i64 = 1;
pub(crate) const STATUS_DOWNLOAD_FAILED: i64 = 2;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "download_comic_page")]
pub struct Model {
    pub comic_path_word: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub chapter_uuid: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub image_index: i32,
    pub url: String,
    pub cache_key: String,
    pub download_status: i64,
    pub width: u32,
    pub height: u32,
    pub format: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init() {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    create_table_if_not_exists(db.deref(), Entity).await;
    if !index_exists(
        db.deref(),
        "download_comic_page",
        "download_comic_page_idx_comic_path_word",
    )
    .await
    {
        create_index(
            db.deref(),
            "download_comic_page",
            vec!["comic_path_word"],
            "download_comic_page_idx_comic_path_word",
        )
        .await;
    }
    if !index_exists(
        db.deref(),
        "download_comic_page",
        "download_comic_page_idx_chapter_uuid",
    )
    .await
    {
        create_index(
            db.deref(),
            "download_comic_page",
            vec!["chapter_uuid"],
            "download_comic_page_idx_chapter_uuid",
        )
        .await;
    }
    if !index_exists(
        db.deref(),
        "download_comic_page",
        "download_comic_page_idx_cache_key",
    )
    .await
    {
        create_index(
            db.deref(),
            "download_comic_page",
            vec!["cache_key"],
            "download_comic_page_idx_cache_key",
        )
        .await;
    }
    if !index_exists(
        db.deref(),
        "download_comic_page",
        "download_comic_page_idx_url",
    )
    .await
    {
        create_index(
            db.deref(),
            "download_comic_page",
            vec!["url"],
            "download_comic_page_idx_url",
        )
        .await;
    }
}

pub(crate) async fn save(
    db: &impl ConnectionTrait,
    model: Model,
) -> Result<InsertResult<ActiveModel>, DbErr> {
    Entity::insert(model.into_active_model()).exec(db).await
}

pub(crate) async fn has_download_pic(cache_key: String) -> anyhow::Result<Option<Model>> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    Ok(Entity::find()
        .filter(Expr::col(Column::CacheKey).eq(cache_key))
        .limit(1)
        .one(db.deref())
        .await?)
}

pub(crate) async fn fetch(
    comic_path_word: &str,
    status: i64,
    limit: u64,
) -> anyhow::Result<Vec<Model>> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    Ok(Entity::find()
        .filter(Expr::col(Column::ComicPathWord).eq(comic_path_word))
        .filter(Column::DownloadStatus.eq(status))
        .limit(limit)
        .all(db.deref())
        .await?)
}

pub(crate) async fn update_status(
    db: &impl ConnectionTrait,
    chapter_uuid: &str,
    image_index: i32,
    status: i64,
    width: u32,
    height: u32,
    format: String,
) -> Result<UpdateResult, DbErr> {
    Entity::update_many()
        .col_expr(Column::DownloadStatus, Expr::value(status))
        .col_expr(Column::Width, Expr::value(width))
        .col_expr(Column::Height, Expr::value(height))
        .col_expr(Column::Format, Expr::value(format))
        .filter(Column::ChapterUuid.eq(chapter_uuid))
        .filter(Column::ImageIndex.eq(image_index))
        .exec(db)
        .await
}

pub(crate) async fn is_all_page_downloaded(comic_path_word: &str) -> anyhow::Result<bool> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    let count = Entity::find()
        .filter(Column::ComicPathWord.eq(comic_path_word))
        .filter(Column::DownloadStatus.ne(STATUS_DOWNLOAD_SUCCESS))
        .count(db.deref())
        .await?;
    Ok(count == 0)
}

pub(crate) async fn delete_by_comic_path_word(
    db: &impl ConnectionTrait,
    comic_path_word: &str,
) -> Result<DeleteResult, DbErr> {
    Entity::delete_many()
        .filter(Column::ComicPathWord.eq(comic_path_word))
        .exec(db)
        .await
}

pub(crate) async fn reset_failed(db: &impl ConnectionTrait) -> Result<(), DbErr> {
    Entity::update_many()
        .col_expr(Column::DownloadStatus, Expr::value(STATUS_INIT))
        .filter(Column::DownloadStatus.eq(STATUS_DOWNLOAD_FAILED))
        .exec(db)
        .await?;
    Ok(())
}

// find_by_comic_path_word_and_chapter_uuid sort by image_index
pub(crate) async fn find_by_comic_path_word_and_chapter_uuid(
    comic_path_word: &str,
    chapter_uuid: &str,
) -> anyhow::Result<Vec<Model>> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    Ok(Entity::find()
        .filter(Column::ComicPathWord.eq(comic_path_word))
        .filter(Column::ChapterUuid.eq(chapter_uuid))
        .order_by(Column::ImageIndex, Order::Asc)
        .all(db.deref())
        .await?)
}
