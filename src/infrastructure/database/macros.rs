#[allow(unused)]
const DATABASE_MACROS_LOGGER_DOMAIN: &str = "[DATABASE-MACROS]";

#[macro_export]
macro_rules! execute_query {
    ($pool:expr, $query:expr) => {{
        use sqlx::query;

        let result = query($query)
            .fetch_all(&$pool)
            .await;

        match result {
            Ok(rows) => rows,
            Err(err) => {
                use crate::infrastructure::logger::logger::Logger;
                Logger::error(
                    DATABASE_MACROS_LOGGER_DOMAIN.to_string(),
                    &format!("Error executing query: {}", err)
                );
                Vec::new()
            }
        }
    }};
}

#[macro_export]
macro_rules! execute_query_as {
    ($pool:expr, $query:expr, $t:ty) => {{
        use sqlx::query_as;

        let result = query_as::<_, $t>($query)
            .fetch_all(&$pool)
            .await;

        match result {
            Ok(rows) => rows,
            Err(err) => {
                use crate::infrastructure::logger::logger::Logger;
                Logger::error(
                    DATABASE_MACROS_LOGGER_DOMAIN.to_string(),
                    &format!("Error executing query_as: {}", err)
                );
                Vec::new()
            }
        }
    }};
}

#[macro_export]
macro_rules! execute_insert {
    ($pool:expr, $query:expr) => {{
        use sqlx::query;

        let result = query($query)
            .execute(&$pool)
            .await;

        match result {
            Ok(_) => {
                use crate::infrastructure::logger::logger::Logger;
                Logger::info(
                    DATABASE_MACROS_LOGGER_DOMAIN.to_string(),
                    &format!("Successfully inserted query {}", &stringify!($query))
                );
                true
            }
            Err(err) => {
                use crate::infrastructure::logger::logger::Logger;
                Logger::error(
                    DATABASE_MACROS_LOGGER_DOMAIN.to_string(),
                    &format!("Error executing insert: {}", err)
                );
                false
            }
        }
    }};
}