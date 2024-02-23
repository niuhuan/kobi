use crate::database::create_table_if_not_exists;
use crate::database::download::DOWNLOAD_DATABASE;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::{OnConflict};
use sea_orm::{DeleteResult, IntoActiveModel, QuerySelect};
use sea_orm::{EntityTrait, UpdateResult};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;

pub(crate) const STATUS_INIT: i64 = 0;
pub(crate) const STATUS_DOWNLOAD_SUCCESS: i64 = 1;
pub(crate) const STATUS_DOWNLOAD_FAILED: i64 = 2;
pub(crate) const STATUS_DOWNLOAD_DELETING: i64 = 3;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "download_comic")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub path_word: String,
    pub alias: Option<String>,
    pub author: String,
    pub b_404: bool,
    pub b_hidden: bool,
    pub ban: i64,
    pub brief: String,
    pub close_comment: bool,
    pub close_roast: bool,
    pub cover: String,
    pub datetime_updated: String,
    pub females: String,
    pub free_type: String,
    pub img_type: i64,
    pub males: String,
    pub name: String,
    pub popular: i64,
    pub reclass: String,
    pub region: String,
    pub restrict: String,
    pub seo_baidu: String,
    pub status: String,
    pub theme: String,
    pub uuid: String,
    //
    pub append_time: i64,
    //
    pub cover_cache_key: String,
    pub cover_download_status: i64,
    pub cover_format: String,
    pub cover_width: u32,
    pub cover_height: u32,
    //
    pub image_count: i64,
    pub image_count_success: i64,
    //
    pub download_status: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init() {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    create_table_if_not_exists(db.deref(), Entity).await;
}

pub(crate) async fn next_comic(status: i64) -> anyhow::Result<Option<Model>> {
    Ok(Entity::find()
        .filter(Column::DownloadStatus.eq(status))
        .limit(1)
        .one(DOWNLOAD_DATABASE.get().unwrap().lock().await.deref())
        .await?)
}

pub(crate) async fn add_image_count(
    db: &impl ConnectionTrait,
    path_word: &str,
    count: i64,
) -> Result<UpdateResult, DbErr> {
    Entity::update_many()
        .filter(Column::PathWord.eq(path_word))
        .col_expr(
            Column::ImageCount,
            Expr::add(Expr::col(Column::ImageCount), count),
        )
        .exec(db)
        .await
}

pub(crate) async fn success_image_count(
    db: &impl ConnectionTrait,
    path_word: &str,
) -> Result<UpdateResult, DbErr> {
    Entity::update_many()
        .filter(Column::PathWord.eq(path_word))
        .col_expr(
            Column::ImageCountSuccess,
            Expr::add(Expr::col(Column::ImageCountSuccess), 1),
        )
        .exec(db)
        .await
}

pub(crate) async fn download_cover_success(
    path_word: &str,
    width: u32,
    height: u32,
    format: &str,
) -> Result<UpdateResult, DbErr> {
    Entity::update_many()
        .filter(Column::PathWord.eq(path_word))
        .col_expr(
            Column::CoverDownloadStatus,
            Expr::value(STATUS_DOWNLOAD_SUCCESS),
        )
        .col_expr(Column::CoverWidth, Expr::value(width))
        .col_expr(Column::CoverHeight, Expr::value(height))
        .col_expr(Column::CoverFormat, Expr::value(format))
        .exec(DOWNLOAD_DATABASE.get().unwrap().lock().await.deref())
        .await
}

pub(crate) async fn download_cover_failed(path_word: &str) -> Result<UpdateResult, DbErr> {
    Entity::update_many()
        .filter(Column::PathWord.eq(path_word))
        .col_expr(
            Column::CoverDownloadStatus,
            Expr::value(STATUS_DOWNLOAD_FAILED),
        )
        .exec(DOWNLOAD_DATABASE.get().unwrap().lock().await.deref())
        .await
}

