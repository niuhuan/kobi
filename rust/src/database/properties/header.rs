use crate::database::properties::PROPERTIES_DATABASE;
use crate::database::{create_index, create_table_if_not_exists, index_exists};
use sea_orm::entity::prelude::*;
use sea_orm::IntoActiveModel;
use sea_orm::Set;
use std::ops::Deref;
use sea_orm::ConnectionTrait;
use anyhow::Result;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "property")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub k: String,
    pub v: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


pub(super) mod migrations {

    pub(crate) mod m000001_create_table_header {
        use sea_orm::{ConnectionTrait, Schema};
        use sea_orm_migration::{MigrationName, MigrationTrait, SchemaManager};
        use sea_orm::sea_query::Index;
        use sea_orm::sea_query::Table;
        pub struct Migration;

        impl MigrationName for Migration {
            fn name(&self) -> &str {
                "m000001_create_table_header"
            }
        }

        #[async_trait::async_trait]
        impl MigrationTrait for Migration {
            async fn up(
                &self,
                manager: &SchemaManager,
            ) -> std::result::Result<(), sea_orm_migration::DbErr> {
                let db = manager.get_connection();
                let backend = db.get_database_backend();
                let schema = Schema::new(backend);
                manager
                    .create_table(
                        schema
                            .create_table_from_entity(super::super::Entity)
                            .if_not_exists()
                            .to_owned(),
                    )
                    .await?;
                manager
                .create_index(
                    Index::create()
                        .name("idx_header_k")
                        .table(super::super::Entity)
                        .if_not_exists()
                        .col(super::super::Column::K)
                        .to_owned(),
                )
                .await?;
                Ok(())
            }

            async fn down(
                &self,
                manager: &SchemaManager,
            ) -> std::result::Result<(), sea_orm_migration::DbErr> {
                manager.drop_table(
                    Table::drop()
                        .table(super::super::Entity)
                        .to_owned(),
                ).await?;
                Ok(())
            }
        }
    }
}

impl Entity {

    /// 获取所有属性
    pub async fn get_all() -> Result<Vec<Model>> {
        let db = super::get_connect().await;
        let lock = db.lock().await;
        let records = Entity::find()
            .all(&*lock)
            .await?;
        Ok(records)
    }

    /// 获取属性值
    pub async fn get_value(key: &str) -> Result<Option<String>> {
        let db = super::get_connect().await;
        let lock = db.lock().await;
        let record = Entity::find_by_id(key)
            .one(&*lock)
            .await?;
        Ok(record.map(|m| m.v))
    }

    /// 设置属性值
    pub async fn set_value(key: String, value: String) -> Result<()> {
        let db = super::get_connect().await;
        let lock = db.lock().await;
        let model = ActiveModel {
            k: Set(key),
            v: Set(value),
        };
        Entity::insert(model)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(Column::K)
                    .update_column(Column::V)
                    .to_owned()
            )
            .exec(lock.deref())
            .await?;
        Ok(())
    }

    pub async fn delete_by_keys(keys: Vec<String>) -> Result<()> {
        let db = super::get_connect().await;
        let lock = db.lock().await;
        Entity::delete_many()
            .filter(Column::K.is_in(keys))
            .exec(lock.deref())
            .await?;
        Ok(())
    }

    pub async fn set_values(values: Vec<Model>) -> Result<()> {
        for value in values {
            Entity::set_value(value.k, value.v).await?;
        }
        Ok(())
    }
} 
