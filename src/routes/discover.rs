use actix_web::HttpResponse;

pub async fn discover_index() -> HttpResponse{
    HttpResponse::Ok().finish()
}

pub async fn discover_search() -> HttpResponse{
    HttpResponse::Ok().finish()
}
