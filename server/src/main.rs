use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use server::api::routes::routes;
use server::api::AppState;
use server::config::Config;
use server::db;
use server::service::Service;
use std::env;
use std::net::TcpListener;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), anyhow::Error> {
    unsafe {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let config = Config::load().expect("Failed to load config");

    let db_pool = db::connect(&config.database_url).await?;
    db::migrate(&db_pool).await?;

    let service = Service::new(db_pool);
    let app_state = AppState::new(service);

    let address = format!("{}:{}", config.host, config.port);
    println!("Listening on {}", address);

    let listener = TcpListener::bind(address)?;

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(Cors::permissive())
            .configure(routes)
            .app_data(Data::new(app_state.clone()))
    })
    .listen(listener)?
    .run()
    .await?;

    Ok(())
}
