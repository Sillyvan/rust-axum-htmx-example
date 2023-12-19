use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, Response},
    Extension, Form,
};
use libsql::Connection;

use crate::{errors::AppError, model::cat::CatFormData, utils::validate_token::validate_token};

pub async fn query_cats_form_get(headers: HeaderMap) -> Result<Response<Body>, AppError> {
    let token: Option<&axum::http::HeaderValue> = headers.get("Cookie");

    let signed_in_response = format!(
        r#"
        <form hx-post='/api/cats' hx-swap='none'>
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

pub async fn query_cats_form_post(
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
