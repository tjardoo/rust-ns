use actix_web::HttpResponse;

pub async fn welcome_page_handler() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to the `ns-web-service`")
}
