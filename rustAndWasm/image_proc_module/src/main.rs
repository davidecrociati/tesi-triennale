use std::time::Instant;
use serde::{Serialize,Deserialize};


#[link(wasm_import_module = "host")]
extern "C" {
    fn get_input_size() -> i32;
    fn get_input(ptr: i32);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    //let serialized_params = String::new();
    //std::io::stdin().read_to_string(&mut serialized_params).expect("Failed to read from stdin");
    //println!("Serialiezed parmas: {}", serialized_params);
    //let editings : Editings = serde_json::from_str(&serialized_params).expect("Deserialization error");

    let mem_size = unsafe { get_input_size() };

    let mut buf: Vec<u8> = Vec::with_capacity(mem_size as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(ptr);

    let input_buf = unsafe {
        get_input(ptr as i32);
        Vec::from_raw_parts(ptr, mem_size as usize, mem_size as usize)
    };

    //println!("input_buf = {:?}", input_buf);

    let editings: Editings = serde_json::from_slice(&input_buf).map_err(|e| {
        eprintln!("ser: {e}");
        e
    }).unwrap();
    println!("[WASI] Editings {:?}",editings);
    //println!("[WASI] Deserialized editings [scala: {:?}, ruota: {:?},specchia: {:?}, bw: {:?},contrasto: {:?}, luminosita: {:?}]", editings.scala, editings.ruota, editings.specchia, editings.bw, editings.contrasto, editings.luminosita );

    image::io::Limits::no_limits();
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
            img = img.adjust_contrast(editings.contrasto);
        }
        if editings.luminosita != 0 {
            img = img.brighten(editings.luminosita);
        }
    }
    let elapsed_for_editing = now.elapsed();


    now = Instant::now();
    {
        img.save(&editings.modified_file_path).expect("Failed to save image");
    }
    let elapsed_for_saving = now.elapsed();
    //print!("{}",editings.modified_file_path);
    println!("[WASI] Elapsed time for:\n\t-opening: {:.2?}\n\t-editing: {:.2?}\n\t-saving: {:.2?}", elapsed_for_opening,elapsed_for_editing,elapsed_for_saving);
}
