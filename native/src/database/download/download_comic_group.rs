use crate::database::download::{download_comic, DOWNLOAD_DATABASE};
use crate::database::{create_index, create_table_if_not_exists, index_exists};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::{Expr, IntoColumnRef, SimpleExpr};
use sea_orm::EntityTrait;
use sea_orm::{ConnectionTrait, DeleteResult, QuerySelect};
use serde_derive::{Deserialize, Serialize};
use std::convert::TryInto;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "download_comic_group")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub comic_path_word: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub group_path_word: String,
    pub count: i64,
    pub name: String,
    //
    pub group_rank: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init() {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    create_table_if_not_exists(db.deref(), Entity).await;
    if !index_exists(
        db.deref(),
        "download_comic_group",
        "download_comic_group_idx_comic_path_word",
    )
    .await
    {
        create_index(
            db.deref(),
            "download_comic_group",
            vec!["comic_path_word"],
            "download_comic_group_idx_comic_path_word",
        )
        .await;
    }
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
