use sqlx::{Pool, Sqlite, Transaction};

pub struct TransactionManager<'a> {
    pub tx: Transaction<'a, Sqlite>,
}

impl<'a> TransactionManager<'a> {

    pub async fn begin(pool: &'a Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        let tx = pool.begin().await?;
        Ok(Self { tx })
    }

    pub async fn commit(self) -> Result<(), sqlx::Error> {
        self.tx.commit().await
    }

    pub async fn rollback(self) -> Result<(), sqlx::Error> {
        self.tx.rollback().await
    }
}