use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::AppError;
use axum::body::Body;
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderValue, Response};

pub async fn sign_out() -> Result<Response<Body>, AppError> {
    let mut response = Response::new(Body::empty());
    let header = response.headers_mut();
    header.insert(
        SET_COOKIE,
        HeaderValue::from_static("session=; Max-Age=0; expired"),
    );
    header.insert("hx-redirect", HeaderValue::from_static("/"));
    Ok(response)
}
