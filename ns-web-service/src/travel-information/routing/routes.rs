use crate::handlers::departures::*;
use crate::handlers::general::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(welcome_page_handler));
}

pub fn departures(cfg: &mut web::ServiceConfig) {
    cfg.route("/departures", web::get().to(get_departures))
        .route("/departures/fetch", web::get().to(download_departures))
        .route("/departures/{departure_id}", web::get().to(get_departure));
}
