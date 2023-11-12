use crate::database::download::{download_comic, DOWNLOAD_DATABASE};
use crate::database::{create_index, create_table_if_not_exists, index_exists};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::{Expr, IntoColumnRef, SimpleExpr};
use sea_orm::EntityTrait;
use sea_orm::{ConnectionTrait, DeleteResult, QuerySelect};
use serde_derive::{Deserialize, Serialize};
use std::convert::TryInto;
use std::ops::Deref;

pub(crate) const STATUS_INIT: i64 = 0;
pub(crate) const STATUS_DOWNLOAD_SUCCESS: i64 = 1;
pub(crate) const STATUS_DOWNLOAD_FAILED: i64 = 2;

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
