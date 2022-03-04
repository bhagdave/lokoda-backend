use actix_web::web;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;

#[derive(Deserialize, Serialize, Debug)]
pub struct Search {
    account_type: String,
    location: String,
    name: String,
    genre: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Results {
    account_type: String,
    image_url: Option<String>,
    location: String,
    name: String,
}

pub async fn do_search(form: &web::Json<Search>, pool: &web::Data<MySqlPool>) -> Result<Vec<Results>, sqlx::Error>{
    sqlx::query_as!(Results,
        r#"
            SELECT account_type,image_url,name, location
            FROM users, user_genres
            JOIN genres on genres.id = user_genres.genre_id
            WHERE users.id = user_genres.user_id
            AND (? = users.account_type)
            AND (? like users.location)
            AND (? like users.name)
            AND (? like genres.genre)
        "#,
        form.account_type,
        form.location,
        form.name,
        form.genre,
    ).fetch_all(pool.get_ref())
    .await
}

