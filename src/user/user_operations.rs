use crate::user::user::{CreateUser,User};

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

pub async fn root() -> &'static str {
    "Hello, World"
}

pub async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}