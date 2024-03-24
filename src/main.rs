use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio_postgres::{NoTls, Error};
// the input to our `create_user` handler
#[tokio::main]
async fn main() -> Result<(), Error>{
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

      let (client, connection) =
      tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;
      tokio::spawn(async move {
      if let Err(e) = connection.await {
          eprintln!("connection error: {}", e);
      }
  });
  Ok(())
}

async fn root() -> &'static str {
    "Hello, World"
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
