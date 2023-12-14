use axum::{response::Html, Extension};
use libsql::{de, Connection};
use minify_html::{minify, Cfg};

use crate::errors::AppError;

#[derive(Debug, serde::Deserialize)]
struct Cat {
    id: i32,
    name: String,
    breed: String,
    owner: String,
}

pub async fn query_cats(Extension(conn): Extension<Connection>) -> Result<Html<String>, AppError> {
    let mut rows = conn
        .query(
            "SELECT 
                  cat.id,
                  cat.name,
                  cat.breed,
                  owner.username AS owner
                FROM 
                  cat
                JOIN 
                  owner ON cat.owner_id = owner.id;",
            (),
        )
        .await?;

    let mut table: String = String::new();

    let row = rows.next().unwrap().unwrap();

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
            cat.id, cat.name, cat.breed, cat.owner
        );
        println!("row loop: {:?}", row);
    }

    let idk = row.column_name(1);

    println!("idk: {:?}", idk);
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

    let minified_reponse = String::from_utf8(minify(&response.as_bytes(), &Cfg::new()))?;

    Ok(Html(minified_reponse))
}
