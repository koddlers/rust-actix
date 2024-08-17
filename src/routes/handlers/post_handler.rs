use crate::utils;
use crate::utils::{api_response, app_state, jwt::Claims};
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web};
use chrono::NaiveDateTime;
use chrono::Utc;
use sea_orm::ColumnTrait;
use sea_orm::{ActiveModelTrait, TransactionTrait};
use sea_orm::{EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(MultipartForm)]
struct CreatePostModel {
    title: Text<String>,
    text: Text<String>,
    file: TempFile,
}

#[derive(Serialize, Deserialize)]
struct PostModel {
    id: i32,
    title: String,
    text: String,
    uuid: Uuid,
    image: Option<String>,
    user_id: i32,
    created_at: NaiveDateTime,
    user: Option<UserModel>,
}
#[derive(Serialize, Deserialize)]
struct UserModel {
    name: String,
    email: String,
}

#[post("create")]
async fn create_post(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
    post_model: MultipartForm<CreatePostModel>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let check_name = post_model.file.file_name.clone().unwrap_or(String::from("null"));
    let max_file_size = (*utils::constants::MAX_FILE_SIZE).clone();

    match &check_name[check_name.len() - 4..] {
        ".png" | ".jpg" => {}
        _ => {
            return Err(api_response::ApiResponse::new(400, String::from("Invalid file type")))
        }
    }

    match post_model.file.size {
        0 => {
            return Err(api_response::ApiResponse::new(400, String::from("Invalid file type")))
        }
        length if length > max_file_size as usize => {
            return Err(api_response::ApiResponse::new(400, String::from("File too big")))
        }

        _ => {}
    }

    let txn = app_state.db.begin().await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;


    let post_entity = entity::post::ActiveModel
    {
        title: Set(post_model.title.clone()),
        text: Set(post_model.text.clone()),
        uuid: Set(Uuid::new_v4()),
        user_id: Set(claim.id),
        created_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };

    let mut new_post = post_entity.save(&txn).await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    // let temp_file_path = post_model.file.file.path();
    let temp_file_path = Path::new("C:\\GitHub\\playground\\rust\\actix-web\\rust-actix\\temp");
    let file_name = post_model.file.file_name.as_ref()
        .map(|m| m.as_ref())
        .unwrap_or("null");

    let timestamp = Utc::now().timestamp();

    let file_path = PathBuf::from("C:\\GitHub\\playground\\rust\\actix-web\\rust-actix\\public");
    let new_file_name = format!("{}-{}", timestamp, file_name);

    match std::fs::copy(temp_file_path, file_path) {
        Ok(_) => {
            new_post.image = Set(Some(new_file_name));
            new_post.save(&txn)
                .await
                .map_err(|err| api_response::ApiResponse::new(500, String::from("couldn't save new post ") + &*err.to_string()))?;

            txn.commit()
                .await
                .map_err(|err| api_response::ApiResponse::new(500, String::from("database transaction failed ") + &*err.to_string()))?;

            std::fs::remove_file(temp_file_path).unwrap_or_default();

            Ok(api_response::ApiResponse::new(200, "Post Created".to_owned()))
        }
        Err(err) => {
            txn.rollback()
                .await
                .map_err(|err| api_response::ApiResponse::new(500, String::from("rolling back transaction ") + &*err.to_string()))?;

            Err(api_response::ApiResponse::new(500, String::from("total failure ") + &*err.to_string()))
        }
    }
}

#[get("my-posts")]
async fn my_posts(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let posts: Vec<PostModel> = entity::post::Entity::find()
        .filter(entity::post::Column::UserId.eq(claim.id)).all(&app_state.db).await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(|post|
            PostModel {
                id: post.id,
                title: post.title,
                text: post.text,
                uuid: post.uuid,
                image: post.image,
                user_id: post.user_id,
                created_at: post.created_at,
                user: None,
            }
        ).collect();
    let res_str = serde_json::to_string(&posts)
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
}


#[get("all-posts")]
async fn all_posts(
    app_state: web::Data<app_state::AppState>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let posts: Vec<PostModel> = entity::post::Entity::find()
        .all(&app_state.db).await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(|post|
            PostModel {
                id: post.id,
                title: post.title,
                text: post.text,
                uuid: post.uuid,
                image: post.image,
                user_id: post.user_id,
                created_at: post.created_at,
                user: None,
            }
        ).collect();
    let res_str = serde_json::to_string(&posts)
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
}


#[get("post/{post_uuid}")]
async fn one_post(
    app_state: web::Data<app_state::AppState>,
    post_uuid: web::Path<Uuid>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let posts: PostModel = entity::post::Entity::find()
        .filter(entity::post::Column::Uuid.eq(post_uuid.clone()))
        .find_also_related(entity::user::Entity)
        .one(&app_state.db).await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .map(|post|
            PostModel {
                id: post.0.id,
                title: post.0.title,
                text: post.0.text,
                uuid: post.0.uuid,
                image: post.0.image,
                user_id: post.0.user_id,
                created_at: post.0.created_at,
                user: post.1.map(|item| UserModel { name: item.name, email: item.email }),
            }
        )
        .ok_or(api_response::ApiResponse::new(404, "No Post Found".to_string()))?;

    let res_str = serde_json::to_string(&posts)
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
}