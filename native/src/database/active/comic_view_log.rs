use crate::database::active::ACTIVE_DATABASE;
use crate::database::{create_index, create_table_if_not_exists, index_exists};
use sea_orm::entity::prelude::*;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
use sea_orm::{EntityTrait, IntoActiveModel, Set};
use std::convert::TryInto;
use std::ops::Deref;

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "comic_view_log")]
pub struct Model {
    // comic info
    #[sea_orm(primary_key, auto_increment = false)]
    pub comic_path_word: String,
    pub comic_name: String,
    pub comic_authors: String,
    pub comic_cover: String,
    // chapter info
    pub chapter_uuid: String,
    pub chapter_name: String,
    pub chapter_ordered: i64,
    pub chapter_size: i64,
    pub chapter_count: i64,
    // read info
    pub page_rank: i32,
    pub view_time: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init() {
    let db = ACTIVE_DATABASE.get().unwrap().lock().await;
    create_table_if_not_exists(db.deref(), Entity).await;
    if !index_exists(db.deref(), "comic_view_log", "comic_view_log_idx_view_time").await {
        create_index(
            db.deref(),
            "comic_view_log",
            vec!["view_time"],
            "comic_view_log_idx_view_time",
        )
        .await;
    }
}

pub(crate) async fn view_info(mut model: Model) -> anyhow::Result<()> {
    let db = ACTIVE_DATABASE.get().unwrap().lock().await;
    if let Some(in_db) = Entity::find_by_id(model.comic_path_word.clone())
        .one(db.deref())
        .await?
    {
        let mut in_db = in_db.into_active_model();
        in_db.comic_path_word = Set(model.comic_path_word);
        in_db.comic_name = Set(model.comic_name);
        in_db.comic_authors = Set(model.comic_authors);
        in_db.comic_cover = Set(model.comic_cover);
        in_db.view_time = Set(chrono::Local::now().timestamp_millis());
        in_db.update(db.deref()).await?;
    } else {
        model.view_time = chrono::Local::now().timestamp_millis();
        model.into_active_model().insert(db.deref()).await?;
    }
    Ok(())
}

pub(crate) async fn view_page(model: Model) -> anyhow::Result<()> {
    let db = ACTIVE_DATABASE.get().unwrap().lock().await;
    if let Some(in_db) = Entity::find_by_id(model.comic_path_word.clone())
        .one(db.deref())
        .await?
    {
        let mut in_db = in_db.into_active_model();
        in_db.comic_path_word = Set(model.comic_path_word);
        in_db.chapter_uuid = Set(model.chapter_uuid);
        in_db.chapter_name = Set(model.chapter_name);
        in_db.chapter_ordered = Set(model.chapter_ordered);
        in_db.chapter_size = Set(model.chapter_size);
        in_db.chapter_count = Set(model.chapter_count);
        in_db.page_rank = Set(model.page_rank);
        in_db.view_time = Set(chrono::Local::now().timestamp_millis());
        in_db.update(db.deref()).await?;
    }
    Ok(())
}

pub(crate) async fn load_view_logs(offset: u64, limit: u64) -> anyhow::Result<Vec<Model>> {
    let db = ACTIVE_DATABASE.get().unwrap().lock().await;
    Ok(Entity::find()
        .order_by_desc(Column::ViewTime)
        .offset(offset)
        .limit(limit)
        .all(db.deref())
        .await?)
}

pub(crate) async fn count() -> anyhow::Result<u64> {
    let db = ACTIVE_DATABASE.get().unwrap().lock().await;
    let count = Entity::find().count(db.deref()).await?;
    Ok(count)
}

pub(crate) async fn view_log_by_comic_path_word(
    path_word: String,
) -> anyhow::Result<Option<Model>> {
    let db = ACTIVE_DATABASE.get().unwrap().lock().await;
    Ok(Entity::find_by_id(path_word).one(db.deref()).await?)
}
