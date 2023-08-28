use actix_web::{web, HttpResponse};
use actix_multipart::form::{MultipartForm, tempfile::TempFile,text::Text};
use serde::{Serialize,Deserialize};
use anyhow::{Result, Context};
use std::error::Error;
use wasmtime::*;
use wasmtime_wasi::*;
use wasmtime_wasi::sync::Dir;
use wasi_common::{pipe::ReadPipe, WasiCtx};
use wasi::wasi_snapshot_preview1::*;
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
#[derive(Serialize, Deserialize)]
pub struct Editings{
    scala: f32,
    ruota: bool,
    specchia: bool,
    bw: bool,
    contrasto: f32,
    luminosita: i32,
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
    match form.0.image.file.persist(filepath) {
        Ok(_) => {
            println!("Image upload done.\n");
            
            let editings = Editings{
                scala : form.0.scala.0,
                ruota : form.0.ruota.0,
                specchia : form.0.specchia.0,
                bw : form.0.bw.0,
                contrasto : form.0.contrasto.0,
                luminosita : form.0.luminosita.0,
                file_name : form.0.file_name.0
            };
            println!("recvd editings [scala: {:?}, ruota: {:?},specchia: {:?}, bw: {:?},contrasto: {:?}, luminosita: {:?}]", editings.scala, editings.ruota, editings.specchia, editings.bw, editings.contrasto, editings.luminosita );
            edit(editings)
        },
        Err(e) => {
            println!("Error: {}", e.to_string());
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub fn edit(e : Editings) -> HttpResponse {
    //possibile passare parametri con env var, cmd line args or file
    let serialized_input = serde_json::to_string(&e).unwrap();
    println!("input for wasi: {}", serialized_input);
    let stdin = ReadPipe::from(serialized_input);
    //let stdout = WritePipe::new_in_memory();

    let engine = Engine::default();
    
    let mut linker: Linker<WasiCtx> = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();

    let  image_directory = Dir::open_ambient_dir("img", ambient_authority()).unwrap();
    let builder = WasiCtxBuilder::new()
    .stdin(Box::new(stdin.clone()))
    //.stdout(Box::new(stdout.clone()))
    .inherit_stdout()
    .inherit_stderr()
    .preopened_dir(image_directory,"img").unwrap();

    let wasi = builder.build();
    
    let module = Module::from_file(&engine, "src/server/image_proc_module.wasm").unwrap();
    let mut store = Store::new(&engine, wasi);
    
    println!("\nimports: ");
    let imp = module.imports();
    for (j, val) in imp.into_iter().enumerate() {
        println!("size: {}, mod: {}, name: {}", j, val.module(),val.name());
    }
    println!("\n");
    match linker.module(&mut store, "", &module) {
        Ok(_) => { /* Module loaded successfully */ },
        Err(error) => {
            eprintln!("Error loading module: {}", error);
            /* Handle the error */
        }
    }

    let instance = linker.instantiate(&mut store, &module).unwrap();
    let instance_main = instance.get_typed_func::<(), ()>(&mut store, "_start").unwrap();
    instance_main.call(&mut store, ()).unwrap();
    /*
    linker.get_default(&mut store, "image_proc_module.wasm").unwrap()
        .typed::<(), ()>(&store).unwrap()
        .call(&mut store, ()).unwrap();
    */
    drop(store);
        HttpResponse::Ok()
        .content_type("text/plain")
        .body("1.jpg")

        
}

//https://docs.wasmtime.dev/lang-rust.html
pub fn invoke_wasm_module(e:Editings) -> Result<(), Box<dyn Error>>  {
    //possibile passare parametri con env var, cmd line args or file
    let serialized_input = serde_json::to_string(&e).unwrap();
    println!("input for wasi: {}", serialized_input);
    let stdin = ReadPipe::from(serialized_input);
    //let stdout = WritePipe::new_in_memory();

    let engine = Engine::default();
    let mut linker: Linker<WasiCtx> = Linker::new(&engine);

    let  image_directory = Dir::open_ambient_dir("img", ambient_authority()).unwrap();
    let mut builder = WasiCtxBuilder::new()
    .stdin(Box::new(stdin.clone()))
    //.stdout(Box::new(stdout.clone()))
    .inherit_stdout()
    .inherit_stderr()
    .preopened_dir(image_directory,"img").unwrap();

    let wasi = builder.build();
    let mut store = Store::new(&engine, wasi);
    
    let module = Module::from_file(&engine, "src/server/image_proc_module.wasm")?;
    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), _>(&store)?
        .call(&mut store, ())?;
    
    drop(store);
    /* 
    let contents: Vec<u8> = stdout.try_into_inner()
        .map_err(|_err| anyhow::Error::msg("sole remaining reference"))?
        .into_inner();
    let output: Output = serde_json::from_slice(&contents)?;
*/
    Ok(())

}