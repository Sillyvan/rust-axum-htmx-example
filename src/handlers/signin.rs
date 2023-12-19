use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderValue, Response};
use axum::response::IntoResponse;
use axum::{response::Html, Extension, Form};
use jsonwebtoken::{encode, EncodingKey, Header};
use libsql::Connection;

use crate::{errors::AppError, user::verify_password};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub id: String,
    pub sub: String,
    exp: usize,
}
#[derive(Debug, serde::Deserialize)]

pub struct SignInForm {
    pub username: String,
    pub password: String,
}

impl Claims {
    fn new(id: String, sub: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let exp = now + 86400; // Add one day (86400 seconds)
        Claims {
            id,
            sub,
            exp: exp as usize,
        }
    }
}

pub async fn sign_in(
    Extension(conn): Extension<Connection>,
    Form(sign_in): Form<SignInForm>,
) -> Result<Response<Body>, AppError> {
    let failed_login = Html(format!(
        r#"
        <span>Failed to login</span>
        <a href="/signin">Try again</a>
        "#
    ))
    .into_response();

    let mut rows: libsql::Rows = conn
        .query(
            "SELECT id, salt, password FROM owner WHERE username = ?1",
            &[sign_in.username.clone()],
        )
        .await?;

    let row = match rows.next().unwrap() {
        Some(row) => row,
        None => {
            return Ok(failed_login);
        }
    };

    let id = row.get_str(0).unwrap();
    let salt = row.get_str(1).unwrap();
    let password = row.get_str(2).unwrap();

    match verify_password(&sign_in.password, password, salt) {
        true => {
            let claims = Claims::new(id.to_string(), sign_in.username.clone());

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret("secret".as_ref()),
            )
            .unwrap();
            let cookie = format!("session={}; Max-Age=86400; HttpOnly", token);
            let mut res = Response::new(Body::empty());
            let headers = res.headers_mut();
            headers.insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());
            headers.insert("HX-Redirect", HeaderValue::from_static("/"));
            Ok(res)
        }
        false => Ok(failed_login),
    }
}
