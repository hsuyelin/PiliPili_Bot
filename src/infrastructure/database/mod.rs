pub mod connection;
pub mod transaction;
pub mod migration;
pub mod database_adapter;
pub mod repository;
pub mod schema;
pub mod crud;
pub mod macros;
pub mod entity;
pub mod bindable;

use sqlx::sqlite::SqlitePool;
use sqlx::Pool;
use std::sync::Arc;
use tokio::sync::Mutex;

pub use crud::CRUD;
pub use entity::Entity;
pub use repository::Repository;
pub use bindable::{Bindable, CloneableBindable};

pub struct Database {
    pool: Arc<Mutex<Pool<sqlx::Sqlite>>>,
}

impl Database {

    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self {
            pool: Arc::new(Mutex::new(pool)),
        })
    }

    pub fn get_pool(&self) -> Arc<Mutex<Pool<sqlx::Sqlite>>> {
        self.pool.clone()
    }
}