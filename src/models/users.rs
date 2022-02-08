use actix_web::web;
use sqlx::MySqlPool;
use sqlx::mysql::MySqlQueryResult;
use guid_create::GUID;

#[derive(serde::Deserialize)]
pub struct SimpleUser {
    pub email: String,
    pub id: String,
}

#[derive(serde::Deserialize)]
pub struct ResetPassword {
    pub email: String,
}

#[derive(serde::Deserialize)]
pub struct UpdatePassword {
    pub email: String,
    pub remember_token: String,
    pub password: String,
}
#[derive(serde::Deserialize)]
pub struct LoginData {
    pub id: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}


pub async fn get_login_data(email: &str, pool: &web::Data<MySqlPool>) -> Result<LoginData, sqlx::Error> {
    // get user from database table
    let user_record = sqlx::query_as!(LoginData,
        r#"
            SELECT id, email, password
            FROM users
            WHERE email = ?
            LIMIT 1
        "#,
        email
    )
    .fetch_one(pool.get_ref())
    .await;
    user_record
}

pub async fn set_remember_token(email: &str, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error> {
    let guid = GUID::rand();
    log::info!("Found user and creating guid of {}", guid.to_string());
    let update = sqlx::query!(
        r#"
        UPDATE users SET remember_token = ?
        WHERE email = ?
        "#,
        guid.to_string(),
        email
    ).execute(pool.get_ref()).await;
    update
}
