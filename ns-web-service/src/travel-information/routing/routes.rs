use crate::handlers::departure::*;
use crate::handlers::download::*;
use crate::handlers::general::*;
use crate::handlers::platform::*;
use crate::handlers::station::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(welcome_page_handler));
}

pub fn departure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/station/{station_code}",
        web::get().to(get_departures_by_station),
    )
    .route(
        "/station/{station_code}/download",
        web::get().to(download_departures_by_station),
    )
    .route(
        "/station/{station_code}/platform/{platform_id}",
        web::get().to(get_departures_by_platform),
    )
    .route(
        "/station/{station_code}/departure/{departure_id}",
        web::get().to(get_departure_by_id),
    );
}
