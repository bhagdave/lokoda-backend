use actix_web::web;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Deserialize, Serialize, Debug)]
pub struct Search {
    account_type: Option<String>,
    location: Option<String>,
    name: Option<String>,
    genre: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Results {
    id: String,
    account_type: String,
    image_url: Option<String>,
    avatar_url: Option<String>,
    location: String,
    name: String,
    genres: Option<serde_json::Value>,
}

pub async fn do_search(
    form: &web::Json<Search>,
    pool: &web::Data<MySqlPool>,
) -> Result<Vec<Results>, sqlx::Error> {
    sqlx::query_as!(Results,
        r#"
            SELECT id,account_type,image_url,avatar_url,name, location, json_extract(genres, '$') as genres
            FROM vw_discover
            WHERE (? = account_type)
            OR (? like location)
            OR (? like name)
            OR (find_in_set(?, genres))
        "#,
        form.account_type,
        form.location,
        form.name,
        form.genre,
    ).fetch_all(pool.get_ref())
        .await
}
