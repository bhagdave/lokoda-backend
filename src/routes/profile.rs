use sqlx::MySqlPool;
use actix_web::{web, HttpResponse};
use actix_session::{Session};
use serde::{Deserialize, Serialize};
use crate::models::users::*;
use crate::models::genre::*;

pub async fn profile_index() -> HttpResponse{
    HttpResponse::Ok().finish()
}
pub async fn profile_update() -> HttpResponse{
    HttpResponse::Ok().finish()
}

pub async fn get_genres(session: Session,pool: web::Data<MySqlPool>)-> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            if userid.is_ok() {
                let genres = get_genre_list(&pool).await;
                match genres {
                    Ok(records) => {
                        HttpResponse::Ok().json(records)
                    }
                    Err(_) => {
                        HttpResponse::Ok().json("No Genres found")
                    }
                }
            } else {
                HttpResponse::Ok().json("not logged_in")
            }
        }
        Ok(None) => {
            log::error!("no token found");
            HttpResponse::Ok().json("no session")
        }
        Err(_) => {
            HttpResponse::Ok().json("Somat went wrong")
        }
    }
}

pub async fn add_genre(session: Session, form: web::Json<UserGenre>, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            if userid.is_ok() {
                let insert = sqlx::query!(
                    r#"
                    INSERT INTO user_genres (genre_id, user_id)
                    VALUES(?, ?)
                    "#,
                    form.genre_id,
                    form.user_id,
                ).execute(pool.get_ref())
                .await;
                match insert {
                    Ok(_) => {
                        HttpResponse::Ok().json("Genre Added")
                    }
                    Err(e) => {
                        log::error!("Failed to execute query: {:?}", e);
                        HttpResponse::InternalServerError().finish()
                    }
                }
            } else {
                HttpResponse::Ok().json("not logged_in")
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

pub async fn get_user_genres(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid 
            {
                Ok(user) => {
                    // Need to know the user id.
                    HttpResponse::Ok().json("logged in")
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
