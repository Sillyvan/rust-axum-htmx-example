use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderValue, Response};
use axum::response::IntoResponse;
use axum::{response::Html, Extension, Form};
use jsonwebtoken::{encode, EncodingKey, Header};
use libsql::Connection;

use crate::{
    errors::AppError,
    user::{verify_password, User},
    utils::minify::minify_response,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

impl Claims {
    fn new(sub: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let exp = now + 60; // Add one day (86400 seconds)
        Claims {
            sub,
            exp: exp as usize,
        }
    }
}

pub async fn sign_in(
    Extension(conn): Extension<Connection>,
    Form(sign_in): Form<User>,
) -> Result<Response<Body>, AppError> {
    let mut successfull_login = Html(minify_response(format!(
        r#"
        <h1>Welcome {}</h1>
        "#,
        sign_in.username
    )))
    .into_response();

    let failed_login = Html(minify_response(format!(
        r#"
        <h1>Failed to login</h1>
        <a href="/signin">Try again</a>
        "#
    )))
    .into_response();

    let mut rows: libsql::Rows = conn
        .query(
            "SELECT salt, password FROM owner WHERE username = ?1",
            &[sign_in.username.clone()],
        )
        .await?;

    let row = match rows.next().unwrap() {
        Some(row) => row,
        None => {
            return Ok(failed_login);
        }
    };

    let salt = row.get_str(0).unwrap();
    let password = row.get_str(1).unwrap();

    match verify_password(&sign_in.password, password, salt) {
        true => {
            let claims = Claims::new(sign_in.username.clone());

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret("secret".as_ref()),
            )
            .unwrap();
            let cookie = format!("session={}; Max-Age=60; HttpOnly", token);
            successfull_login
                .headers_mut()
                .insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());
            Ok(successfull_login)
        }
        false => Ok(failed_login),
    }
}
