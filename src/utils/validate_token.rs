use axum::http::HeaderValue;
use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey, Validation};

use crate::handlers::signin::Claims;

pub fn validate_token(token: Option<&HeaderValue>) -> bool {
    let validation = Validation::default();

    let binding = match token {
        Some(token) => match token.to_str() {
            Ok(token_str) => token_str.split("=").collect::<Vec<&str>>(),
            Err(_) => return false,
        },
        None => return false,
    };
    let jwt = match binding.get(1) {
        Some(jwt) => jwt,
        None => return false,
    };

    match decode::<Claims>(
        jwt,
        &DecodingKey::from_secret("secret".as_ref()),
        &validation,
    ) {
        Ok(_) => true,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => false,
            _ => false,
        },
    }
}
