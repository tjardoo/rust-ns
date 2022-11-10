use crate::handlers::departures::*;
use crate::handlers::general::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(welcome_page_handler));
}

pub fn departures(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/station/{station_code}/departures",
        web::get().to(get_departures_by_station_code),
    )
    .route(
        "/station/{station_code}/departures/fetch",
        web::get().to(fetch_departures_by_station_code),
    )
    .route(
        "/station/{station_code}/departures/download",
        web::get().to(download_departures_by_station_code),
    )
    .route(
        "/station/{station_code}/departures/{departure_id}",
        web::get().to(get_departure_by_station_code_and_id),
    );
}
