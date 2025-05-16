use actix_web::{App, HttpServer};
use rat::api::routes::routes;
use rat::api::AppState;
use rat::config::Config;
use rat::db;
use rat::service::Service;
use std::net::TcpListener;
use std::sync::Arc;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), anyhow::Error> {
    std::env::set_var("RUST_LOG", "server=info");
    env_logger::init();

    let config = Config::load().expect("Failed to load config");

    let db_pool = db::connect(&config.database_url).await?;
    db::migrate(&db_pool).await?;

    let service = Service::new(db_pool);
    let app_state = Arc::new(AppState::new(service));

    let address = format!("{}:{}", config.port, config.port);
    log::info!("Listening on {}", address);

    let listener = TcpListener::bind(address)?;

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::new("server"))
            .configure(routes)
            .app_data(app_state)
    })
    .listen(listener)?
    .run()
    .await?;

    Ok(())
}
