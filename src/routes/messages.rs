use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use actix_session::{Session};
use crate::models::*;
use crate::models::users::*;

pub async fn messages() -> HttpResponse{
    HttpResponse::Ok().finish()
}
pub async fn new_message() -> HttpResponse{
    HttpResponse::Ok().finish()
}
pub async fn search_messages() -> HttpResponse{
    HttpResponse::Ok().finish()
}
pub async fn block_contact() -> HttpResponse{
    HttpResponse::Ok().finish()
}
pub async fn delete_contact() -> HttpResponse{
    HttpResponse::Ok().finish()
}
pub async fn get_contacts(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    match fetch_contacts(&user, &pool).await {
                        Ok(contacts) => {
                            HttpResponse::Ok().json(contacts)
                        }
                        Err(e) => {
                            log::error!("Whoops: {:?}", e);
                            HttpResponse::Ok().json("Error")
                        }
                    }
                }
                Err(e) => {
                    log::error!("Got user from check_session_token : {:?}", e);
                    HttpResponse::Ok().json("not logged_in but have a cookie?")
                }
            }
        }
        Ok(None) => {
            log::info!("Was not able to get tk from session cookie");
            HttpResponse::Ok().json("No Session")
        }
        Err(e) => {
            log::error!("Whoops: {:?}", e);
            HttpResponse::Ok().json("Error")
        }
    }
}

pub async fn get_groups(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    match messaging::get_groups(&user, &pool).await {
                        Ok(groups) => {
                            HttpResponse::Ok().json(groups)
                        }
                        Err(e) => {
                            log::error!("Whoops: {:?}", e);
                            HttpResponse::Ok().json("Error")
                        }
                    }
                }
                Err(e) => {
                    log::error!("Got user from check_session_token : {:?}", e);
                    HttpResponse::Ok().json("not logged_in but have a cookie?")
                }
            }
        }
        Ok(None) => {
            log::info!("Was not able to get tk from session cookie");
            HttpResponse::Ok().json("No Session")
        }
        Err(e) => {
            log::error!("Whoops: {:?}", e);
            HttpResponse::Ok().json("Error")
        }
    }
}

pub async fn get_group(group_id: web::Path<String> ,session: Session, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(_user) => {
                    match messaging::get_group(&group_id, &pool).await {
                        Ok(group) => {
                            HttpResponse::Ok().json(group)
                        }
                        Err(e) => {
                            log::error!("Whoops: {:?}", e);
                            HttpResponse::Ok().json("Error")
                        }
                    }
                }
                Err(e) => {
                    log::error!("Got user from check_session_token : {:?}", e);
                    HttpResponse::Ok().json("not logged_in but have a cookie?")
                }
            }
        }
        Ok(None) => {
            log::info!("Was not able to get tk from session cookie");
            HttpResponse::Ok().json("No Session")
        }
        Err(e) => {
            log::error!("Whoops: {:?}", e);
            HttpResponse::Ok().json("Error")
        }
    }
}
