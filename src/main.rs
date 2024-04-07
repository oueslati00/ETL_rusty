use axum::{
    routing::{get, post},
     Router
};
use tokio_postgres::{NoTls, Error};
mod user;
// the input to our `create_user` handler
#[tokio::main]
async fn main() -> Result<(), Error>{
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(user::user_operations::root))
        .route("/users", post(user::user_operations::create_user));

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


