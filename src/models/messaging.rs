use actix_web::web;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;

#[derive(Deserialize, Serialize, Debug)]
pub struct Contact {
    id: String,
    image_url: Option<String>,
    avatar_url: Option<String>,
    name: String,
    blocked: Option<i8>
}


pub async fn fetch_contacts(user: &str, pool: &web::Data<MySqlPool>) -> Result<Vec<Contact>, sqlx::Error>{
    sqlx::query_as!(Contact,
        r#"
            SELECT 
                users.id, 
                users.name, 
                users.avatar_url, 
                users.image_url, 
                blocked 
            FROM 
                contacts 
            JOIN users ON contact_id = users.id
            WHERE contacts.user_id = ?
        "#,
        user
    ).fetch_all(pool.get_ref())
    .await
}
