use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use crate::models::search::*;
use actix_session::{Session};

pub async fn discover_index() -> HttpResponse{
    HttpResponse::Ok().finish()
}

pub async fn discover_search(session: Session, form: web::Json<Search>, pool: web::Data<MySqlPool>) -> HttpResponse{
    do_search(&form, &pool);
    HttpResponse::Ok().finish()
}
