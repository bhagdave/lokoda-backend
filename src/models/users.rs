use actix_web::web;
use sqlx::MySqlPool;

#[derive(serde::Deserialize)]
pub struct LoginData {
    pub id: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}


pub async fn get_login_data(form: web::Json<LoginForm>, pool: web::Data<MySqlPool>) -> Result<LoginData, sqlx::Error> {
    // get user from database table
    let user_record = sqlx::query_as!(LoginData,
        r#"
            SELECT id, email, password
            FROM users
            WHERE email = ?
            LIMIT 1
        "#,
        form.email
    )
    .fetch_one(pool.get_ref())
    .await;
    user_record
}
