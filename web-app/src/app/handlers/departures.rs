use actix_web::{web::Path, HttpResponse};
use awc::Client;
use serde_json::Value;
use std::error::Error;

pub async fn departure_overview() -> Result<HttpResponse, Box<dyn Error>> {
    let awc_client = Client::default();

    let response = awc_client
        .get("http://localhost:7878/departures/")
        .send()
        .await
        .unwrap()
        .body()
        .await?;

    let departures: Value = serde_json::from_str(&std::str::from_utf8(&response)?)?;

    println!("{:#?}", departures);

    Ok(HttpResponse::Ok().json(departures))
}

pub async fn departure_show(path: Path<i32>) -> Result<HttpResponse, Box<dyn Error>> {
    let departure_id: i32 = path.into_inner();

    let awc_client = Client::default();

    let url = format!("http://localhost:7878/departures/{}", departure_id);

    let response = awc_client
        .get(url)
        .send()
        .await
        .unwrap()
        .body()
        .await
        .unwrap();

    let departures: Value = serde_json::from_str(&std::str::from_utf8(&response)?)?;

    println!("{:#?}", departures);

    Ok(HttpResponse::Ok().json(departures))
}
