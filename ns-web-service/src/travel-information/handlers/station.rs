use crate::state::AppState;
use crate::{database::station::*, errors::RustNSError};
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct QueryFilter {
    pub limit: Option<u32>,
}

pub async fn get_departures_by_station(
    app_state: web::Data<AppState>,
    params: web::Path<String>,
    query: web::Query<QueryFilter>,
) -> Result<HttpResponse, RustNSError> {
    let station_code = params.into_inner();
    let query_filter = query.into_inner();

    let limit = match query_filter.limit {
        None => 10,
        Some(limit) => limit,
    };

    db_get_departures_by_station(&app_state.pool, station_code, limit)
        .await
        .map(|departures| HttpResponse::Ok().json(departures))
}
