use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, Response},
    Extension, Form,
};
use libsql::Connection;

use crate::{
    errors::AppError, model::cat::CatDeleteFormData, utils::validate_token::validate_token,
};

pub async fn delete_cat(
    headers: HeaderMap,
    Extension(conn): Extension<Connection>,
    Form(cat): Form<CatDeleteFormData>,
) -> Result<Response<Body>, AppError> {
    let cookie_header: Option<&axum::http::HeaderValue> = headers.get("Cookie");
    let token = validate_token(cookie_header);

    match token {
        Some(t) => {
            conn.execute(
                "DELETE FROM cat WHERE id = $1 AND owner_id = $2;",
                &[cat.id.to_string(), t.claims.id],
            )
            .await?;

            let mut res = Response::new(Body::empty());
            res.headers_mut()
                .insert("HX-Trigger", HeaderValue::from_static("update-cats"));

            Ok(res)
        }
        None => return Ok(Response::new(Body::empty())),
    }
}
