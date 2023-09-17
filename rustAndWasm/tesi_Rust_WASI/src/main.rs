use actix_web::{web, App, HttpServer};
use actix_files as fs;
use env_logger;
use wasmtime::*;
use wasmtime_wasi::*;
use wasmtime_wasi::sync::Dir;
use tesi_Rust_WASI::common::AppState;
use tesi_Rust_WASI::server;
use std::sync::Mutex;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let engine = Engine::default();
    
    let mut linker: Linker<WasiCtx> = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();

    let  image_directory = Dir::open_ambient_dir("img", ambient_authority()).expect("Error opening directory");
    let builder = WasiCtxBuilder::new()
    .inherit_stdout()
    .inherit_stderr()
    .preopened_dir(image_directory,"img").expect("Error setting preopened dir");

    let wasi_context = builder.build();
    
    let module = Module::from_file(&engine, "src/server/image_proc_module.wasm").expect("Error creating module from disk file");
    let store = Store::new(&engine, wasi_context);
    let state = web::Data::new(AppState {
        linker: Mutex::new(linker),
        module: Mutex::new(module),
        store: Mutex::new(store)
    });
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(server::handlers::index))
            .route("/upload", web::post().to(server::handlers::upload))
            .service(fs::Files::new("/script", "./src/static/"))
            .service(fs::Files::new("/img", "./img/"))
            .app_data(state.clone())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}