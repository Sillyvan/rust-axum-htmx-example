use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, Response},
    Extension, Form,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use libsql::Connection;

use crate::errors::AppError;

use super::signin::Claims;

#[derive(Debug, serde::Deserialize)]
pub struct CatForm {
    pub name: String,
    pub breed: String,
}

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
    Form(form): Form<CatForm>,
) -> Result<Response<Body>, AppError> {
    let token: Option<&axum::http::HeaderValue> = headers.get("Cookie");

    let jwt = match token {
        Some(t) => Some(t.to_str().unwrap().split("=").collect::<Vec<&str>>()[1]),
        None => None,
    };

    let owner_id = match jwt {
        Some(t) => Some(
            decode::<Claims>(
                t,
                &DecodingKey::from_secret("secret".as_ref()),
                &Validation::default(),
            )
            .unwrap()
            .claims
            .id,
        ),

        None => return Ok(Response::new(Body::empty())),
    };

    conn.execute(
        "INSERT INTO cat (name, breed, owner_id) VALUES ($1, $2, $3);",
        &[form.name, form.breed, owner_id.unwrap()],
    )
    .await?;

    let mut res = Response::new(Body::empty());
    res.headers_mut()
        .insert("HX-Trigger", HeaderValue::from_static("update-cats"));

    Ok(res)
}
