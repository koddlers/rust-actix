use crate::utils::api_response::ApiResponse;
use crate::utils::app_state::AppState;

use actix_web::web::{Data, Json};
use actix_web::{post, Responder};

use entity::user::ActiveModel;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use sha256::digest;
use crate::utils::jwt::encode_jwt;

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
pub async fn register(app_state: Data<AppState>, data: Json<RegisterModel>) -> impl Responder {
    let user = ActiveModel {
        name: Set(data.name.clone()),
        email: Set(data.email.clone()),
        password: Set(digest(&data.password)),
        ..Default::default()
    }.insert(&app_state.db).await.unwrap();

    // TODO: return the newly created `user` without `password`
    ApiResponse::new(200, format!("{}", user.id))
}

#[post("/login")]
pub async fn login(app_state: Data<AppState>, data: Json<LoginModel>) -> impl Responder {
    let user = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(&data.email))
                .add(entity::user::Column::Password.eq(digest(&data.password)))
        ).one(&app_state.db).await.unwrap();

    if user.is_none() {
        return ApiResponse::new(401, String::from("User not found"));
    }

    let user_data = user.unwrap();
    let token = encode_jwt(user_data.email, user_data.id).unwrap();

    ApiResponse::new(200, format!("{{ 'token': '{}' }}", token))
}