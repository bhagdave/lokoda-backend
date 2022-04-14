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
            .route("/discover", web::get().to(discover_index))
            .route("/search", web::post().to(discover_search))
            .route("/profile", web::get().to(profile_index))
            .route("/profile/{user_id}", web::get().to(get_profile))
            .route("/updte", web::post().to(profile_update))
            .route("/get_genres", web::get().to(get_genres))
            .route("/get_user_genres", web::get().to(get_user_genres))
            .route("/get_genres_for_profile/{user_id}", web::get().to(get_genres_for_profile))
            .route("/add_genre", web::post().to(add_genre))
            .route("/delete_genre", web::post().to(delete_genre))
            .route("/get_shows_for_profile/{user_id}", web::get().to(get_shows_for_profile))
            .route("/add_show", web::post().to(add_show))
            .route("/cancel_show", web::get().to(cancel_user_show))
            .route("/get_user_shows", web::get().to(get_user_shows))
            .route("/add_image", web::post().to(add_image_url))
            .route("/delete_image", web::get().to(delete_image_url))
            .route("/embed_url", web::post().to(add_embed_url))
            .route("/unembed_url", web::get().to(delete_embed_url))
            .route("/messages", web::get().to(messages))
            .route("/newmessage", web::post().to(new_message))
            .route("/searchmessage", web::post().to(search_messages))
            .route("/blockcontact", web::post().to(block_contact))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/reset_password", web::post().to(reset_password))
            .service(
                web::resource("/update_password")
                .route(web::post().to(update_password))
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}


