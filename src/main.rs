use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash,
};
use axum::{response::Html, routing::get, Router};
use libsql::{Connection, Database, Rows};
use tower_http::cors::CorsLayer;

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    password: String,
}

#[derive(Debug)]
struct Genres {
    media_type_id: i32,
    name: String,
}

impl From<libsql::Row> for Genres {
    fn from(row: libsql::Row) -> Self {
        Genres {
            media_type_id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
        }
    }
}

#[tokio::main]
async fn main() {
    let db = Database::open("example.db")
        .map_err(|e| {
            eprintln!("Error opening database: {}", e);
        })
        .unwrap();
    let conn = db
        .connect()
        .map_err(|e| {
            eprintln!("Error onnecting database: {}", e);
        })
        .unwrap();

    let user: User = User {
        id: 1,
        name: String::from("name"),
        password: String::from("password"),
    };

    encrypt_user(user);

    let app = Router::new()
        .route("/api/cats", get(|| query_cats(conn)))
        .layer(CorsLayer::permissive());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn query_cats(conn: Connection) -> Html<String> {
    let mut rows = conn
        .query("SELECT * FROM genres;", ())
        .await
        .map_err(|e| eprintln!("Error executing query: {}", e))
        .unwrap();

    let mut table: String = String::new();

    while let Some(row_result) = rows.next().unwrap() {
        let row: Genres = row_result.into();
        table += &format!(
            r#"
            <tr>
              <td>{}</td>
              <td>{}</td>
            </tr>
            "#,
            row.media_type_id, row.name
        );
        println!("row loop: {:?}", row);
    }

    Html(format!(
        r#"
        <table>
          <thead>
            <tr>
              <th>Id</th>
              <th>Name</th>
            </tr>
          </thead>
          <tbody>
            {}
          </tbody>
        </table>
        "#,
        table
    ))
}
fn encrypt_user(mut user: User) -> User {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    user.password = PasswordHash::new(&password_hash)
        .unwrap()
        .hash
        .unwrap()
        .to_string();
    println!("user: {:?}", user);
    user
}

fn login() {
    todo!()
}

fn register() {
    todo!()
}
