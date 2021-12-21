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
                        HttpResponse::Ok().finish()
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
