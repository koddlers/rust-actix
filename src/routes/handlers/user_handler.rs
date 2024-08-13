use crate::utils::api_response::ApiResponse;
use crate::utils::app_state::AppState;
use actix_web::{get, web, Responder};

#[get("")]
pub async fn user(_app_state: web::Data<AppState>) -> impl Responder {
    ApiResponse::new(200, "Verifyed user".to_string())
}