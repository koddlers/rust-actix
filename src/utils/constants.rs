use std::env;
use std::string::ToString;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_db();
    pub static ref SECRET: String = set_secret();
    pub static ref MAX_FILE_SIZE: u64 = set_max_file_size();
}

// Application defaults
const _HOST: &str = "localhost";
const _PORT: &str = "5050";
const _SECRET: &str = "961814da78737d7df5d994ace36ca7641c1933df3af7bef90bc0b77e9d4965f4";
const _MAX_FILE_SIZE: &str = "10485760";


fn set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap_or(_HOST.to_string())
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT")
        .unwrap_or(_PORT.to_string()).parse::<u16>()
        .expect("Cannot parse the port to serve the application on")
}

fn set_db() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL")
        .expect("ERROR: `DATABASE_URL` is not set")
}

fn set_secret() -> String {
    dotenv::dotenv().ok();
    env::var("SECRET").unwrap_or(_SECRET.to_string())
}

fn set_max_file_size() -> u64 {
    dotenv::dotenv().ok();
    env::var("MAX_FILE_SIZE")
        .unwrap_or(_MAX_FILE_SIZE.to_string()).parse::<u64>()
        .expect("Cannot parse the port to serve the application on")
}