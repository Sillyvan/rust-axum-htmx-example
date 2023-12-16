use axum::{response::Html, Extension, Form};
use libsql::Connection;

use crate::{
    errors::AppError,
    user::{encrypt_password, User},
    utils::minify::minify_response,
};

pub async fn sign_up(
    Extension(conn): Extension<Connection>,
    Form(sign_up): Form<User>,
) -> Result<Html<String>, AppError> {
    let (password, salt) = encrypt_password(sign_up.password);

    let params = [sign_up.username.clone(), password, salt];

    conn.execute(
        "INSERT INTO owner (username, password, salt) VALUES (?1, ?2, ?3)",
        &params,
    )
    .await?;

    let response = format!(
        r#"
        <h1>Welcome {}</h1>
        "#,
        sign_up.username
    );

    Ok(Html(minify_response(response)))
}
