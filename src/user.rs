use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash,
};

#[derive(Debug, serde::Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

pub fn encrypt_password(password: &str) -> (String, String) {
    let salt = SaltString::generate(&mut OsRng);
    let argon2: Argon2<'_> = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();

    let encrypt_password = PasswordHash::from(password_hash).hash.unwrap().to_string();

    (encrypt_password, salt.to_string())
}

pub fn verify_password(raw_password: &str, db_password: &str, salt: &str) -> bool {
    let argon2: Argon2<'_> = Argon2::default();

    let salt_string = SaltString::from_b64(&salt).unwrap();
    let password_hash = argon2
        .hash_password(raw_password.as_bytes(), &salt_string)
        .unwrap();

    let encrypt_password = PasswordHash::from(password_hash).hash.unwrap().to_string();

    encrypt_password == db_password
}
