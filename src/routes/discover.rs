use crate::models::search::*;
use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;

pub async fn discover_index() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn discover_search(form: web::Json<Search>, pool: web::Data<MySqlPool>) -> HttpResponse {
    match do_search(&form, &pool).await {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(_) => HttpResponse::Ok().json("Unable to obtain shows"),
    }
}
