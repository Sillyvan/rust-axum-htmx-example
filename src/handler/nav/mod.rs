use axum::{
    body::Body,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use sailfish::TemplateOnce;

use crate::{errors::AppError, utils::validate_token::validate_token};

#[derive(TemplateOnce)]
#[template(path = "./nav/nav.stpl")]
struct SignInError {
    username: Option<String>,
}

pub async fn nav(headers: HeaderMap) -> Result<Response<Body>, AppError> {
    let cookie_header: Option<&axum::http::HeaderValue> = headers.get("Cookie");
    let token = validate_token(cookie_header);

    let responses = SignInError {
        username: token.map(|t| t.claims.sub),
    }
    .render_once()?
    .into_response();

    Ok(responses)
}
