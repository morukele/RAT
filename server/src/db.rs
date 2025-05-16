use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;

pub async fn connect(database_url: &str) -> Result<Pool<Postgres>, crate::error::Error> {
    PgPoolOptions::new()
        .max_connections(20)
        .max_lifetime(Duration::from_secs(30 * 60))
        .connect(database_url)
        .await
        .map_err(|err| {
            log::error!("Failed to connect to database: {}", err);
            err.into()
        })
}

pub async fn migrate(db: &Pool<Postgres>) -> Result<(), crate::error::Error> {
    match sqlx::migrate!("./db/migrations").run(db).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!("Failed to run migrations: {}", err);
            Err(err)
        }
    }
    .map_err(|_| crate::error::Error::MigrationError(String::from("Failed to run migrations")))?;

    Ok(())
}
