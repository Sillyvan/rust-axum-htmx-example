use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, Response},
    Extension, Form,
};
use libsql::Connection;

use crate::{errors::AppError, model::cat::CatFormData, utils::validate_token::validate_token};

pub async fn post_cat(
    headers: HeaderMap,
    Extension(conn): Extension<Connection>,
    Form(form): Form<CatFormData>,
) -> Result<Response<Body>, AppError> {
    let cookie_header: Option<&axum::http::HeaderValue> = headers.get("Cookie");
    let token = validate_token(cookie_header);

    let mut res: Response<Body> = Response::new(Body::empty());

    match token {
        Some(t) => {
            conn.execute(
                "INSERT INTO cat (name, breed, owner_id) VALUES ($1, $2, $3);",
                &[form.name, form.breed, t.claims.id],
            )
            .await?;
        }
        None => return Ok(res),
    }

    res.headers_mut()
        .insert("HX-Trigger", HeaderValue::from_static("update-cats"));

    Ok(res)
}
