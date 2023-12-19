use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, Response},
    response::IntoResponse,
    Extension, Form,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use libsql::{de, Connection};

use crate::{errors::AppError, utils::validate_token::validate_token};

use super::signin::Claims;

#[derive(Debug, serde::Deserialize)]
struct Cat {
    id: i32,
    name: String,
    breed: String,
    owner_name: String,
}

pub async fn query_cats(
    headers: HeaderMap,
    Extension(conn): Extension<Connection>,
) -> Result<Response<Body>, AppError> {
    let token: Option<&axum::http::HeaderValue> = headers.get("Cookie");

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

    let response = generate_table(&mut rows, validate_token(token), username)
        .await?
        .into_response();

    Ok(response)
}

#[derive(Debug, serde::Deserialize)]
pub struct IdkMan {
    id: i32,
}

pub async fn query_cats_delete(
    headers: HeaderMap,
    Extension(conn): Extension<Connection>,
    Form(id): Form<IdkMan>,
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
        "DELETE FROM cat WHERE id = $1 AND owner_id = $2;",
        &[id.id.to_string(), owner_id.unwrap()],
    )
    .await?;

    let mut res = Response::new(Body::empty());
    res.headers_mut()
        .insert("HX-Trigger", HeaderValue::from_static("update-cats"));

    Ok(res)
}

async fn generate_table(
    rows: &mut libsql::Rows,
    is_token_valid: bool,
    username: Option<String>,
) -> Result<String, AppError> {
    let mut table_rows: Vec<String> = Vec::new();

    match username {
        Some(u) => {
            while let Some(current_row) = rows.next()? {
                let cat: Cat = de::from_row::<Cat>(&current_row)?;

                let row: String = if is_token_valid && cat.owner_name == u {
                    format!(
                        r#"
            <tr key='{}'>
              <td>{}</td>
              <td>{}</td>
              <td>{}</td> 
              <td><a role='button' href='/' hx-delete='/api/cats' hx-vals='{{"id": {}}}'>x</a></td>
            </tr>
            "#,
                        cat.id, cat.name, cat.breed, cat.owner_name, cat.id
                    )
                } else {
                    format!(
                        r#"
            <tr key='{}'>
              <td>{}</td>
              <td>{}</td>
              <td>{}</td>
              <td></td>
            </tr>
            "#,
                        cat.id, cat.name, cat.breed, cat.owner_name
                    )
                };
                table_rows.push(row);
            }
        }
        None => {
            while let Some(current_row) = rows.next()? {
                let cat: Cat = de::from_row::<Cat>(&current_row)?;

                let row = format!(
                    r#"
            <tr key='{}'>
              <td>{}</td>
              <td>{}</td>
              <td>{}</td>
            </tr>
            "#,
                    cat.id, cat.name, cat.breed, cat.owner_name
                );
                table_rows.push(row);
            }
        }
    }

    let table = if is_token_valid {
        format!(
            r#"
            <thead>
              <tr>
                <th>Name</th>
                <th>Breed</th>
                <th>Owner</th>
                <th>Action</th>
              </tr>
            </thead>
            <tbody>
              {}
            </tbody>
          "#,
            table_rows.join("\n")
        )
    } else {
        format!(
            r#"
            <thead>
              <tr>
                <th>Name</th>
                <th>Breed</th>
                <th>Owner</th>
              </tr>
            </thead>
            <tbody>
              {}
            </tbody>
          </>
          "#,
            table_rows.join("\n")
        )
    };

    Ok(table)
}
