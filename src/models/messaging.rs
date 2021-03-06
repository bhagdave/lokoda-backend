use actix_web::web;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use sqlx::mysql::{MySqlQueryResult};
use sqlx::types::chrono::NaiveDateTime;
use crate::models::users::*;
use guid_create::GUID;


use futures::stream::StreamExt;
use futures::stream::futures_unordered::FuturesUnordered;

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
    last_message_date: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    id : String,
    user_id : String,
    message: String,
    created_at: NaiveDateTime,
    created_time: Option<String>,
    created_date: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewMessage {
    group_id : String,
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewGroup {
    name : String,
    users: Vec<String>,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub async fn get_groups(user: &str, pool: &web::Data<MySqlPool>) -> Result<Vec<Grouped>, sqlx::Error>{
    sqlx::query_as!(Grouped,
        r#"
            SELECT 
                id, name, message as last_message,DATE_FORMAT(last_message, "%M %d") as last_message_date
            FROM 
                `groups` 
                JOIN user_groups ON 
                    user_groups.group_id =`groups`.id 
                    AND user_id = ? 
                LEFT JOIN 
                    (SELECT group_id, message FROM messages LIMIT 1 ) x 
                    ON x.group_id = user_groups.group_id
            ORDER BY `groups`.last_message desc
        "#,
        user
    ).fetch_all(pool.get_ref())
    .await
}

pub async fn new_message(user: &str, new_message: &web::Json<NewMessage>, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error>{
    let group = Group::get_group(&new_message.group_id, pool).await;
    group.add_new_message(&user, &new_message.message, pool).await
}

pub async fn get_users_groups(user: &str, pool: &web::Data<MySqlPool>) -> Result<Vec<Group>, sqlx::Error>{
    let mut rows = sqlx::query!(
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
    .await?;
    rows.iter_mut().map(|row| 
        row.fetch_last_message(&pool)
    )
    .collect::<FuturesUnordered<_>>()
    .collect::<Vec<_>>()
    .await;
    rows.iter_mut().map(|row| 
        row.get_users(&pool)
    )
    .collect::<FuturesUnordered<_>>()
    .collect::<Vec<_>>()
    .await;
    Ok(rows)
}

pub async fn get_group(group_id: &str, pool: &web::Data<MySqlPool>) -> Result<Group, sqlx::Error>{
    let mut group = Group::get_group(group_id, pool).await;
    group.fetch_messages(pool).await;
    group.get_users(&pool).await;
    Ok(group)
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
            ORDER BY users.name
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

pub async fn delete_contact(user_id: &str, contact_id :&str, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM contacts 
        WHERE user_id = ? AND contact_id = ?
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

pub async fn create_group(user: &str, new_group: web::Json<NewGroup>, pool: &web::Data<MySqlPool>) -> Result<Group, sqlx::Error> {
    let mut group = Group::new_group(&new_group.name, &pool).await;
    group.add_new_user(&user, &pool).await?;
    for user_id in &new_group.users {
        group.add_new_user(&user_id, &pool).await?;
    }
    group.get_users(&pool).await;

    Ok(group)
}

pub async fn delete_group(group: &str, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM `groups` 
        WHERE id = ?
        "#,
        group,
    ).execute(pool.get_ref())
    .await
}


impl Group { 
    pub async fn get_group(group_id : &str, pool: &web::Data<MySqlPool>) -> Self {
        let group = sqlx::query!("SELECT name FROM `groups` WHERE id = ?", group_id)
            .fetch_one(pool.get_ref())
            .await;
        match group {
            Ok(group) => {
                Self {id : group_id.to_string(), name: group.name, messages:None, users:None}
            }
            Err(_) => {
                Self {id : group_id.to_string(), name: "NOTFOUND".to_string(), messages:None, users:None}
            }
        }
    }

    pub async fn new_group(name : &str, pool:&web::Data<MySqlPool>) -> Self {
        let guid = GUID::rand();
        sqlx::query!(
            r#"
            INSERT INTO `groups` (id, name)
            VALUES(?, ?)
            "#,
            guid.to_string(),
            name,
        ).execute(pool.get_ref())
        .await.unwrap();
        log::info!("Group id is {}", guid);
        Self {id : guid.to_string(), name: name.to_string(), messages:None, users:None}
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
        .await?;
        sqlx::query!(
            r#"
            UPDATE `groups` SET last_message = now()
            WHERE id = ?
            "#,
            self.id
        ).execute(pool.get_ref())
        .await
    }

    pub async fn add_new_user(&self, user_id: &str, pool: &web::Data<MySqlPool>) -> Result<MySqlQueryResult, sqlx::Error>{
        sqlx::query!(
            r#"
            INSERT INTO `user_groups` (group_id, user_id)
            VALUES(?, ?)
            "#,
            self.id, 
            user_id, 
        ).execute(pool.get_ref())
        .await
    }

    pub async fn fetch_messages(&mut self, pool: &web::Data<MySqlPool>){
        match sqlx::query_as!(Message,
            r#"
                SELECT id,user_id,message,created_at, 
                    DATE_FORMAT(created_at, "%H:%i") AS created_time, DATE_FORMAT(created_at, "%D %b") AS created_date
                FROM messages
                WHERE
                    group_id = ?
                ORDER BY created_at desc
            "#,
            self.id,
        ).fetch_all(pool.get_ref())
        .await
        {
            Ok(messages) => {
                self.messages = Some(messages);
            }
            Err(_) => {
                self.messages = None;
            }
        }
    }

    pub async fn fetch_last_message(&mut self, pool: &web::Data<MySqlPool>){
        match sqlx::query_as!(Message,
            r#"
                SELECT id,user_id,message,created_at, 
                    DATE_FORMAT(created_at, "%H:%i") AS created_time, DATE_FORMAT(created_at, "%D %b") AS created_date
                FROM messages
                WHERE
                    group_id = ?
                ORDER BY created_at desc
                LIMIT 1
            "#,
            self.id,
        ).fetch_all(pool.get_ref())
        .await
        {
            Ok(messages) => {
                self.messages = Some(messages);
            }
            Err(_) => {
                self.messages = None;
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
