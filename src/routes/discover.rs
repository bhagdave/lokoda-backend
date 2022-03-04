use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use crate::models::search::*;
use crate::models::users::*;
use actix_session::{Session};

pub async fn discover_index() -> HttpResponse{
    HttpResponse::Ok().finish()
}

pub async fn discover_search(session: Session, form: web::Json<Search>, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid 
            {
                Ok(_) => {
                    match do_search(&form, &pool).await {
                        Ok(records) => {
                            HttpResponse::Ok().json(records)
                        }
                        Err(_) => {
                            HttpResponse::Ok().json("Unable to obtain shows")
                        }
                    }
                }
                Err(_) => {
                    HttpResponse::Ok().json("not logged_in")
                }
            }
        }
        Ok(None) => {
            HttpResponse::Ok().json("No Session")
        }
        Err(_) => {
            HttpResponse::Ok().json("Error")
        }
    }
}
