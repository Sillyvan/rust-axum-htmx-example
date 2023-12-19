use axum::{
    body::Body,
    http::{HeaderMap, Response},
    response::IntoResponse,
    Extension,
};
use libsql::Connection;

use crate::{
    errors::AppError,
    utils::{generate_table::generate_table, validate_token::validate_token},
};

pub async fn get_cats(
    headers: HeaderMap,
    Extension(conn): Extension<Connection>,
) -> Result<Response<Body>, AppError> {
    let cookie_header: Option<&axum::http::HeaderValue> = headers.get("Cookie");
    let token = validate_token(cookie_header);

    let mut rows = conn
        .query(
            "SELECT 
              cat.id,
              cat.name,
              cat.breed,
              owner.username AS owner_name
            FROM 
              cat
            JOIN 
              owner ON cat.owner_id = owner.id;",
            (),
        )
        .await?;

    let res = generate_table(&mut rows, token.map(|t| t.claims.sub))
        .await?
        .into_response();

    Ok(res)
}

pub async fn get_cats_form(headers: HeaderMap) -> Result<Response<Body>, AppError> {
    let token: Option<&axum::http::HeaderValue> = headers.get("Cookie");

    let signed_in_response = format!(
        r#"
        <form hx-post='/api/cats' hx-swap='none' hx-on::after-request="this.reset()">
            <h2>Add Cat</h2>
            <input type="text" name="name" placeholder="name" />
            <input type="text" name="breed" placeholder="breed" />
            <input type="submit" value="Add cat" />
        </form>
        
        "#,
    );

    return match token {
        Some(_t) => Ok(Response::new(Body::from(signed_in_response))),
        None => Ok(Response::new(Body::empty())),
    };
}
