use actix_web::web;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use sqlx::Row;
use sqlx::mysql::{MySqlQueryResult, MySqlRow};
use sqlx::types::chrono::NaiveDateTime;
use crate::models::users::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Contact {
    id: String,
    image_url: Option<String>,
    avatar_url: Option<String>,
    name: String,
    blocked: Option<i8>
}

#[derive(Deserialize, Serialize)]
pub struct Group {
    id: String,
    name: String,
    messages: Option<Vec<Message>>,
    users: Option<Vec<ProfileData>>,
}

#[derive(Deserialize, Serialize)]
pub struct Grouped {
    id: String,
    name: String,
    last_message: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    id : String,
    user_id : String,
    message: String,
    created_at: NaiveDateTime,
}



pub async fn get_users_groups(user: &str, pool: &web::Data<MySqlPool>) -> Result<Vec<Group>, sqlx::Error>{
    sqlx::query!(
        r#"
            SELECT 
                groups.id, 
                groups.name
            FROM 
                `groups` 
            JOIN user_groups ON group_id = groups.id AND user_id = ?
        "#,
        user
    )
    .map(|row| (Group { id: row.id, name: row.name, messages: None, users: None }))
    .fetch_all(pool.get_ref())
    .await
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

pub async fn add_contact(user_id: &str, contact_id :&str, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO contacts (user_id, contact_id, blocked)
        VALUES(?, ?, 0)
        "#,
        user_id,
        contact_id,
    ).execute(pool.get_ref())
    .await
}

pub async fn block_contact(user_id: &str, contact_id :&str, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE contacts SET blocked = 1
        WHERE user_id = ? and contact_id = ?
        "#,
        user_id,
        contact_id,
    ).execute(pool.get_ref())
    .await
}

pub async fn create_group(name: &str, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO `groups` (name)
        VALUES(?)
        "#,
        name,
    ).execute(pool.get_ref())
    .await
}

impl Group { 
    pub async fn get_group(group_id : &str, pool: &web::Data<MySqlPool>) -> Self {
        let group = sqlx::query!("SELECT name FROM `groups` WHERE id = ?", group_id)
            .fetch_one(pool.get_ref())
            .await;
        Self {id : group_id.to_string(), name: group.unwrap().name, messages:None, users:None}
    }

    pub async fn add_new_message(self, user_id: &str, message: &str, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error>{
        sqlx::query!(
            r#"
            INSERT INTO `messages` (group_id, user_id, message, created_at)
            VALUES(?, ?, ?, NOW())
            "#,
            self.id, 
            user_id, 
            message,
        ).execute(pool.get_ref())
        .await
    }

    pub async fn fetch_messages(&mut self, pool: &web::Data<MySqlPool>){
        match sqlx::query_as!(Message,
            r#"
                SELECT id,user_id,message,created_at
                FROM messages
                WHERE
                    group_id = ?
                ORDER BY created_at desc
            "#,
            self.id,
        ).fetch_all(pool.get_ref())
        .await {
            Ok(messages) => {
                    self.messages = Some(messages);
            }
            Err(_) => {
                self.messages = None
            }
        }
    }

    pub async fn get_users(&mut self, pool: &web::Data<MySqlPool>){
        match sqlx::query_as!(ProfileData,
            r#"
                SELECT users.id, users.name, users.email, users.account_type, users.location, users.embed_url, users.image_url, users.avatar_url
                FROM users
                JOIN user_groups ON user_groups.user_id = users.id AND group_id = ?
            "#,
            self.id,
        ).fetch_all(pool.get_ref())
        .await {
            Ok(users) => {
                    self.users = Some(users);
            }
            Err(_) => {
                self.users = None
            }
        }
    }
}
