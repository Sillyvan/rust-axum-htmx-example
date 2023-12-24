mod errors;
mod handler;
mod model;
mod test;
mod utils;

use std::process::Command;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use errors::AppError;
use handler::{
    auth::{sign_in::sign_in, sign_out::sign_out, sign_up::sign_up},
    cats::{
        delete::delete_cat,
        get::{get_cats, get_cats_form},
        post::post_cat,
    },
    nav::nav,
};
use libsql::Database;
use tower_http::compression::CompressionLayer;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv::dotenv().ok();

    //get run argument
    let args: Vec<String> = std::env::args().collect();

    let db = match args.contains(&String::from("--test")) {
        true => {
            println!("Running test enviroment");
            match Command::new("sh").arg("./create_test_db.sh").output() {
                Ok(_) => println!("Test db created"),
                Err(e) => println!("Error creating test db: {}", e),
            }
            Database::open("test.db")?
        }
        false => Database::open("database.db")?,
    };

    let conn = db.connect()?;
    let server_url = std::env::var("SERVER_URL").expect("SERVER_URL must be set");

    let app = Router::new()
        .route("/api/nav", get(nav))
        .route("/api/cats", get(get_cats).post(post_cat).delete(delete_cat))
        .route("/api/cats/form", get(get_cats_form))
        .route("/api/signup", post(sign_up))
        .route("/api/signin", post(sign_in))
        .route("/api/signout", post(sign_out))
        .layer(Extension(conn))
        .layer(CompressionLayer::new().br(true).gzip(true));

    println!("Server running at http://{}", server_url);
    println!("Press Ctrl+C to quit.");
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(server_url).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
