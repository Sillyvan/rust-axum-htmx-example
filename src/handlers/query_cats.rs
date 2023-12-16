use axum::{http::HeaderMap, response::Html, Extension};
use jsonwebtoken::{decode, DecodingKey, Validation};
use libsql::{de, Connection};

use crate::{
    errors::AppError,
    utils::{minify::minify_response, validate_token::validate_token},
};

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
) -> Result<Html<String>, AppError> {
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

    println!("username: {:?}", username);

    let table = generate_table(&mut rows, validate_token(token), username).await?;

    Ok(Html(minify_response(table)))
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
            <tr key='{}'>s
              <td>{}</td>
              <td>{}</td>
              <td>{}</td>
              <td><a role='button' style='background-color: red' href='#'>x</a></td>
            </tr>
            "#,
                        cat.id, cat.name, cat.breed, cat.owner_name
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
          <table>
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
          </table>
          "#,
            table_rows.join("\n")
        )
    } else {
        format!(
            r#"
          <table>
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
          </table>
          "#,
            table_rows.join("\n")
        )
    };

    Ok(table)
}
