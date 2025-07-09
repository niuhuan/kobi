use std::ops::Deref;

use crate::database::connect_db;
use once_cell::sync::OnceCell;
use sea_orm::DatabaseConnection;
use sea_orm_migration::{MigrationTrait, MigratorTrait};
use tokio::sync::Mutex;

pub(crate) mod property;
pub(crate) mod header;

pub(crate) static PROPERTIES_DATABASE: OnceCell<Mutex<DatabaseConnection>> = OnceCell::new();

pub(crate) async fn init() {
    let db = connect_db("properties.db").await;
    PROPERTIES_DATABASE.set(Mutex::new(db)).unwrap();
    // init tables
    property::init().await;
    migrations().await.unwrap();
}

pub(crate) async fn migrations() -> anyhow::Result<()> {
    let lock = PROPERTIES_DATABASE.get().unwrap().lock().await;
    Migrator::up(lock.deref(), None).await?;
    Ok(())
}

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(
                header::migrations::m000001_create_table_header::Migration,
            ),
        ]
    }
}
