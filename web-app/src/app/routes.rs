use crate::handlers::departures::*;
use crate::handlers::general::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(welcome_page_handler));
}

pub fn departure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/departures/overview", web::get().to(departure_overview))
        .route("/departures/{departure_id}", web::get().to(departure_show));
}
