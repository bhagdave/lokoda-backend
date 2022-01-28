use crate::emails::send_email;
use sqlx::MySqlPool;
use actix_web::{web, HttpResponse};
use bcrypt::*;
use actix_web::http::header::ContentType;
use actix_session::{Session};

#[derive(serde::Deserialize)]
pub struct LoginData {
    email: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct ResetPassword {
    email: String,
}

#[derive(serde::Deserialize)]
pub struct UpdatePassword {
    hash: String,
    password: String,
}

pub async fn login(session: Session, form: web::Json<LoginData>, pool: web::Data<MySqlPool>,) -> HttpResponse{
    log::info!("Getting to the Login function");
    log::info!("email required is {}", form.email);
    // get user from database table
    let user_record = sqlx::query_as!(LoginData,
        r#"
            SELECT email, password
            FROM users
            WHERE email = ?
            LIMIT 1
        "#,
        form.email
    )
    .fetch_one(pool.get_ref())
    .await;
    match user_record
    {
        Ok(record) => {
            log::info!("Found user");
            // check password against hashed password
            match verify(&form.password, &record.password) {
                Ok(verified) => {
                    if verified {
                        log::info!("Logged in okay");
                        let _session_result =session.insert("logged_in", 1);
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
    let user_record = sqlx::query_as!(LoginData,
        r#"
            SELECT email, password
            FROM users
            WHERE email = ?
            LIMIT 1
        "#,
        form.email
    )
    .fetch_one(pool.get_ref())
    .await;
    match user_record
    {
        Ok(record) => {
            log::info!("Found user");
            // TODO - Send email
            send_email(&record.email, "david.g.h.gill@gmail.com","Password reset request","Hello");  
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Unable to find user {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_password(_form: web::Json<UpdatePassword>, pool: web::Data<MySqlPool>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
