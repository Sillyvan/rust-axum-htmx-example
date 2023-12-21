use axum::{
    body::Body,
    http::HeaderMap,
    response::{IntoResponse, Response},
    Extension,
};
use libsql::Connection;

use crate::{errors::AppError, model::cat::Cat, utils::validate_token::validate_token};

use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "./cat_table.stpl")]
struct CatTable<'a> {
    username: &'a Option<String>,
    cats: &'a Vec<Cat>,
}

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

    let mut cats = vec![];
    while let Some(current_row) = rows.next()? {
        let cat: Cat = libsql::de::from_row::<Cat>(&current_row)?;
        cats.push(cat);
    }

    let response = CatTable {
        username: &token.map(|t| t.claims.sub),
        cats: &cats,
    }
    .render_once()?
    .into_response();

    Ok(response)
}

#[derive(TemplateOnce)]
#[template(path = "./auth/sign_in.stpl")]
struct SignInTemplate;

pub async fn get_cats_form(headers: HeaderMap) -> Result<Response<Body>, AppError> {
    let token: Option<&axum::http::HeaderValue> = headers.get("Cookie");

    let signed_in_response2 = SignInTemplate {}.render_once()?.into_response();

    return match token {
        Some(_t) => Ok(signed_in_response2),
        None => Ok(Response::new(Body::empty())),
    };
}
