use axum::{
    body::Body,
    http::{HeaderValue, Response},
    Extension, Form,
};
use libsql::Connection;

use crate::{errors::AppError, user::encrypt_password};

use super::signin::SignInForm;

pub async fn sign_up(
    Extension(conn): Extension<Connection>,
    Form(sign_up): Form<SignInForm>,
) -> Result<Response<Body>, AppError> {
    let (password, salt) = encrypt_password(&sign_up.password);

    let params = [sign_up.username.clone(), password, salt];

    conn.execute(
        "INSERT INTO owner (username, password, salt) VALUES (lower(?1), ?2, ?3)",
        &params,
    )
    .await?;

    let mut res = Response::new(Body::empty());
    res.headers_mut()
        .insert("Hx-Redirect", HeaderValue::from_static("/signin"));

    Ok(res)
}
