use crate::utils::api_response::ApiResponse;
use crate::utils::app_state::AppState;
use crate::utils::jwt::Claims;
use actix_web::{get, post, web};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UpdateUser {
    name: String,
}

#[get("")]
pub async fn user(app_state: web::Data<AppState>, claims: Claims) -> Result<ApiResponse, ApiResponse> {
    let user = entity::user::Entity::find_by_id(claims.id)
        .one(&app_state.db).await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, String::from("User Not Found")))?;

    let data = format!("{{ 'name': '{}', 'email': '{}' }}", user.name, user.email);
    Ok(ApiResponse::new(200, data))
}


#[post("update")]
pub async fn update_user(
    app_state: web::Data<AppState>,
    user_data: web::Json<UpdateUser>,
    claims: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let mut user_model = entity::user::Entity::find_by_id(claims.id)
        .one(&app_state.db).await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, String::from("User Not Found")))?
        .into_active_model();

    // update the user
    user_model.name = Set(user_data.name.clone());
    user_model.update(&app_state.db).await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, String::from("User Updated")))
}