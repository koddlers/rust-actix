use crate::utils::api_response::ApiResponse;
use crate::utils::app_state::AppState;

use actix_web::web::{Data, Json};
use actix_web::{post, Responder};

use entity::user::ActiveModel;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

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
        password: Set(data.password.clone()),
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
                .add(entity::user::Column::Password.eq(&data.password))
        ).one(&app_state.db).await.unwrap();

    if user.is_none() {
        return ApiResponse::new(401, String::from("User not found"));
    }

    ApiResponse::new(200, user.unwrap().name)
}