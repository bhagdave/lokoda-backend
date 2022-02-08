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
    log::info!("email required is {}", form.email);
    let user_record = get_login_data(&form.email, &pool).await;
    match user_record
    {
        Ok(record) => {
            log::info!("Found user");
            // check password against hashed password
            match verify(&form.password, &record.password) {
                Ok(verified) => {
                    if verified {
                        let _result = session.insert("logged_in", 1);
                        let _result = session.insert("session", record.id);
                        HttpResponse::Ok().insert_header(ContentType::json()).body("data")
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
                    let update = sqlx::query!(
                            r#"
                            UPDATE users SET password = ?
                            WHERE id = ?
                            "#,
                            password_hash,
                            record.id
                        ).execute(pool.get_ref()).await;
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
