use crate::routes::*;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, middleware, HttpServer};
use actix_web::middleware::Logger;
use sqlx::MySqlPool;
use std::net::TcpListener;
use actix_session::{CookieSession};


pub fn run(listener: TcpListener, db_pool: MySqlPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*")) // for testing purposes only
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}


