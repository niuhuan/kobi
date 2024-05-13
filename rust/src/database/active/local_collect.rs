use crate::database::active::ACTIVE_DATABASE;
use crate::database::create_table_if_not_exists;
use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;
use serde_derive::{Deserialize, Serialize};
use std::convert::TryInto;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "local_collect")]
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
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init() {
    let db = ACTIVE_DATABASE.get().unwrap().lock().await;
    create_table_if_not_exists(db.deref(), Entity).await;
}
