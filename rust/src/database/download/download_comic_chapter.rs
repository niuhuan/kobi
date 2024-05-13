use crate::database::download::DOWNLOAD_DATABASE;
use crate::database::{create_index, create_table_if_not_exists, index_exists};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::OnConflict;
use sea_orm::{DeleteResult, IntoActiveModel, Order, QueryOrder};
use sea_orm::{UpdateResult};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;

pub(crate) const STATUS_INIT: i64 = 0;
pub(crate) const STATUS_FETCH_SUCCESS: i64 = 1;
pub(crate) const STATUS_FETCH_FAILED: i64 = 2;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "download_comic_chapter")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub comic_path_word: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub uuid: String,
    pub comic_id: String,
    pub count: i64,
    pub datetime_created: String,
    pub group_path_word: String,
    pub img_type: i64,
    pub index: i64,
    pub is_long: bool,
    pub name: String,
    pub news: String,
    pub next: Option<String>,
    pub ordered: i64,
    pub prev: Option<String>,
    pub size: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    //
    pub download_status: i64,
    // pub contents: Vec<ChapterImage>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init() {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    create_table_if_not_exists(db.deref(), Entity).await;
    if !index_exists(
        db.deref(),
        "download_comic_chapter",
        "download_comic_chapter_idx_comic_path_word",
    )
    .await
    {
        create_index(
            db.deref(),
            "download_comic_chapter",
            vec!["comic_path_word"],
            "download_comic_chapter_idx_comic_path_word",
        )
        .await;
    }
}

pub(crate) async fn all_chapter(
    comic_path_word: &str,
    status: impl Into<Option<i64>>,
) -> anyhow::Result<Vec<Model>> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    let mut f = Entity::find().filter(Column::ComicPathWord.eq(comic_path_word));
    if let Some(status) = status.into() {
        f = f.filter(Column::DownloadStatus.eq(status));
    }
    let list = f.all(db.deref()).await?;
    Ok(list)
}

pub(crate) async fn update_status(
    db: &impl ConnectionTrait,
    uuid: &str,
    status: i64,
) -> Result<UpdateResult, DbErr> {
    Entity::update_many()
        .col_expr(Column::DownloadStatus, Expr::value(status))
        .filter(Column::Uuid.eq(uuid))
        .exec(db)
        .await
}

pub(crate) async fn is_all_chapter_fetched(comic_path_word: &str) -> anyhow::Result<bool> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    let count = Entity::find()
        .filter(Column::ComicPathWord.eq(comic_path_word))
        .filter(Column::DownloadStatus.ne(STATUS_FETCH_SUCCESS))
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

pub(crate) async fn insert_or_update_info(
    db: &impl ConnectionTrait,
    model: Model,
) -> Result<(), DbErr> {
    // 不需要更新downloadStatus
    let result = Entity::insert(model.into_active_model())
        .on_conflict(
            OnConflict::columns(vec![Column::ComicPathWord, Column::Uuid])
                .update_columns(vec![
                    Column::ComicId,
                    Column::Count,
                    Column::DatetimeCreated,
                    Column::GroupPathWord,
                    Column::ImgType,
                    Column::Index,
                    Column::IsLong,
                    Column::Name,
                    Column::News,
                    Column::Next,
                    Column::Ordered,
                    Column::Prev,
                    Column::Size,
                    Column::TypeField,
                ])
                .to_owned(),
        )
        .exec(db)
        .await;
    match result {
        Ok(_) => Ok(()),
        Err(DbErr::RecordNotInserted) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn in_download_chapter_uuid(comic_path_word: String) -> anyhow::Result<Vec<String>> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    let list = Entity::find()
        .filter(Column::ComicPathWord.eq(comic_path_word))
        .all(db.deref())
        .await?
        .into_iter()
        .map(|v| v.uuid)
        .collect::<Vec<_>>();
    Ok(list)
}

pub(crate) async fn reset_failed(db: &impl ConnectionTrait) -> Result<(), DbErr> {
    Entity::update_many()
        .col_expr(Column::DownloadStatus, Expr::value(STATUS_INIT))
        .filter(Column::DownloadStatus.eq(STATUS_FETCH_FAILED))
        .exec(db)
        .await?;
    Ok(())
}

// find_by_comic_path_word sort by ordered
pub(crate) async fn find_by_comic_path_word(comic_path_word: &str) -> anyhow::Result<Vec<Model>> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    let result = Entity::find()
        .filter(Column::ComicPathWord.eq(comic_path_word))
        .order_by(Column::Ordered, Order::Asc)
        .all(db.deref())
        .await?;
    Ok(result)
}
