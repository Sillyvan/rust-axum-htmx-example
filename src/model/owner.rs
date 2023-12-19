#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Owner {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub salt: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OwnerFormData {
    pub username: String,
    pub password: String,
}
