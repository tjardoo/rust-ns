use crate::state::AppState;
use crate::{database::departures::*, errors::RustNSError};
use actix_web::{web, HttpResponse};

pub async fn get_departures(app_state: web::Data<AppState>) -> Result<HttpResponse, RustNSError> {
    db_get_departures(&app_state.pool)
        .await
        .map(|departures| HttpResponse::Ok().json(departures))
}
