use actix_web::{web, HttpResponse};
use actix_multipart::form::{MultipartForm, tempfile::TempFile,text::Text};
use image::{DynamicImage, ImageFormat};
use std::time::Instant;
#[derive(MultipartForm)]
pub struct ImageUpload {
    image: TempFile,
    scala: Text<f32>,
    ruota: Text<bool>,
    specchia: Text<bool>,
    bw: Text<bool>,
    contrasto: Text<f32>,
    luminosita: Text<f32>,
    file_name: Text<String>
}
pub struct Editings{
    scala: f32,
    ruota: bool,
    specchia: bool,
    bw: bool,
    contrasto: f32,
    luminosita: f32,
    file_name: String
}

pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("..\\static\\index.html"))
}


pub async fn upload(form: MultipartForm<ImageUpload>) -> HttpResponse {
    let filepath = format!("img\\uploaded\\{}", form.0.file_name.as_str());
    println!("Tryng to upload: {:?}...", form.0.file_name);
    println!("scala: {:?}, ruota: {:?},specchia: {:?}, bw: {:?},contrasto: {:?}, luminosita: {:?}", form.0.scala, form.0.ruota, form.0.specchia, form.0.bw, form.0.contrasto, form.0.luminosita );
    match form.0.image.file.persist(filepath) {
        Ok(_) => {
            println!("Image upload done.");
            
            let editings = Editings{
                scala : form.0.scala.0,
                ruota : form.0.ruota.0,
                specchia : form.0.specchia.0,
                bw : form.0.bw.0,
                contrasto : form.0.contrasto.0,
                luminosita : form.0.luminosita.0,
                file_name : form.0.file_name.0
            };
            edit(editings)
        },
        Err(e) => {
            println!("Error: {}", e.to_string());
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub fn edit(e : Editings) -> HttpResponse{
        let filepath = format!("img\\uploaded\\{}", e.file_name);

        let mut now = Instant::now();
        let img: DynamicImage = image::open(filepath).expect("Failed to open image");
        let mut elapsed = now.elapsed();
        println!("Elapsed time for opening: {:.2?}", elapsed);
    
        now = Instant::now();
        let gray_img = img.grayscale();
        elapsed = now.elapsed();
        println!("Elapsed time for editing: {:.2?}", elapsed);

        let modified_filepath = format!("img\\modified\\{}", e.file_name);
        
        now = Instant::now();
        gray_img.save_with_format(&modified_filepath, ImageFormat::Jpeg).expect("Failed to save image");
        elapsed = now.elapsed();
        println!("Elapsed time for saving: {:.2?}", elapsed);

    
        HttpResponse::Ok()
        .content_type("text/plain")
        .body(modified_filepath)
        
}