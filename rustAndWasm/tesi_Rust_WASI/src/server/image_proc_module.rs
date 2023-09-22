use std::{io::Read,time::Instant};
use serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Editings{
    scala: f32,
    ruota: bool,
    specchia: bool,
    bw: bool,
    contrasto: f32,
    luminosita: i32,
    file_path: String,
    modified_file_path: String
}

fn main() {
    let mut serialized_params = String::new();
    std::io::stdin().read_to_string(&mut serialized_params).expect("Failed to read from stdin");
    let editings : Editings = serde_json::from_str(&serialized_params).expect("Deserialization error");
    println!("[WASI] Deserialized editings [{:?}]", editings);

    let mut img;
    let mut now = Instant::now();
    {
        img = image::open(editings.file_path).expect("Failed to open image");    
    }
    let elapsed_for_opening = now.elapsed();

    now = Instant::now();
    {
        if editings.scala != 0.0 {
            let new_width = (img.width() as f32) * editings.scala;
            let new_heigth = (img.height() as f32) * editings.scala;
            img = img.resize(new_width as u32, new_heigth as u32, image::imageops::FilterType::Nearest);
        }
        if editings.ruota {
            img = img.rotate90();
        }
        if editings.specchia {
            img = img.fliph();
        }
        if editings.bw {
            img = img.grayscale();
        }
        if editings.contrasto != 0.0 {
            img = img.adjust_contrast(editings.contrasto)
        }
        if editings.luminosita != 0 {
            img = img.brighten(editings.luminosita)
        }
    }
    let elapsed_for_editing = now.elapsed();

    now = Instant::now();
    {
        img.save(editings.modified_file_path).expect("Failed to save image");
    }
    let elapsed_for_saving = now.elapsed();
    
    println!("[WASI] Elapsed time for:\n\t-opening: {:.2?}\n\t-editing: {:.2?}\n\t-saving: {:.2?}", elapsed_for_opening, elapsed_for_editing, elapsed_for_saving);
}
