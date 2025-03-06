use async_trait::async_trait;
use sqlx::{Sqlite, Pool};

#[async_trait]
pub trait DatabaseAdapter {
    async fn execute(&self, query: &str) -> Result<u64, sqlx::Error>;
    async fn fetch_all(&self, query: &str) -> Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error>;
}

pub struct SqliteAdapter {
    pool: Pool<Sqlite>,
}

impl SqliteAdapter {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DatabaseAdapter for SqliteAdapter {

    async fn execute(&self, query: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(query).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    async fn fetch_all(&self, query: &str) -> Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> {
        let rows = sqlx::query(query).fetch_all(&self.pool).await?;
        Ok(rows)
    }
}
