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
    log::info!("Getting to the register function");
    let insert = sqlx::query!(
        r#"
        INSERT INTO users (id, email, name, password)
        VALUES(?, ?, ?, ?)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        form.password
    ).execute(pool.get_ref())
    .await;
    log::info!("Query executed!");
    match insert
    {
        Ok(_) => {
            log::info!("Worked");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
