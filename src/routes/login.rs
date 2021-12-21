use sqlx::MySqlPool;
use actix_web::{web, HttpResponse};
use bcrypt::*;

#[derive(serde::Deserialize)]
pub struct LoginData {
    email: String,
    password: String,
}

pub async fn login(form: web::Form<LoginData>, pool: web::Data<MySqlPool>,) -> HttpResponse{
    log::info!("Getting to the Login function");
    // get user from database table
    // check password against hashed password
}
