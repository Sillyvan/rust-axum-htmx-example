use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash,
};

#[derive(Debug, serde::Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub fn encrypt_password(password: String) -> (String, String) {
    let salt = SaltString::generate(&mut OsRng);
    let argon2: Argon2<'_> = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let encrypt_password = PasswordHash::new(&password_hash)
        .unwrap()
        .hash
        .unwrap()
        .to_string();

    (encrypt_password, salt.to_string())
}

// get pw from form
// get salt from db
// hash pw with salt
// compare hash with hash in db

pub fn verify_password(raw_password: &str, db_password: &str, salt: &str) -> bool {
    let argon2: Argon2<'_> = Argon2::default();

    let password_hash = argon2
        .hash_password(
            raw_password.as_bytes(),
            &SaltString::from_b64(&salt).unwrap(),
        )
        .unwrap()
        .to_string();

    let encrypt_password = PasswordHash::new(&password_hash)
        .unwrap()
        .hash
        .unwrap()
        .to_string();

    encrypt_password == db_password
}
