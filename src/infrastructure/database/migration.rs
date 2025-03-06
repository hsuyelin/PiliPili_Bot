use sqlx::{Sqlite, Pool};
use std::fs;
use std::path::Path;

const MIGRATION_DIR: &str = "config/database/migrations";

pub struct MigrationManager;

impl MigrationManager {

    pub async fn run_migrations(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let migration_path = Path::new(MIGRATION_DIR);
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
