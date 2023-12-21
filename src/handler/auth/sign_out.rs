use crate::errors::AppError;
use axum::body::Body;
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderValue, Response};

const HX_LOCATION: &str = "HX-LOCATION";
const HX_LOCATION_VALUE: &str = "/";
const EXPIRY_COOKIE: &str = "session=; Max-Age=0; expired";

pub async fn sign_out() -> Result<Response<Body>, AppError> {
    let mut response = Response::new(Body::empty());
    let header = response.headers_mut();
    header.insert(SET_COOKIE, HeaderValue::from_static(EXPIRY_COOKIE));
    header.insert(HX_LOCATION, HeaderValue::from_static(HX_LOCATION_VALUE));
    Ok(response)
}
