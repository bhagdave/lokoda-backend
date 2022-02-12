use crate::emails::send_email;
use crate::models::users::*;
use sqlx::MySqlPool;
use actix_web::{web, HttpResponse};
use bcrypt::*;
use actix_web::http::header::ContentType;
use actix_session::{Session};
use guid_create::GUID;


pub async fn login(session: Session, form: web::Json<LoginForm>, pool: web::Data<MySqlPool>) -> HttpResponse{
    log::info!("Getting to the Login function");
    let user_record = get_login_data(&form.email, &pool).await;
    match user_record
    {
        Ok(record) => {
            // check password against hashed password
            match verify(&form.password, &record.password) {
                Ok(verified) => {
                    if verified {
                        let token = create_session_token(&record.id, &pool).await;
                        match token 
                        {
                            Ok(token) => {
                                let _result = session.insert("tk",&token);
                                let response = format!("{{'token' : {}}}", token);
                                HttpResponse::Ok().insert_header(ContentType::json()).body(response)
                            }
                            Err(_) => {
                                HttpResponse::Ok().body("Unable to create session token")
                            }
                        }
                    } else {
                        log::error!("Failed to login");
                        HttpResponse::InternalServerError().finish()
                    }
                }
                Err(e) => {
                    log::error!("Failed to login {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            log::error!("Unable to find user {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

}

pub async fn reset_password(form: web::Json<ResetPassword>, pool: web::Data<MySqlPool>) -> HttpResponse {
    log::info!("Password reset request!");
    // get user from database table
    let user_record = get_login_data(&form.email, &pool).await;
    match user_record
    {
        Ok(record) => {
            let guid = GUID::rand();
            log::info!("Found user and creating guid of {}", guid.to_string());
            let update = set_remember_token(&record.email, &pool).await;
            match update
            {
                Ok(_) => {
                    let message = format!("Hello please visit http://lokoda.co.uk/update-password/{}", guid.to_string());
                    send_email(&record.email, "david.g.h.gill@gmail.com","Password reset request",&message);  
                    HttpResponse::Ok().finish()
                }
                Err(e) => {
                    log::error!("Unable to update user {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            log::error!("Unable to find user {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_password(form: web::Json<UpdatePassword>, pool: web::Data<MySqlPool>) -> HttpResponse {
    log::info!("update password request!");
    // get user from database table
    let user_record = get_simple_user(&form, &pool).await;
    match user_record
    {
        Ok(record) => {
            // Hash Password
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
            // Update database
            match password_hash.chars().count()
            {
                0 => {
                    log::error!("Unable to hash password");
                    HttpResponse::InternalServerError().finish()
                }
                _ => {
                    let update = update_user_password(&password_hash, &record.id, &pool).await;
                        match update
                        {
                            Ok(_) => {
                                let message = format!("Hi your password has been changed");
                                send_email(&record.email, "david.g.h.gill@gmail.com","Password reset success",&message);  
                                HttpResponse::Ok().finish()
                            }
                            Err(e) => {
                                log::error!("Unable to update user {:?}", e);
                                HttpResponse::InternalServerError().finish()
                            }
                        }
                }
            }
        }
        Err(e) => {
            log::error!("Unable to find user {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
