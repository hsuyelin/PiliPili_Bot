#[cfg(test)]
mod tests {
    use sqlx::{SqlitePool, Sqlite, Row};
    use async_trait::async_trait;
    use sqlx::sqlite::SqliteRow;
    use sqlx::FromRow;
    
    use pilipili_bot::infrastructure::database::{CRUD, Entity, Repository, CloneableBindable};

    #[derive(Debug, FromRow, Clone)]
    struct User {
        id: String,
        name: String,
        email: String,
        deleted: bool,
    }

    impl User {
        fn new(id: &str, name: &str, email: &str) -> Self {
            Self {
                id: id.to_string(),
                name: name.to_string(),
                email: email.to_string(),
                deleted: false,
            }
        }
    }

    #[async_trait]
    impl Entity<Sqlite> for User {
        fn get_table_name() -> &'static str {
            "users"
        }

        fn to_values(&self) -> Vec<(String, Box<dyn CloneableBindable + Send + Sync + 'static>)> {
            vec![
                ("id".to_string(), Box::new(self.id.clone())),
                ("name".to_string(), Box::new(self.name.clone())),
                ("email".to_string(), Box::new(self.email.clone())),
                ("deleted".to_string(), Box::new(self.deleted.clone())),
            ]
        }

        fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
            Ok(Self {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                email: row.try_get("email")?,
                deleted: row.try_get::<i32, _>("deleted")? != 0,
            })
        }

        fn id(&self) -> &str {
            &self.id
        }
    }

    #[tokio::test]
    async fn test_repository() -> Result<(), sqlx::Error> {
        let pool = SqlitePool::connect(":memory:").await?;

        sqlx::query(
            "CREATE TABLE users (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT NOT NULL,
                deleted INTEGER NOT NULL DEFAULT 0
            );",
        )
            .execute(&pool)
            .await?;

        // Initialize repository
        let repo = Repository::<User>::new(pool.clone());

        // Create a new user
        let user = User::new("1", "Alice", "alice@example.com");
        repo.create(&user).await?;

        // Fetch the user by ID
        let fetched_user = repo.fetch("1").await?;
        assert!(fetched_user.is_some());
        let fetched_user = fetched_user.unwrap();
        assert_eq!(fetched_user.name, "Alice");
        assert_eq!(fetched_user.email, "alice@example.com");

        // Update user
        let updated_user = User {
            name: "Alice Updated".to_string(),
            ..user.clone()
        };
        repo.update(&updated_user).await?;
        let fetched_user = repo.fetch("1").await?;
        assert_eq!(fetched_user.unwrap().name, "Alice Updated");

        // Delete user
        repo.delete("1", true).await?;
        let fetched_user = repo.fetch("1").await?;
        assert!(fetched_user.is_some());
        assert!(fetched_user.unwrap().deleted);

        repo.delete("1", false).await?;
        let fetched_user = repo.fetch("1").await?;
        assert!(fetched_user.is_none());

        Ok(())
    }
}