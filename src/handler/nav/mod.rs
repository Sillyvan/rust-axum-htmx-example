use axum::{http::HeaderMap, response::Html};

use crate::{errors::AppError, utils::validate_token::validate_token};

pub async fn nav(headers: HeaderMap) -> Result<Html<String>, AppError> {
    let cookie_header: Option<&axum::http::HeaderValue> = headers.get("Cookie");
    let token = validate_token(cookie_header);

    let response = match token {
        Some(t) => {
            format!(
                r#"
    <nav>
      <ul>
        <li><span>Axum + HTMX ❤️</span></li>
      </ul>
      <ul>
          <li><span>Welcome {}</span></li>
          <li><a hx-post='/signout' >Logout</a></li>
      </ul>
    </nav>
          "#,
                t.claims.sub
            )
        }
        None => {
            format!(
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
            )
        }
    };

    Ok(Html(response))
}
