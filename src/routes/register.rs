use sqlx::MySqlPool;
use actix_web::{web, HttpResponse};
use actix_web::http::header::ContentType;
use bcrypt::*;

#[derive(serde::Deserialize)]
pub struct UserData {
    email: String,
    name: String,
    password: String,
    account_type: String,
    location: String,
}

pub async fn register(form: web::Json<UserData>, pool: web::Data<MySqlPool>,) -> HttpResponse{
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
        INSERT INTO users (email, name, password, account_type, location)
        VALUES(?, ?, ?, ?, ?)
        "#,
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
            HttpResponse::Ok().insert_header(ContentType::json()).body("data")
        }
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
