use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash,
};

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
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
