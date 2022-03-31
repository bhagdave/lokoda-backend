//! tests/health_check.rs
use std::net::TcpListener;
use lokoda_backend::startup::run;
use lokoda_backend::configuration::*;
use sqlx::{Connection, Executor, MySqlConnection, MySqlPool};

pub struct TestApp {
    pub address: String,
    pub db_pool: MySqlPool,
}


async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Uanble to bind to address");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = "testing".to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> MySqlPool {
    let mut connection = MySqlConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to MYSQL.");

    connection.execute(r#"CREATE DATABASE testing;"#)
        .await
        .expect("Failed to create database.");
        

    // migrate
    let connection_pool = MySqlPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to MySQl.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}

#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to get health check");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


