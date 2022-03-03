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

pub async fn do_search(form: &web::Json<Search>, pool: &web::Data<MySqlPool>) {
}

