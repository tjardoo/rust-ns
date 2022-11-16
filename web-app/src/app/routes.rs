use crate::handlers::departures::*;
use crate::handlers::general::*;
use actix_web::web;

use super::handlers::platform::show_platform_display;
use super::handlers::station::show_station_display;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(welcome_page_handler));
}

pub fn departure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/departures", web::get().to(get_station_departure_overview))
        .route(
            "/departures/{departure_id}",
            web::get().to(get_station_departure_by_id),
        );
}

pub fn station_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/station", web::get().to(show_station_display));
}

pub fn platform_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/platform", web::get().to(show_platform_display));
}
