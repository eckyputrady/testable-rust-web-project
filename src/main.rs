mod auth;
mod infrastructure;

use actix_web::{HttpServer, App, web};
use std::sync::Arc;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{Registry, EnvFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_bunyan_formatter::{JsonStorageLayer, BunyanFormattingLayer};
use tracing_log::LogTracer;

#[actix_web::main]
async fn main() {
    LogTracer::init().expect("Unable to setup log tracer!");

    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name, non_blocking_writer);
    let subscriber = Registry::default()
        .with(EnvFilter::new("INFO"))
        .with(JsonStorageLayer)
        .with(bunyan_formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    if let Err(e) = dotenv::dotenv() {
        print!("Not applying .env : {:?}", e);
    }

    let pg_pool = Arc::new(infrastructure::postgresql::configure().await);
    let redis_client = Arc::new(infrastructure::redis::configure().await);

    let port = std::env::var("PORT").expect("PORT env var must be set");
    let address = format!("0.0.0.0:{}", port);
    println!("Binding server to {} ...", address);
    HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger)
                .configure(|cfg| configure_features(redis_client.clone(), pg_pool.clone(), cfg))
        })
        .bind(address)
        .expect("Unable to bind server")
        .run()
        .await
        .expect("Failed to start web server")
}

fn configure_features(redis_client: Arc<redis::Client>, pg_pool: Arc<PgPool>, cfg: &mut web::ServiceConfig) {
    configure_auth(redis_client.clone(), pg_pool.clone(), cfg);
}

fn configure_auth(redis_client: Arc<redis::Client>, pg_pool: Arc<PgPool>, cfg: &mut web::ServiceConfig) {
    use crate::auth::auth_service_impl::AuthServiceImpl;
    use crate::auth::postgres_credential_repo::PostgresCredentialRepoImpl;
    use crate::auth::redis_token_repo::RedisTokenRepoImpl;
    use crate::auth::rest_auth_controller;

    let service = AuthServiceImpl {
        credential_repo: PostgresCredentialRepoImpl {
            pg_pool: pg_pool.clone()
        },
        token_repo: RedisTokenRepoImpl {
            redis_client: redis_client.clone()
        }
    };
    rest_auth_controller::configure(web::Data::new(service), cfg);
}
