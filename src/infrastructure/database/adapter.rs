use async_trait::async_trait;
use sqlx::{Any, Pool, Error};

#[async_trait]
pub trait DatabaseAdapter {
    async fn execute(&self, query: &str) -> Result<u64, Error>;
    async fn fetch_all(&self, query: &str) -> Result<Vec<sqlx::any::AnyRow>, Error>;
}

pub struct AnyDataBasePoolAdapter {
    pool: Pool<Any>,
}

impl AnyDataBasePoolAdapter {
    pub fn new(pool: Pool<Any>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DatabaseAdapter for AnyDataBasePoolAdapter {

    async fn execute(&self, query: &str) -> Result<u64, Error> {
        let result = sqlx::query(query).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    async fn fetch_all(&self, query: &str) -> Result<Vec<sqlx::any::AnyRow>, Error> {
        let rows = sqlx::query(query).fetch_all(&self.pool).await?;
        Ok(rows)
    }
}
