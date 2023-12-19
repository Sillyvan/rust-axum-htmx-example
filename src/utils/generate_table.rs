use libsql::de;

use crate::{errors::AppError, model::cat::Cat};

pub async fn generate_table(
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
              <td><a role='button' class="outline" href='/' hx-delete='/api/cats' hx-vals='{{"id": {}}}'>Delete</a></td>
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
