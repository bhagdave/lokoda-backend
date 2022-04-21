use sqlx::MySqlPool;
use actix_web::{web, HttpResponse};
use actix_session::{Session};
use crate::models::users::*;
use crate::models::genre::*;
use crate::models::shows::*;

pub async fn profile_index(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    match get_profile_data(&user, &pool).await {
                        Ok(profile) => {
                            HttpResponse::Ok().json(profile)
                        }
                        Err(e) => {
                            log::error!("Whoops: {:?}", e);
                            HttpResponse::Ok().json("Error")
                        }
                    }
                }
                Err(e) => {
                    log::error!("Got user from check_session_token : {:?}", e);
                    HttpResponse::Ok().json("not logged_in but have a cookie?")
                }
            }
        }
        Ok(None) => {
            log::info!("Was not able to get tk from session cookie");
            HttpResponse::Ok().json("No Session")
        }
        Err(e) => {
            log::error!("Whoops: {:?}", e);
            HttpResponse::Ok().json("Error")
        }
    }
}

pub async fn get_profile(user_id: web::Path<String>, pool: web::Data<MySqlPool>) -> HttpResponse{
    match get_profile_data(&user_id, &pool).await {
        Ok(profile) => {
            HttpResponse::Ok().json(profile)
        }
        Err(e) => {
            log::error!("Whoops: {:?}", e);
            HttpResponse::Ok().json("Error")
        }
    }
}

pub async fn profile_update(session: Session, profile: web::Json<UpdateProfileData>, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    let update = update_profile(&user, &profile, &pool).await; 
                    match update {
                        Ok(_) => {
                            HttpResponse::Ok().json("Profile Updated")
                        }
                        Err(e) => {
                            log::error!("Failed to execute query: {:?}", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                }
                Err(_) => {
                    HttpResponse::Ok().json("not logged_in")
                }
            }
        }
        Ok(None) => {
            HttpResponse::Ok().json("No Session")
        }
        Err(_) => {
            HttpResponse::Ok().json("Error")
        }
    }
}

pub async fn get_genres(pool: web::Data<MySqlPool>)-> HttpResponse{
    let genres = get_genre_list(&pool).await;
    match genres {
        Ok(records) => {
            HttpResponse::Ok().json(records)
        }
        Err(_) => {
            HttpResponse::Ok().json("No Genres found")
        }
    }
}
pub async fn add_genre(session: Session, form: web::Json<UserGenre>, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    let insert = add_genre_to_user(&user, form.genre_id, &pool).await; 
                    match insert {
                        Ok(_) => {
                            HttpResponse::Ok().json("Genre Added")
                        }
                        Err(e) => {
                            log::error!("Failed to execute query: {:?}", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                }
                Err(_) => {
                    HttpResponse::Ok().json("not logged_in")
                }
            }
        }
        Ok(None) => {
            HttpResponse::Ok().json("No Session")
        }
        Err(_) => {
            HttpResponse::Ok().json("Error")
        }
    }
}
pub async fn delete_genre(session: Session, form: web::Json<UserGenre>, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    let delete = delete_genre_from_user(&user, form.genre_id, &pool).await; 
                    match delete {
                        Ok(_) => {
                            HttpResponse::Ok().json("Genre Removed")
                        }
                        Err(e) => {
                            log::error!("Failed to execute query: {:?}", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                }
                Err(_) => {
                    HttpResponse::Ok().json("not logged_in")
                }
            }
        }
        Ok(None) => {
            HttpResponse::Ok().json("No Session")
        }
        Err(_) => {
            HttpResponse::Ok().json("Error")
        }
    }
}
pub async fn get_user_genres(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid 
            {
                Ok(user) => {
                    // Need to know the user id.
                    match get_user_genre_list(&user, &pool).await
                    {
                        Ok(records) => {
                            HttpResponse::Ok().json(records)
                        }
                        Err(_) => {
                            HttpResponse::Ok().json("Unable to obtain genres")
                        }
                    }
                }
                Err(_) => {
                    HttpResponse::Ok().json("not logged_in")
                }
            }
        }
        Ok(None) => {
            HttpResponse::Ok().json("No Session")
        }
        Err(_) => {
            HttpResponse::Ok().json("Error")
        }
    }
}
pub async fn get_genres_for_profile(user_id: web::Path<String>, pool: web::Data<MySqlPool>) -> HttpResponse{
    // Need to know the user id.
    match get_user_genre_list(&user_id, &pool).await
    {
        Ok(records) => {
            HttpResponse::Ok().json(records)
        }
        Err(_) => {
            HttpResponse::Ok().json("Unable to obtain genres")
        }
    }
}

