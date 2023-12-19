use std::time::{SystemTime, UNIX_EPOCH};

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::body::Body;
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderValue, Response};
use axum::response::IntoResponse;
use axum::{response::Html, Extension, Form};
use jsonwebtoken::{encode, EncodingKey, Header};
use libsql::Connection;

use crate::errors::AppError;
use crate::model::Owner::OwnerFormData;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub id: String,
    pub sub: String,
    exp: u64,
}

impl Claims {
    fn new(id: String, sub: String) -> Result<Self, AppError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let exp = now + 86400; // Add one day (86400 seconds)
        Ok(Claims { id, sub, exp })
    }
}

const SELECT_OWNER: &str = "SELECT id, password FROM owner WHERE username = ?1";
const SECRET: &[u8] = b"secret";
const HX_REDIRECT: &str = "HX-Redirect";
const HX_REDIRECT_VALUE: HeaderValue = HeaderValue::from_static(REDIRECT_PATH);

pub async fn sign_in(
    Extension(conn): Extension<Connection>,
    Form(sign_in): Form<OwnerFormData>,
) -> Result<Response<Body>, AppError> {
    let failed_login = Html(format!(
        r#"
        <span>Failed to login</span>
        <a href="/signin">Try again</a>
        "#
    ))
    .into_response();

    let mut stmt: libsql::Statement = conn.prepare(SELECT_OWNER).await?;
    let mut rows = stmt.query(&[sign_in.username.clone()]).await?;

    let row: libsql::Row = match rows.next().unwrap() {
        Some(row) => row,
        None => {
            return Ok(failed_login);
        }
    };

    let id = row.get_str(0)?;
    let password = row.get_str(1)?;

    match verify_password(&sign_in.password, password) {
        Ok(_) => {
            let claims = Claims::new(id.to_string(), sign_in.username.to_string())?;
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(SECRET),
            )?;

            let mut response = Response::new(Body::empty());
            let header = response.headers_mut();
            header.insert(
                SET_COOKIE,
                HeaderValue::from_str(&format!(
                    "{}={}; {}",
                    SESSION_COOKIE, token, COOKIE_ATTRIBUTES
                ))
                .unwrap(),
            );
            header.insert(HX_REDIRECT, HX_REDIRECT_VALUE);

            Ok(response)
        }
        Err(_) => Ok(failed_login),
    }
}

const SESSION_COOKIE: &str = "session";
const COOKIE_ATTRIBUTES: &str = "Max-Age=86400; HttpOnly";
const REDIRECT_PATH: &str = "/";

fn verify_password(
    raw_password: &str,
    db_password: &str,
) -> Result<(), argon2::password_hash::Error> {
    let argon2: Argon2<'_> = Argon2::default();

    let parsed_db_hash = PasswordHash::new(db_password)?;

    argon2.verify_password(raw_password.as_bytes(), &parsed_db_hash)
}
