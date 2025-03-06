use std::marker::PhantomData;
use async_trait::async_trait;
use sqlx::{Pool, Sqlite, Error};
use sqlx::query::Query;

use crate::infrastructure::database::crud::CRUD;
use crate::infrastructure::database::entity::Entity;
use crate::infrastructure::database::bindable::CloneableBindable;

pub struct Repository<T> where T: Entity<Sqlite> {
    pool: Pool<Sqlite>,
    _phantom: PhantomData<T>,
}

impl<T> Repository<T> where T: Entity<Sqlite> {

    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            pool,
            _phantom: PhantomData,
        }
    }

    fn bind_values<'q>(
        &self,
        mut sql_query: Query<'q, Sqlite, sqlx::sqlite::SqliteArguments<'q>>,
        values: Vec<(String, Box<dyn CloneableBindable + Send + Sync + 'static>)>,
    ) -> Result<Query<'q, Sqlite, sqlx::sqlite::SqliteArguments<'q>>, Error> {
        for (_, value) in values.into_iter() {
            sql_query = value.bind_value(sql_query);
        }
        Ok(sql_query)
    }
}

#[async_trait]
impl<T> CRUD<T> for Repository<T> where T: Entity<Sqlite> {

    async fn create(&self, entity: &T) -> Result<(), Error> {
        let table_name = T::get_table_name();
        let values = entity.to_values();

        let columns: Vec<String> = values.iter().map(|(col, _)| col.clone()).collect();
        let placeholders: Vec<String> = (0..values.len()).map(|i| format!("?{}", i + 1)).collect();

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            columns.join(", "),
            placeholders.join(", ")
        );

        let sql_query = sqlx::query(query.as_str());
        let bound_query = self.bind_values(sql_query, values)?;
        bound_query.execute(&self.pool).await?;
        Ok(())
    }

    async fn fetch(&self, id: &str) -> Result<Option<T>, Error> {
        let table_name = T::get_table_name();
        let query = format!("SELECT * FROM {} WHERE id = ?", table_name);

        sqlx::query_as::<_, T>(query.as_str())
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn update(&self, entity: &T) -> Result<(), Error> {
        let table_name = T::get_table_name();
        let values = entity.to_values();

        let set_clause: Vec<String> = values
            .iter()
            .enumerate()
            .map(|(i, (col, _))| format!("{} = ?{}", col, i + 1))
            .collect();

        let query = format!(
            "UPDATE {} SET {} WHERE id = ?{}",
            table_name,
            set_clause.join(", "),
            values.len() + 1
        );

        let sql_query = sqlx::query(query.as_str());
        let mut bound_query = self.bind_values(sql_query, values)?;
        bound_query = bound_query.bind(entity.id());
        bound_query.execute(&self.pool).await?;
        Ok(())
    }

    async fn delete(&self, id: &str, logical: bool) -> Result<(), Error> {
        let table_name = T::get_table_name();
        let query = if logical {
            format!("UPDATE {} SET deleted = 1 WHERE id = ?", table_name)
        } else {
            format!("DELETE FROM {} WHERE id = ?", table_name)
        };

        sqlx::query(query.as_str())
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}