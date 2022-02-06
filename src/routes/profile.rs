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
