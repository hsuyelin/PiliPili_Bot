use sqlx::{Pool, Sqlite};
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::fs;
use std::path::Path;

const DATABASE_DIR: &str = "config/database";
const DATABASE_FILE: &str = "config/database/data.db";

static DB_POOL: Lazy<Arc<Pool<Sqlite>>> = Lazy::new(|| {
    init_database();
    let db_url = format!("sqlite://{}", DATABASE_FILE);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(10)
        .connect_lazy(&db_url)
        .expect("Failed to create database connection pool");
    Arc::new(pool)
});

fn init_database() {
    let db_dir = Path::new(DATABASE_DIR);
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).expect("Failed to create database directory");
    }

    let db_path = Path::new(DATABASE_FILE);
    if !db_path.exists() {
        fs::File::create(db_path).expect("Failed to create database file");
    }
}

pub fn get_db_pool() -> Arc<Pool<Sqlite>> {
    Arc::clone(&DB_POOL)
}