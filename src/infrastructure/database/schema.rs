use sqlx::{Pool, Row, Any};

pub async fn get_table_schema(
    pool: &Pool<Any>,
    table_name: &str
) -> Option<String> {
    let query = "SELECT sql FROM sqlite_master WHERE type='table' AND name = ?";
    if let Ok(row) = sqlx::query(query)
        .bind(table_name)
        .fetch_one(pool)
        .await
    {
        return row.try_get::<String, _>(0).ok();
    }
    None
}