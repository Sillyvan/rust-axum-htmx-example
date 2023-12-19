mod errors;
mod handler;
mod model;
mod utils;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use errors::AppError;
use handler::{
    nav::nav,
    query_cats::{query_cats, query_cats_delete},
    query_cats_form::{query_cats_form_get, query_cats_form_post},
    signin::sign_in,
    signout::sign_out,
    signup::sign_up,
};
use libsql::Database;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let db = Database::open("database.db")?;
    let conn = db.connect()?;

    let app = Router::new()
        .route("/api/nav", get(nav))
        .route(
            "/api/cats",
            get(query_cats)
                .post(query_cats_form_post)
                .delete(query_cats_delete),
        )
        .route("/api/cats/form", get(query_cats_form_get))
        .route("/api/signup", post(sign_up))
        .route("/api/signin", post(sign_in))
        .route("/api/signout", post(sign_out))
        .layer(Extension(conn));

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
