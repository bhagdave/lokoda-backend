use actix_web::HttpResponse;

pub async fn messages() -> HttpResponse{
    HttpResponse::Ok().finish()
}
pub async fn new_message() -> HttpResponse{
    HttpResponse::Ok().finish()
}
pub async fn search_messages() -> HttpResponse{
    HttpResponse::Ok().finish()
}
pub async fn block_contact() -> HttpResponse{
    HttpResponse::Ok().finish()
}
