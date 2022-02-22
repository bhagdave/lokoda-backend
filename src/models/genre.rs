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

#[derive(Deserialize, Serialize, Debug)]
pub struct UserGenreList {
    genre: String,
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

pub async fn add_genre_to_user(userid: &str, genre: i32, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error>{
    let insert = sqlx::query!(
        r#"
        INSERT INTO user_genres (genre_id, user_id)
        VALUES(?, ?)
        "#,
        genre,
        userid,
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

pub async fn get_user_genre_list(userid: &str, pool: &web::Data<MySqlPool>) -> Result<Vec<UserGenreList>, sqlx::Error>{
    sqlx::query_as!(UserGenreList,
        r#"
            SELECT genre
            FROM genres, user_genres
            WHERE user_id = ?
            AND genres.id = user_genres.genre_id
        "#,
        userid,
    ).fetch_all(pool.get_ref())
    .await
}