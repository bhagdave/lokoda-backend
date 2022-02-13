use sqlx::MySqlPool;
use actix_web::{web, HttpResponse};
use actix_web::http::header::ContentType;
use bcrypt::*;
use crate::models::users::*;


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
    let insert = register_new_user(&form, &pool, &password_hash).await;
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
