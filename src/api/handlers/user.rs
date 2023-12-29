use axum::{
    http::StatusCode,
    Json,
    extract::Path,
};

use crate::{
    models::user::{
        User, 
        NewUser
    },
    services::user_service,
};

// Create
pub async fn create_user_handler(Json(new_user): Json<NewUser>) -> Result<Json<User>, StatusCode> {
    user_service::create_user(new_user)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// Read
pub async fn get_user_handler(Path(user_id): Path<i32>) -> Result<Json<User>, StatusCode> {
    user_service::get_user(user_id)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// Read all
pub async fn get_users_handler() -> Result<Json<Vec<User>>, StatusCode> {
    user_service::get_users()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// Delete
pub async fn delete_user_handler(Path(user_id): Path<i32>) -> Result<StatusCode, StatusCode> {
    user_service::delete_user(user_id)
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
