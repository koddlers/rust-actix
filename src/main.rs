mod utils;
mod routes;

use std::error::{Error, Request};
use std::fmt::{Display, Formatter};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;

#[derive(Debug)]
struct MainError {
    message: String,
}

impl Display for MainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl Error for MainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!()
    }

    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&dyn Error> {
        todo!()
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> Result<(), MainError> {
    set_logger();
    dotenv::dotenv().ok();
    env_logger::init();

    let port = (*utils::constants::PORT).clone();
    let address = (*utils::constants::ADDRESS).clone();
    let db_url = (*utils::constants::DATABASE_URL).clone();

    println!("Running on Port: {}", port);

    let db: DatabaseConnection = Database::connect(&db_url)
        .await.
        map_err(|err| MainError { message: err.to_string() })?;

    Migrator::up(&db, None)
        .await
        .map_err(|err| MainError { message: err.to_string() })?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
            .configure(routes::auth_route::config)
            .configure(routes::user_routes::config)
    })
        .bind((address, port))
        .map_err(|err| MainError { message: err.to_string() })?
        .run()
        .await
        .map_err(|err| MainError { message: err.to_string() })
}

fn set_logger() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
}