use std::time::SystemTime;
use actix_web::{HttpResponse, web};
use actix_multipart::form::{MultipartForm, tempfile::TempFile,text::Text};
use serde::{Serialize,Deserialize};
//use anyhow::Result;
//use std::error::Error;
use wasmtime::*;
use wasmtime_wasi::*;
use wasmtime_wasi::sync::Dir;
use wasi_common::{pipe::{ReadPipe, WritePipe}, WasiCtx};
use crate::common::AppState;
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
pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("..\\static\\index.html"))
}


pub async fn upload(data: web::Data<AppState>, form: MultipartForm<ImageUpload>) -> HttpResponse {
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
            println!("recvd editings [scala: {:?}, ruota: {:?},specchia: {:?}, bw: {:?},contrasto: {:?}, luminosita: {:?}]", editings.scala, editings.ruota, editings.specchia, editings.bw, editings.contrasto, editings.luminosita );
            edit(data,editings)
        },
        Err(e) => {
            println!("Error persisting file: {}", e.to_string());
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub fn edit(data: web::Data<AppState>, e : Editings) -> HttpResponse {
/*
    let engine = Engine::default();
    
    let mut linker: Linker<WasiCtx> = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();

    let  image_directory = Dir::open_ambient_dir("img", ambient_authority()).expect("Error opening directory");
    let builder = WasiCtxBuilder::new()
    .inherit_stdout()
    .inherit_stderr()
    .preopened_dir(image_directory,"img").expect("Error setting preopened dir");

    let wasi = builder.build();
    
    let module = Module::from_file(&engine, "src/server/image_proc_module.wasm").expect("Error creating module from disk file");*/
    //let engine = data.engine.lock().unwrap();
    //let wasi = data.wasi_context.lock().unwrap();
    let mut linker = data.linker.lock().unwrap();
    let module = data.module.lock().unwrap();
    
    let mut store = data.store.lock().unwrap();
    
    let memory_type = MemoryType::new(1, None);
    Memory::new(&mut *store, memory_type);
    let serialized_input = serde_json::to_vec(&e).expect("Error serializing input");
    let mem_size = serialized_input.len() as i32;
    
    linker
    .func_wrap("host", "get_input_size", move || -> i32 { mem_size })
    .expect("should define the function");
    linker
        .func_wrap(
            "host",
            "get_input",
            move |mut caller: Caller<'_, WasiCtx>, ptr: i32| {
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => return (),
                };
                let offset = ptr as u32 as usize;
                match mem.write(&mut caller, offset, &serialized_input) {
                    Ok(_) => {}
                    _ => return (),
                };
                
            },
        )
        .expect("should define the function");

    linker.module(&mut *store, "", &module).expect("Error linking store to module");


    let instance = linker.instantiate(&mut *store, &module).expect("Error istantiating module");
    let instance_main = instance.get_typed_func::<(), ()>(&mut *store, "_start").expect("Error finding main function");
    instance_main.call(&mut * store, ()).expect("Error calling main function");
    println!("\n\n");
    drop(store);
    
/*
    let contents: Vec<u8> = stdout.try_into_inner().expect("sole remaining reference to WritePipe").into_inner();
    let out: String = contents.iter().map( |&i|  char::from_u32(i as u32).unwrap()).collect();
    */
    HttpResponse::Ok()
    .content_type("text/plain")
    .body(e.modified_file_path)

        
}