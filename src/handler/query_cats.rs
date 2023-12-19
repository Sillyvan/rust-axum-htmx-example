use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, Response},
    response::IntoResponse,
    Extension, Form,
};
use libsql::{de, Connection};

use crate::{
    errors::AppError,
    model::cat::{Cat, CatDeleteFormData},
    utils::validate_token::validate_token,
};

pub async fn query_cats(
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

pub async fn query_cats_delete(
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

async fn generate_table(
    rows: &mut libsql::Rows,
    username: Option<String>,
) -> Result<String, AppError> {
    let mut table_rows: Vec<String> = Vec::new();

    match username.clone() {
        Some(u) => {
            while let Some(current_row) = rows.next()? {
                let cat: Cat = de::from_row::<Cat>(&current_row)?;

                let row: String = if cat.owner_name == u {
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

    let table = if username.is_some() {
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
