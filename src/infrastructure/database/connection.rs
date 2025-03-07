use std::sync::Arc;
use sqlx::{Pool, Any, Error};

use super::adapter::{AnyDataBasePoolAdapter, DatabaseAdapter};
use super::config::DatabaseConfig;

#[derive(Clone)]
pub struct DatabaseConnectionManager {
    pool_adapter: Arc<dyn DatabaseAdapter>,
}

impl DatabaseConnectionManager {

    pub fn new(config: &DatabaseConfig) -> Self {
        let pool = Self::initialize_pool(config);
        let pool_adapter = Arc::new(AnyDataBasePoolAdapter::new(pool));
        Self { pool_adapter }
    }

    fn initialize_pool(config: &DatabaseConfig) -> Pool<Any> {
        let db_url = &config.get_db_url();
        Pool::connect_lazy(db_url).expect("Failed to create database pool")
    }

    pub fn get_adapter(&self) -> Arc<dyn DatabaseAdapter> {
        Arc::clone(&self.pool_adapter)
    }

    pub async fn execute_query(&self, query: &str) -> Result<u64, Error> {
        self.pool_adapter.execute(query).await
    }

    pub async fn fetch_all_query(&self, query: &str) -> Result<Vec<sqlx::any::AnyRow>, Error> {
        self.pool_adapter.fetch_all(query).await
    }
}