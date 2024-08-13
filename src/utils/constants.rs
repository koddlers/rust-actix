use std::env;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_db();
    pub static ref SECRET: String = set_secret();
}

fn set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap()
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT").unwrap().parse::<u16>().unwrap()
}

fn set_db() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}

fn set_secret() -> String {
    dotenv::dotenv().ok();
    env::var("SECRET").unwrap()
}