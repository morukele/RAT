use actix_web::{App, HttpServer};
use rat::api::routes::routes;
use rat::config::Config;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load().expect("Failed to load config");
    let address = format!("{}:{}", config.port, config.port);

    let listener = TcpListener::bind(address)?;

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::new("server"))
            .configure(routes)
    })
    .listen(listener)?
    .run()
    .await
}
