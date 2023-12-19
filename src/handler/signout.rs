use crate::errors::AppError;
use axum::body::Body;
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderValue, Response};

const REDIRECT_PATH: &str = "/";
const EXPIRY_COOKIE: &str = "session=; Max-Age=0; expired";

pub async fn sign_out() -> Result<Response<Body>, AppError> {
    let mut response = Response::new(Body::empty());
    let header = response.headers_mut();
    header.insert(SET_COOKIE, HeaderValue::from_static(EXPIRY_COOKIE));
    header.insert("hx-redirect", HeaderValue::from_static(REDIRECT_PATH));
    Ok(response)
}
