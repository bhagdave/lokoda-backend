use sqlx::MySqlPool;
use actix_web::{web, HttpResponse};
use actix_session::{Session};
use serde::{Deserialize, Serialize};

pub async fn profile_index() -> HttpResponse{
    HttpResponse::Ok().finish()
}
pub async fn profile_update() -> HttpResponse{
    HttpResponse::Ok().finish()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Genre {
    id: i32,
    genre: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct UserGenre {
    user_id: String,
    genre_id: i32,

}
pub async fn get_genres(session: Session,pool: web::Data<MySqlPool>)-> HttpResponse{
    let logged_in = session.get::<i32>("logged_in");
    match logged_in {
        Ok(Some(x)) => {
            if x == 1 {
                let genres = sqlx::query_as!(Genre,
                    r#"
                        SELECT id, genre
                        FROM genres
                    "#
                )
                .fetch_all(pool.get_ref())
                .await;
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
            HttpResponse::Ok().json("no session")
        }
        Err(_) => {
            HttpResponse::Ok().json("Somat went wrong")
        }
    }
}

pub async fn add_genre(session: Session, form: web::Json<UserGenre>, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<i32>("logged_in");
    let _user_id = session.get::<String>("user_id");
    match logged_in {
        Ok(Some(x)) => {
            if x == 1 {
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

pub async fn get_user_genres(session: Session, _pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<i32>("logged_in");
    match logged_in {
        Ok(Some(x)) => {
            if x == 1 {
                // Need to know the user id.
                HttpResponse::Ok().json("logged in")
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
