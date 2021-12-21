use actix_web::HttpResponse;

pub async fn register() -> HttpResponse{
    HttpResponse::Ok().finish()
}
