use axum::http::HeaderValue;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};

use crate::handler::signin::Claims;

pub fn validate_token(cookie_header: Option<&HeaderValue>) -> Option<TokenData<Claims>> {
    let validation = Validation::default();

    let header = match cookie_header {
        Some(t) => t,
        None => return None,
    };

    let jwt = header.to_str().unwrap().split("=").collect::<Vec<&str>>()[1];

    match decode::<Claims>(
        jwt,
        &DecodingKey::from_secret("secret".as_ref()),
        &validation,
    ) {
        Ok(t) => Some(t),
        Err(_) => None,
    }
}
