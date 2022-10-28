use std::error::Error;

use crate::state::AppState;
use crate::{database::departures::*, errors::RustNSError};
use actix_web::{web, HttpResponse};
use openssl::ssl::{SslConnector, SslVerifyMode};
use std::env;

pub async fn get_departures(app_state: web::Data<AppState>) -> Result<HttpResponse, RustNSError> {
    db_get_departures(&app_state.pool)
        .await
        .map(|departures| HttpResponse::Ok().json(departures))
}

pub async fn download_departures(
    _app_state: web::Data<AppState>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let ssl = {
        let mut ssl = SslConnector::builder(openssl::ssl::SslMethod::tls()).unwrap();
        ssl.set_verify(SslVerifyMode::NONE);
        ssl.build()
    };

    let connector = awc::Connector::new().openssl(ssl);

    let ns_api_key = env::var("NS_API_KEY").expect("NS_API_KEY is not set in the .env file.");

    let mut response = awc::ClientBuilder::new()
        .connector(connector)
        .finish()
        .get("https://gateway.apiportal.ns.nl/reisinformatie-api/api/v2/departures?station=HN")
        .insert_header(("Ocp-Apim-Subscription-Key", ns_api_key))
        .insert_header(("User-Agent", "Actix-web"))
        .send()
        .await?;

    let data = response.body().await?;

    let data = std::str::from_utf8(&data)?;

    println!("{:?}", data);

    Ok(HttpResponse::Ok().json(data))
}
