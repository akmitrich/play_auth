use sqlx::{Connection, Executor};

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("configuration"))?;
    settings.try_into()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }

    pub async fn configure_test_database(&self) -> sqlx::PgPool {
        let mut connection = sqlx::PgConnection::connect(&self.connection_string_without_db())
            .await
            .expect("Failed to connect to Postgres.");
        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, self.database_name).as_str())
            .await
            .expect("Failed to create database.");
        let connection_pool = sqlx::PgPool::connect(&self.connection_string())
            .await
            .expect("Failed to connect a pool to Postgres.");
        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database.");
        connection_pool
    }
}
