use sqlx::{Pool, Any};
use std::fs;
use std::path::Path;
use crate::infrastructure::database::schema::get_table_schema;

pub struct MigrationManager;

impl MigrationManager {
    
    pub async fn ensure_database(
        pool: &Pool<Any>, 
        tables: Vec<&str>
    ) -> Result<(), sqlx::Error> {
        for table in tables {
            if get_table_schema(pool, table).await.is_none() {
                let create_table_sql = format!(
                    "CREATE TABLE IF NOT EXISTS {} (id TEXT PRIMARY KEY, name TEXT)",
                    table
                );
                sqlx::query(&create_table_sql).execute(pool).await?;
            }
        }
        Ok(())
    }

    pub async fn run_migrations(
        pool: &Pool<Any>,
        migration_dir: &str
    ) -> Result<(), sqlx::Error> {
        let migration_path = Path::new(migration_dir);
        
        if !migration_path.exists() {
            fs::create_dir_all(migration_path)?;
        }

        for entry in fs::read_dir(migration_path)? {
            let path = entry?.path();
            if path.extension() == Some(std::ffi::OsStr::new("sql")) {
                let sql = fs::read_to_string(path)?;
                sqlx::query(&sql).execute(pool).await?;
            }
        }
        Ok(())
    }
}
