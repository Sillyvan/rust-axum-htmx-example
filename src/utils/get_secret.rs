pub fn get_secret() -> String {
    std::env::var("SECRET").expect("SECRET must be set")
}
