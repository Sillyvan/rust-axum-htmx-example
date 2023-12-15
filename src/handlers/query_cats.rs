use axum::{response::Html, Extension};
use libsql::{de, Connection};

use crate::{errors::AppError, utils::minify::minify_response};

#[derive(Debug, serde::Deserialize)]
struct Cat {
    id: i32,
    name: String,
    breed: String,
    owner_name: String,
}

pub async fn query_cats(Extension(conn): Extension<Connection>) -> Result<Html<String>, AppError> {
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

    let mut table: String = String::new();

    while let Some(current_row) = rows.next()? {
        let cat: Cat = de::from_row::<Cat>(&current_row)?;

        table += &format!(
            r#"
          <tr key='{}'>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
          </tr>
          "#,
            cat.id, cat.name, cat.breed, cat.owner_name
        );
    }

    let response = format!(
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
        table
    );

    Ok(Html(minify_response(response)))
}
