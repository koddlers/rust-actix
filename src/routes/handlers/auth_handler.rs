use crate::utils::api_response::ApiResponse;
use crate::utils::app_state::AppState;

use actix_web::web::{Data, Json};
use actix_web::post;

use crate::utils::jwt::encode_jwt;
use entity::user::ActiveModel;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Serialize, Deserialize)]
struct RegisterModel {
    name: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginModel {
    email: String,
    password: String,
}

#[post("/register")]
pub async fn register(app_state: Data<AppState>, data: Json<RegisterModel>) -> Result<ApiResponse, ApiResponse> {
    let user = ActiveModel {
        name: Set(data.name.clone()),
        email: Set(data.email.clone()),
        password: Set(digest(&data.password)),
        ..Default::default()
    }.insert(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    // TODO: return the newly created `user` without `password`
    Ok(ApiResponse::new(200, format!("{}", user.id)))
}

#[post("/login")]
pub async fn login(app_state: Data<AppState>, data: Json<LoginModel>) -> Result<ApiResponse, ApiResponse> {
    let user = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(&data.email))
                .add(entity::user::Column::Password.eq(digest(&data.password)))
        ).one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(401, String::from("User not found")))?;

    let token = encode_jwt(user.email, user.id)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, format!("{{ 'token': '{}' }}", token)))
}