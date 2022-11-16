use actix_web::HttpResponse;

pub async fn error_page_handler() -> HttpResponse {
    HttpResponse::NotFound().json("404 - Page not found")
}
