mod utils;
mod routes;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    set_logger();
    dotenv::dotenv().ok();
    env_logger::init();

    let port = (*utils::constants::PORT).clone();
    let address = (*utils::constants::ADDRESS).clone();
    let db_url = (*utils::constants::DATABASE_URL).clone();

    let db: DatabaseConnection = Database::connect(&db_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
            .configure(routes::auth_route::config)
            .configure(routes::user_routes::config)
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