use async_trait::async_trait;
use sqlx::Error;

#[async_trait]
pub trait CRUD<T> {
    async fn create(&self, entity: &T) -> Result<(), Error>;
    async fn fetch(&self, id: &str) -> Result<Option<T>, Error>;
    async fn update(&self, entity: &T) -> Result<(), Error>;
    async fn delete(&self, id: &str, logical: bool) -> Result<(), Error>;
}
