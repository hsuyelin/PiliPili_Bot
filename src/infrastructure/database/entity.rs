use sqlx::{Database, FromRow};
use sqlx::sqlite::{SqliteRow};
use crate::infrastructure::database::bindable::CloneableBindable;

pub trait Entity<DB>: for<'r> FromRow<'r, SqliteRow> + Send + Sync + Unpin
where
    DB: Database,
{
    fn get_table_name() -> &'static str;
    fn to_values(&self) -> Vec<(String, Box<dyn CloneableBindable + Send + Sync + 'static>)>;
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> where Self: Sized;
    fn id(&self) -> &str;
}