use actix_web::web;
use sqlx::MySqlPool;
use sqlx::mysql::MySqlQueryResult;
use guid_create::GUID;
use serde::{Serialize};
use bcrypt::*;


#[derive(serde::Deserialize, Serialize)]
pub struct UserData {
    pub email: String,
    name: String,
    pub password: String,
    account_type: String,
    location: String,
}

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
pub struct PasswordUpdate {
    pub password: String,
    pub old_password: String,
}

#[derive(serde::Deserialize)]
pub struct UpdatePassword {
    pub email: String,
    pub remember_token: String,
    pub password: String,
}

#[derive(serde::Deserialize, Serialize)]
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

#[derive(serde::Deserialize)]
pub struct AddUrl {
    pub url: String,
}

#[derive(serde::Deserialize)]
pub struct Profile {
    pub id: String,
}

#[derive(serde::Deserialize, Serialize)]
pub struct ProfileData {
    pub id: String,
    pub name: String,
    pub email: String,
    pub account_type: String,
    pub location: String,
    pub embed_url: Option<String>,
    pub avatar_url: Option<String>,
    pub image_url: Option<String>
}

#[derive(serde::Deserialize, Serialize)]
pub struct UpdateProfileData {
    name: Option<String>,
    email: Option<String>,
    location: Option<String>,
    embed_url: Option<String>,
    avatar_url: Option<String>,
    image_url: Option<String>
}

pub async fn get_profile_data(user: &str, pool: &web::Data<MySqlPool>) -> Result<ProfileData, sqlx::Error> {
    // get user profile from database table
    let profile_record = sqlx::query_as!(ProfileData,
        r#"
            SELECT id, name, email, account_type, location, embed_url, image_url, avatar_url
            FROM users
            WHERE id = ?
            LIMIT 1
        "#,
        user
    )
    .fetch_one(pool.get_ref())
    .await;
    profile_record
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

pub async fn get_simple_user(form: &web::Json<UpdatePassword>, pool: &web::Data<MySqlPool>) -> Result<SimpleUser, sqlx::Error> {
    let user_record = sqlx::query_as!(SimpleUser,
        r#"
            SELECT email, id
            FROM users
            WHERE email = ?
            AND remember_token = ?
            LIMIT 1
        "#,
        form.email,
        form.remember_token
    )
    .fetch_one(pool.get_ref())
    .await;
    user_record
}

pub async fn update_user_password(password: &str, id: &str, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
            r#"
            UPDATE users SET password = ?
            WHERE id = ?
            "#,
            password,
            id
        ).execute(pool.get_ref()).await
}

pub async fn create_session_token(id:&str, pool: &web::Data<MySqlPool>) -> Result<String, sqlx::Error> {
    let existing_token = sqlx::query!(
        r#"
        SELECT token FROM sessions
        WHERE user = ?
        AND created > DATE_SUB(NOW(), INTERVAL 1 HOUR)
        "#,
        id
    ).fetch_one(pool.get_ref())
    .await;
    match existing_token
    {
        Ok(record) => {
            return Ok(record.token);
        }
        Err(_) => {
        }
    }
    let token = GUID::rand();  
    let insert = sqlx::query!(
        r#"
        INSERT INTO sessions (user, token)
        VALUES(?, ?)
        "#,
        id,
        token.to_string()
    ).execute(pool.get_ref())
    .await;
    match insert 
    {
        Ok(_) => {
            Ok(token.to_string())
        }
        Err(e) => {
            log::error!("Unable to create token {:?}", e);
            Err(e)
        }
    }
}

pub async fn check_session_token(token:&str, pool: &web::Data<MySqlPool>) -> Result<String, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT user FROM sessions 
        WHERE token = ?
        AND created > DATE_SUB(NOW(), INTERVAL 1 HOUR)
        "#,
        token
    ).fetch_one(pool.get_ref())
    .await;

    match result
    {
        Ok(user) => {
            Ok(user.user)
        }
        Err(e) => {
            Err(e)
        }
    }
}

pub async fn register_new_user(form: &web::Json<UserData>, pool: &web::Data<MySqlPool>, pwdhash: &str) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (email, name, password, account_type, location)
        VALUES(?, ?, ?, ?, ?)
        "#,
        form.email,
        form.name,
        pwdhash,
        form.account_type,
        form.location
    ).execute(pool.get_ref())
    .await
}

pub async fn add_embed_url_to_user(user: &str, url: &web::Json<AddUrl>, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users SET embed_url = ? 
        WHERE users.id = ?
        "#,
        url.url,
        user
    ).execute(pool.get_ref())
    .await
}

pub async fn delete_embed_url_from_user(user: &str, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users SET embed_url = NULL 
        WHERE users.id = ?
        "#,
        user
    ).execute(pool.get_ref())
    .await
}

pub async fn add_avatar_url_to_user(user: &str, url: &web::Json<AddUrl>, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users SET avatar_url = ? 
        WHERE users.id = ?
        "#,
        url.url,
        user
    ).execute(pool.get_ref())
    .await
}

pub async fn delete_avatar_url_from_user(user: &str, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users SET avatar_url = NULL 
        WHERE users.id = ?
        "#,
        user
    ).execute(pool.get_ref())
    .await
}
pub async fn add_image_url_to_user(user: &str, url: &web::Json<AddUrl>, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users SET image_url = ?
        WHERE users.id = ?
        "#,
        url.url,
        user
    ).execute(pool.get_ref())
    .await
}

pub async fn delete_image_url_from_user(user: &str, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error>{
    sqlx::query!(
        r#"
        UPDATE users SET image_url = NULL
        WHERE users.id = ?
        "#,
        user
    ).execute(pool.get_ref())
    .await
}

pub async fn update_profile(user: &str, profile: &web::Json<UpdateProfileData>, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error>{
    sqlx::query!(
        r#"
        UPDATE users SET name = ?, email = ?, location = ?, embed_url = ?, image_url = ?, avatar_url = ?
        WHERE users.id = ?
        "#,
        profile.name,
        profile.email,
        profile.location,
        profile.embed_url,
        profile.image_url,
        profile.avatar_url,
        user
    ).execute(pool.get_ref())
    .await
}

pub async fn change_password_for_user(user: &str, password: &web::Json<PasswordUpdate>, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error>{
    let password_hash = match hash(&password.password,bcrypt::DEFAULT_COST)
    {
        Ok(hashed_password)=> {
            hashed_password
        }
        Err(_e) => {
            log::error!("Failed to encrypt password");
            "".to_string()
        }
    };
    sqlx::query!(
        r#"
        UPDATE users SET password = ?
        WHERE users.id = ?
        "#,
        password_hash,
        user
    ).execute(pool.get_ref())
    .await
}
pub async fn get_user(user: &str, pool: &web::Data<MySqlPool>) -> Result<LoginData, sqlx::Error>{
    let user = sqlx::query_as!(LoginData,
        r#"
        SELECT id, email, password 
        FROM users 
        WHERE users.id = ?
        LIMIT 1
        "#,
        user
    ).fetch_one(pool.get_ref())
    .await;
    user
}

pub async fn delete_user_account(user: &str, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error>{
    sqlx::query!(
        r#"
        DELETE FROM user_shows WHERE user_id = ?
        "#,
        user
    ).execute(pool.get_ref())
    .await?;
    sqlx::query!(
        r#"
        DELETE FROM user_genres WHERE user_id = ?
        "#,
        user
    ).execute(pool.get_ref())
    .await?;
    sqlx::query!(
        r#"
        DELETE FROM users WHERE users.id = ?
        "#,
        user
    ).execute(pool.get_ref())
    .await
}

