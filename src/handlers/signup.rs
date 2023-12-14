use axum::{response::Html, Extension, Form};
use libsql::Connection;
use serde::Deserialize;

use crate::{errors::AppError, user::encrypt_password};

#[derive(Deserialize, Debug)]
pub struct SignUp {
    username: String,
    password: String,
}

pub async fn sign_up(
    Extension(conn): Extension<Connection>,
    Form(sign_up): Form<SignUp>,
) -> Result<Html<String>, AppError> {
    println!("Username: {}", sign_up.username);
    println!("Password: {}", sign_up.password);

    let password = encrypt_password(sign_up.password);

    let mut rows = conn
        .query(
            "SELECT 
    cat.id,
    cat.name AS cat_name,
    cat.breed,
    owner.username AS owner_name
FROM 
    cat
JOIN 
    owner ON cat.owner_id = owner.id;",
            (),
        )
        .await?;

    // Return a response
    Ok(Html(format!(
        r#"
        <h1>Sign Uped</h1>
        "#
    )))
}