pub(crate) async fn is_cover_download_success(path_word: &str) -> anyhow::Result<bool> {
    let model = Entity::find()
        .filter(Column::PathWord.eq(path_word))
        .one(DOWNLOAD_DATABASE.get().unwrap().lock().await.deref())
        .await?;
    Ok(model
        .expect("is_cover_download_success none")
        .cover_download_status
        == STATUS_DOWNLOAD_SUCCESS)
}

pub(crate) async fn update_status(path_word: &str, status: i64) -> Result<UpdateResult, DbErr> {
    Entity::update_many()
        .filter(Column::PathWord.eq(path_word))
        .col_expr(Column::DownloadStatus, Expr::value(status))
        .exec(DOWNLOAD_DATABASE.get().unwrap().lock().await.deref())
        .await
}

pub(crate) async fn next_deleting_comic() -> anyhow::Result<Option<Model>> {
    Ok(Entity::find()
        .filter(Column::DownloadStatus.eq(STATUS_DOWNLOAD_DELETING))
        .limit(1)
        .one(DOWNLOAD_DATABASE.get().unwrap().lock().await.deref())
        .await?)
}

pub(crate) async fn delete_by_comic_path_word(
    db: &impl ConnectionTrait,
    path_word: &str,
) -> Result<DeleteResult, DbErr> {
    Entity::delete_many()
        .filter(Column::PathWord.eq(path_word))
        .exec(db)
        .await
}

pub(crate) async fn insert_or_update_info(
    db: &impl ConnectionTrait,
    model: Model,
) -> Result<(), DbErr> {
    let result = Entity::insert(model.into_active_model())
        .on_conflict(
            OnConflict::column(Column::PathWord)
                .update_columns(vec![
                    Column::Alias,
                    Column::Author,
                    Column::B404,
                    Column::BHidden,
                    Column::Ban,
                    Column::Brief,
                    Column::CloseComment,
                    Column::CloseRoast,
                    Column::Cover,
                    Column::DatetimeUpdated,
                    Column::Females,
                    Column::FreeType,
                    Column::ImgType,
                    Column::Males,
                    Column::Name,
                    Column::Popular,
                    Column::Reclass,
                    Column::Region,
                    Column::Restrict,
                    Column::SeoBaidu,
                    Column::Status,
                    Column::Theme,
                    Column::Uuid,
                    Column::DownloadStatus,
                    Column::AppendTime,
                ])
                .to_owned(),
        )
        .exec(db)
        .await;
    // https://www.sea-ql.org/SeaORM/docs/basic-crud/insert/
    // Performing an upsert statement without inserting or updating any of the row will result in a DbErr::RecordNotInserted error.
    // If you want RecordNotInserted to be an Ok instead of an error, call .do_nothing():
    if let Err(DbErr::RecordNotInserted) = result {
        return Ok(());
    }
    result?;
    Ok(())
}

pub(crate) async fn reset_failed(db: &impl ConnectionTrait) -> Result<(), DbErr> {
    Entity::update_many()
        .col_expr(Column::DownloadStatus, Expr::value(STATUS_INIT))
        .filter(Column::DownloadStatus.eq(STATUS_DOWNLOAD_FAILED))
        .exec(db)
        .await?;
    Ok(())
}

pub(crate) async fn has_download_cover(cache_key: String) -> anyhow::Result<Option<Model>> {
    let model = Entity::find()
        .filter(Column::CoverCacheKey.eq(cache_key))
        .filter(Column::CoverDownloadStatus.eq(STATUS_DOWNLOAD_SUCCESS))
        .limit(1)
        .one(DOWNLOAD_DATABASE.get().unwrap().lock().await.deref())
        .await?;
    Ok(model)
}

pub(crate) async fn all() -> anyhow::Result<Vec<Model>> {
    let models = Entity::find()
        .all(DOWNLOAD_DATABASE.get().unwrap().lock().await.deref())
        .await?;
    Ok(models)
}

pub(crate) async fn find_by_uuid_list(uuid_list: &[String]) -> anyhow::Result<Vec<Model>> {
    let models = Entity::find()
        .filter(Column::Uuid.is_in(uuid_list))
        .all(DOWNLOAD_DATABASE.get().unwrap().lock().await.deref())
        .await?;
    Ok(models)
}
