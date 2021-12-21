use sqlx::MySqlPool;
use uuid::Uuid;
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct UserData {
    email: String,
    name: String,
    password: String,
}

pub async fn register(form: web::Form<UserData>, pool: web::Data<MySqlPool>,) -> HttpResponse{
    match sqlx::query!(
        r#"
        INSERT INTO users (id, email, name, password)
        VALUES(?, ?, ?, ?)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        form.password
    ).execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(_e) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}
