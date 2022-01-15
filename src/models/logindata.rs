
#[derive(serde::Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct ResetPassword {
    pub email: String,
}
