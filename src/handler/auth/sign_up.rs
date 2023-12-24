use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher,
};
use axum::{
    body::Body,
    http::{HeaderValue, Response},
    response::IntoResponse,
    Extension, Form,
};
use libsql::Connection;

use crate::{errors::AppError, model::owner::OwnerFormData};

trait Hash {
    fn hash(self) -> Result<(String, String, String), AppError>;
}

impl Hash for OwnerFormData {
    fn hash(self) -> Result<(String, String, String), AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2: Argon2<'_> = Argon2::default();

        let password_hash = argon2
            .hash_password(&self.password.as_bytes(), &salt)
            .unwrap();

        let password = PasswordHash::from(password_hash).to_string();
        Ok((self.username, password, salt.to_string()))
    }
}

const INSERT_OWNER: &str =
    "INSERT INTO owner (username, password, salt) VALUES (lower(?1), ?2, ?3)";

const HX_LOCATION: &str = "HX-LOCATION";
const HX_LOCATION_VALUE: HeaderValue = HeaderValue::from_static("/signin");
const USERNAME_TAKEN: &str = "<div id='signup-error'>Username already taken</div>";

pub async fn sign_up(
    Extension(conn): Extension<Connection>,
    Form(owner): Form<OwnerFormData>,
) -> Result<Response<Body>, AppError> {
    let (username, password, salt) = owner.hash()?;

    let mut stmt = conn.prepare(INSERT_OWNER).await?;

    let result = stmt.execute(&[username, password, salt]).await;

    match result {
        Ok(_) => {
            let mut res = Response::new(Body::empty());
            res.headers_mut().insert(HX_LOCATION, HX_LOCATION_VALUE);
            return Ok(res);
        }
        Err(_) => Ok(USERNAME_TAKEN.into_response()),
    }
}
