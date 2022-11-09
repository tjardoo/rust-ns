use actix_web::HttpResponse;

pub async fn welcome_page_handler() -> HttpResponse {
    HttpResponse::Ok().json("Welcome to the app")
}
