#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Cat {
    pub id: i64,
    pub name: String,
    pub breed: String,
    pub owner_name: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CatFormData {
    pub name: String,
    pub breed: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CatDeleteFormData {
    pub id: i64,
}
