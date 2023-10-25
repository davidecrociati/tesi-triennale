use std::time::SystemTime;

use actix_web::HttpResponse;
use actix_multipart::form::{MultipartForm, tempfile::TempFile,text::Text};

use anyhow::Error;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::{sync::Dir, WasiCtxBuilder, ambient_authority};
use wasi_common::{pipe::ReadPipe, WasiCtx};

use serde::{Serialize,Deserialize};


#[derive(MultipartForm)]
pub struct ImageUpload {
    image: TempFile,
    scala: Text<f32>,
    ruota: Text<bool>,
    specchia: Text<bool>,
    bw: Text<bool>,
    contrasto: Text<f32>,
    luminosita: Text<i32>,
    file_name: Text<String>
}
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

pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/index.html"))
}


pub async fn upload(form: MultipartForm<ImageUpload>) -> HttpResponse {

    let new_file_name = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let filepath = format!("img/uploaded/{:?}_{}",new_file_name, form.0.file_name.as_str());
    println!("Tryng to upload: {:?}...", form.0.file_name.as_str());
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
                file_path : format!("img/uploaded/{:?}_{}",new_file_name, form.0.file_name.as_str()),
                modified_file_path : format!("img/modified/{:?}_{}",SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(), form.0.file_name.as_str())
            };
            match edit(editings){
                Ok(response) =>{             
                    response
                },
                Err(e) => {
                    eprintln!("Error: {}", e.to_string());
                    HttpResponse::InternalServerError().finish()
                },
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e.to_string());
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub fn edit(e : Editings) -> Result<HttpResponse, Error> {
    let serialized_input = serde_json::to_string(&e).expect("Error serializing editings");
    println!("input for wasi module: {}", serialized_input);
    let stdin = ReadPipe::from(serialized_input);

    let engine = Engine::default();
    
    let mut linker: Linker<WasiCtx> = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).expect("Error adding wasi context to linker");

    let  image_directory = Dir::open_ambient_dir("img", ambient_authority()).expect("Error opening img directory");
    let builder = WasiCtxBuilder::new()
    .stdin(Box::new(stdin.clone()))
    .inherit_stdout()
    .inherit_stderr()
    .preopened_dir(image_directory,"img").expect("Error setting preopened dir");

    let wasi = builder.build();
    
    let module = Module::from_file(&engine, "src/server/image_proc_module.wasm").expect("Error istantiating module from file");
    let mut store = Store::new(&engine, wasi);


    linker.module(&mut store, "", &module).expect("Error linking store to module");


    let instance = linker.instantiate(&mut store, &module).expect("Error istantiating module");
    let instance_main = instance.get_typed_func::<(), ()>(&mut store, "_start").expect("Error finding main function");
    instance_main.call(&mut store, ()).expect("Error calling main function");
    println!("\n\n");
    drop(store);
    
    Ok(HttpResponse::Ok()
    .content_type("text/plain")
    .body(e.modified_file_path))
        
}