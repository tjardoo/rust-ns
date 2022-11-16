use actix_web::{http::header::ContentType, web, Error, HttpResponse};

use crate::app::errors::WebAppError;

pub async fn show_station_display(template: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();

    ctx.insert("title", "Hello Station!");

    let rendered = template
        .render("station.html", &ctx)
        .map_err(|error| WebAppError::TeraError(error.to_string()))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}
