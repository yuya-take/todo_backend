use axum::{
    routing::{get, post, delete},
    Router,
};

use crate::api::handlers::user::{
    create_user_handler,
    get_user_handler,
    get_users_handler,
    delete_user_handler,
};

// ルーターを定義する関数
pub fn app_router() -> Router {
    Router::new()
        // ここで各エンドポイントのルートを定義
        .route("/users", post(create_user_handler))
        .route("/users/:user_id", get(get_user_handler))
        .route("/users", get(get_users_handler))
        .route("/users/:user_id", delete(delete_user_handler))
}
