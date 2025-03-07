pub struct SqliteConfig {
    pub db_path: String,
}

pub struct PostgresConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String,
}

pub enum DatabaseConfig {
    Sqlite(SqliteConfig),
    Postgres(PostgresConfig),
}

impl DatabaseConfig {

    pub fn get_db_url(&self) -> String {
        match self {
            DatabaseConfig::Sqlite(config) => format!("sqlite://{}", config.db_path),
            DatabaseConfig::Postgres(config) => format!(
                "postgresql://{}:{}@{}:{}/{}",
                config.user, config.password, config.host, config.port, config.dbname
            ),
        }
    }
}