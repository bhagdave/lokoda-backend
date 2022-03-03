use sqlx::MySqlPool;
use actix_web::{web, HttpResponse};
use actix_session::{Session};
use crate::models::users::*;
use crate::models::genre::*;
use crate::models::shows::*;

pub async fn profile_index() -> HttpResponse{
    HttpResponse::Ok().finish()
}
pub async fn profile_update() -> HttpResponse{
    HttpResponse::Ok().finish()
}

pub async fn get_genres(session: Session,pool: web::Data<MySqlPool>)-> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            if userid.is_ok() {
                let genres = get_genre_list(&pool).await;
                match genres {
                    Ok(records) => {
                        HttpResponse::Ok().json(records)
                    }
                    Err(_) => {
                        HttpResponse::Ok().json("No Genres found")
                    }
                }
            } else {
                HttpResponse::Ok().json("not logged_in")
            }
        }
        Ok(None) => {
            log::error!("no token found");
            HttpResponse::Ok().json("no session")
        }
        Err(_) => {
            HttpResponse::Ok().json("Somat went wrong")
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

pub async fn get_genres_for_profile(session: Session, profile: web::Json<Profile>, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid 
            {
                Ok(_) => {
                    // Need to know the user id.
                    match get_user_genre_list(&profile.id, &pool).await
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

pub async fn get_shows_for_profile(session: Session, profile: web::Json<Profile>, pool: web::Data<MySqlPool>) -> HttpResponse{
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid 
            {
                Ok(_) => {
                    // Need to know the user id.
                    match get_user_shows(&profile.id, &pool).await
                    {
                        Ok(records) => {
                            HttpResponse::Ok().json(records)
                        }
                        Err(_) => {
                            HttpResponse::Ok().json("Unable to obtain shows")
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
