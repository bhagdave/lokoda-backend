use actix_web::web;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;

#[derive(Deserialize, Serialize, Debug)]
pub struct Search {
    account_type: String,
    location: String,
    name: String,
    genre: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Results {
    account_type: String,
    image_url: String,
    location: String,
    name: String,
    genre: i32,
}

pub async fn do_search(form: &web::Json<Search>, pool: &web::Data<MySqlPool>) -> Result<Vec<Results>, sqlx::Error>{
    sqlx::query_as!(Results,
        r#"
            SELECT account_type,image_url,name, location
            FROM users, user_genres
            WHERE users.id = user_genres.user_id
            JOIN genres on genres.id = user_genres.genre_id
            AND ($1::text IS null OR $1 = users.account_type)
            AND ($2::text IS null OR $2 like users.location)
            AND ($3::text IS null OR $3 like users.name)
            AND ($4::text Is null OR $4 like genres.genre)
        "#,
        form.account_type,
        form.location,
        form.name,
        form.genre,
    ).fetch_all(pool.get_ref())
    .await
}

