use serde::{Serialize, Deserialize};
use actix_web::web;
use sqlx::MySqlPool;
use sqlx::mysql::MySqlQueryResult;


#[derive(Deserialize, Serialize, Debug)]
pub struct Genre {
    id: i32,
    genre: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct UserGenre {
    pub user_id: String,
    pub genre_id: i32,

}

pub async fn get_genre_list(pool: &web::Data<MySqlPool>) -> Result<Vec<Genre>, sqlx::Error> {
    sqlx::query_as!(Genre,
        r#"
            SELECT id, genre
            FROM genres
        "#
    )
    .fetch_all(pool.get_ref())
    .await
}

pub async fn add_genre_to_user(pool: &web::Data<MySqlPool>) -> Result<(), sqlx::Error>{
}
