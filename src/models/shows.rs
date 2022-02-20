use serde::{Serialize, Deserialize};
use actix_web::web;
use sqlx::MySqlPool;
use sqlx::mysql::MySqlQueryResult;

#[derive(Deserialize, Serialize, Debug)]
pub struct Show {
    city: String,
    venue: String,
    day: i32,
    month: i32,
    year: i32
}

pub async fn add_user_show(userid: &str, show: web::Json<Show>, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error> {
    let insert = sqlx::query!(
        r#"
        INSERT INTO user_shows (day,month,year,city,venue,user_id)
        VALUES(?, ?, ?, ?, ?, ?)
        "#,
        show.day,
        show.month,
        show.year,
        show.city,
        show.venue,
        userid
    ).execute(pool.get_ref())
    .await;
    match insert {
        Ok(record) => {
            Ok(record)
        }
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            Err(e)
        }
    }
}

pub async fn get_user_shows(pool: &web::Data<MySqlPool>){
}
