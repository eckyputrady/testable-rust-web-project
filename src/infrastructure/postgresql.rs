use sqlx_pg_migrate::migrate;
use include_dir::{include_dir, Dir};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

static MIGRATIONS: Dir = include_dir!("migrations");

pub async fn configure() -> PgPool {
    let db_url = env::var("DB_URL").expect("DB_URL env var needs to be set");
    configure_with_db_url(&db_url).await
}

pub async fn configure_with_db_url(db_url: &str) -> PgPool {
    migrate(&db_url, &MIGRATIONS).await.expect("Unable to migrate DB");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgresql")
}