use sqlx::MySqlPool;
use uuid::Uuid;
use actix_web::{web, HttpResponse};
use bcrypt::*;

#[derive(serde::Deserialize)]
pub struct UserData {
    email: String,
    name: String,
    password: String,
    account_type: String,
    location: String,
}

pub async fn register(form: web::Form<UserData>, pool: web::Data<MySqlPool>,) -> HttpResponse{
    let password_hash = match hash(&form.password,bcrypt::DEFAULT_COST)
    {
        Ok(hashed_password)=> {
            hashed_password
        }
        Err(_e) => {
            log::error!("Failed to encrypt password");
            "".to_string()
        }
    };
    let insert = sqlx::query!(
        r#"
        INSERT INTO users (id, email, name, password, account_type, location)
        VALUES(?, ?, ?, ?, ?, ?)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        password_hash,
        form.account_type,
        form.location
    ).execute(pool.get_ref())
    .await;
    match insert
    {
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
