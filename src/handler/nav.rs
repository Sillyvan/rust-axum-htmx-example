use axum::{http::HeaderMap, response::Html};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::errors::AppError;

use super::signin::Claims;

pub async fn nav(headers: HeaderMap) -> Result<Html<String>, AppError> {
    let token: Option<&axum::http::HeaderValue> = headers.get("Cookie");

    let jwt = match token {
        Some(t) => Some(t.to_str().unwrap().split("=").collect::<Vec<&str>>()[1]),
        None => None,
    };

    let username = match jwt {
        Some(t) => Some(
            decode::<Claims>(
                t,
                &DecodingKey::from_secret("secret".as_ref()),
                &Validation::default(),
            )
            .unwrap()
            .claims
            .sub,
        ),
        None => None,
    };

    let response = match username {
        Some(u) => format!(
            r#"
      <nav>
        <ul>
          <li><span>Axum + HTMX ❤️</span></li>
        </ul>
        <ul>
            <li><span>Welcome {}</span></li>
            <li><a hx-post='/api/signout' >Logout</a></li>
        </ul>
      </nav>
            "#,
            u
        ),
        None => format!(
            r#"
      <nav>
        <ul>
          <li><span>Axum + HTMX ❤️</span></li>
        </ul>
        <ul>
          <li><a href="/">Home</a></li>
          <li><a href="/signin">Sign In</a></li>
          <li><a href="/signup">Sign Up</a></li>
        </ul>
      </nav>
            "#
        ),
    };

    Ok(Html(response))
}