pub async fn get_shows_for_profile(user_id: web::Path<String>, pool: web::Data<MySqlPool>) -> HttpResponse{
    // Need to know the user id.
    match crate::models::shows::get_user_shows(&user_id, &pool).await
    {
        Ok(records) => {
            HttpResponse::Ok().json(records)
        }
        Err(_) => {
            HttpResponse::Ok().json("Unable to obtain shows")
        }
    }
}
pub async fn add_show(session: Session, add_show: web::Json<Show>, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid 
            {
                Ok(user) => {
                    match add_user_show(&user, add_show, &pool).await
                    {
                        Ok(_) => {
                            HttpResponse::Ok().json("Show added")
                        }
                        Err(_) => {
                            HttpResponse::Ok().json("Unable to add show")
                        }
                    }
                }
                Err(_) => {
                    HttpResponse::Ok().json("not logged_in")
                }
            }
        }
        Ok(None) => {
            HttpResponse::Ok().json("No Session")
        }
        Err(_) => {
            HttpResponse::Ok().json("Error")
        }
    }
}

pub async fn cancel_user_show(session: Session, show_id: web::Path<String>, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid 
            {
                Ok(user) => {
                    match cancel_show(&show_id, &user, &pool).await
                    {
                        Ok(_) => {
                            HttpResponse::Ok().json("Show cancelled")
                        }
                        Err(_) => {
                            HttpResponse::Ok().json("Unable to cancel show")
                        }
                    }
                }
                Err(_) => {
                    HttpResponse::Ok().json("not logged_in")
                }
            }
        }
        Ok(None) => {
            HttpResponse::Ok().json("No Session")
        }
        Err(_) => {
            HttpResponse::Ok().json("Error")
        }
    }
}

pub async fn update_show(session: Session, update_show: web::Json<Show>, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid 
            {
                Ok(user) => {
                    match update_user_show(&user, update_show, &pool).await
                    {
                        Ok(_) => {
                            HttpResponse::Ok().json("Show updated")
                        }
                        Err(_) => {
                            HttpResponse::Ok().json("Unable to updated show")
                        }
                    }
                }
                Err(_) => {
                    HttpResponse::Ok().json("not logged_in")
                }
            }
        }
        Ok(None) => {
            HttpResponse::Ok().json("No Session")
        }
        Err(_) => {
            HttpResponse::Ok().json("Error")
        }
    }
}

pub async fn add_embed_url(session: Session, add_url: web::Json<AddUrl>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid 
            {
                Ok(user) => {
                    match add_embed_url_to_user(&user, &add_url, &pool).await
                    {
                        Ok(_) => {
                            HttpResponse::Ok().json("Url embedded")
                        }
                        Err(_) => {
                            HttpResponse::Ok().json("Unable to embed url")
                        }
                    }
                }
                Err(_) => {
                    HttpResponse::Ok().json("not logged_in")
                }
            }
        }
        Ok(None) => {
            HttpResponse::Ok().json("No Session")
        }
        Err(_) => {
            HttpResponse::Ok().json("Error")
        }
    }
}
pub async fn delete_embed_url(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid 
            {
                Ok(user) => {
                    match delete_embed_url_from_user(&user, &pool).await
                    {
                        Ok(_) => {
                            HttpResponse::Ok().json("Url unembedded")
                        }
                        Err(_) => {
                            HttpResponse::Ok().json("Unable to unembed url")
                        }
                    }
                }
                Err(_) => {
                    HttpResponse::Ok().json("not logged_in")
                }
            }
        }
        Ok(None) => {
            HttpResponse::Ok().json("No Session")
        }
        Err(_) => {
            HttpResponse::Ok().json("Error")
        }
    }
}

pub async fn add_image_url(session: Session, add_url: web::Json<AddUrl>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid 
            {
                Ok(user) => {
                    match add_image_url_to_user(&user, &add_url, &pool).await
                    {
                        Ok(_) => {
                            HttpResponse::Ok().json("Image Url added")
                        }
                        Err(_) => {
                            HttpResponse::Ok().json("Unable to add image url")
                        }
                    }
                }
                Err(_) => {
                    HttpResponse::Ok().json("not logged_in")
                }
            }
        }
        Ok(None) => {
            HttpResponse::Ok().json("No Session")
        }
        Err(_) => {
            HttpResponse::Ok().json("Error")
        }
    }
}
pub async fn delete_image_url(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid 
            {
                Ok(user) => {
                    match delete_image_url_from_user(&user, &pool).await
                    {
                        Ok(_) => {
                            HttpResponse::Ok().json("Image Url removed")
                        }
                        Err(_) => {
                            HttpResponse::Ok().json("Unable to remove image url")
                        }
                    }
                }
                Err(_) => {
                    HttpResponse::Ok().json("not logged_in")
                }
            }
        }
        Ok(None) => {
            HttpResponse::Ok().json("No Session")
        }
        Err(_) => {
            HttpResponse::Ok().json("Error")
        }
    }
}

