use actix_web::{web, HttpResponse, Result};
use actix_multipart::Multipart;
use futures_util::stream::StreamExt;
use std::io::Write;
use tempfile::NamedTempFile; // Requires "tempfile" crate

pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("..\\client\\index.html"))
}

pub async fn upload(mut payload: Multipart) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn edit(info: web::Json<EditInfo>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub struct EditInfo {
}
