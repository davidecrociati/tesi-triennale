use actix_web::{web, HttpResponse};
use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use std::path::PathBuf;
use std::env;

#[derive(MultipartForm)]
pub struct ImageUpload {
    image: TempFile
}

#[derive(serde::Deserialize)]
pub struct FormData {
    scala: f32,
    ruota: bool,
    specchia: bool,
    bw: bool,
    contrasto: f32,
    luminosita: f32
}

pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("..\\static\\index.html"))
}


pub async fn upload(form: MultipartForm<ImageUpload>) -> HttpResponse {
    let file_name: &str = form
        .0
        .image
        .file_name
        .as_ref()
        .map(|m| m.as_ref())
        .unwrap_or("null");
    
    let filepath = format!("img\\uploaded\\{}", file_name);
    println!("Tryng to upload: {}...", file_name);

    match form.0.image.file.persist(filepath) {
        Ok(_) => {
            println!("Image upload done.");
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            println!("Error: {}", e.to_string());
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn edit(info: web::Form<FormData>) -> HttpResponse {
    println!("scala:{}, contrasto:{}, luminosita:{}",info.scala, info.contrasto, info.luminosita);
    println!("ruota:{}, specchia:{}, bw:{}",info.ruota, info.specchia, info.bw);
    HttpResponse::Ok().finish()
}

