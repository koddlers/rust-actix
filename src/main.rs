mod utils;
mod routes;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    set_logger();
    dotenv::dotenv().ok();
    env_logger::init();

    let port = (*utils::constants::PORT).clone();
    let address = (*utils::constants::ADDRESS).clone();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
    })
        .bind((address, port))?
        .run()
        .await
}

fn set_logger() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
}