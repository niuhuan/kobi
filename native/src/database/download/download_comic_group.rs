use crate::database::download::DOWNLOAD_DATABASE;
use crate::database::{create_index, create_table_if_not_exists, index_exists};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::OnConflict;
use sea_orm::{DeleteResult, Order, QueryOrder};
use sea_orm::{IntoActiveModel};
use serde_derive::{Deserialize, Serialize};
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

pub(crate) async fn insert_or_update_info(
    db: &impl ConnectionTrait,
    model: Model,
) -> Result<(), DbErr> {
    // https://www.sea-ql.org/SeaORM/docs/basic-crud/insert/
    // Performing an upsert statement without inserting or updating any of the row will result in a DbErr::RecordNotInserted error.
    // If you want RecordNotInserted to be an Ok instead of an error, call .do_nothing():
    Entity::insert(model.into_active_model())
        .on_conflict(
            OnConflict::columns(vec![Column::ComicPathWord, Column::GroupPathWord])
                .do_nothing()
                .to_owned(),
        )
        .exec(db)
        .await?;
    Ok(())
}

// find_by_comic_path_word order by rank
pub(crate) async fn find_by_comic_path_word(comic_path_word: &str) -> anyhow::Result<Vec<Model>> {
    let db = DOWNLOAD_DATABASE.get().unwrap().lock().await;
    let result = Entity::find()
        .filter(Column::ComicPathWord.eq(comic_path_word))
        .order_by(Column::GroupRank, Order::Asc)
        .all(db.deref())
        .await?;
    Ok(result)
}
