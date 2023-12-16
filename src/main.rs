mod errors;
mod handlers;
mod user;
mod utils;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use errors::AppError;
use handlers::{query_cats::query_cats, signin::sign_in, signup::sign_up};
use libsql::Database;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let db = Database::open("database.db")?;
    let conn = db.connect()?;

    let app = Router::new()
        .route("/api/cats", get(query_cats))
        .route("/api/signup", post(sign_up))
        .route("/api/signin", post(sign_in))
        .layer(Extension(conn));

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
