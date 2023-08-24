use actix_web::{web, App, HttpServer};
use actix_files as fs;
use env_logger;
mod server{
    pub mod handlers;
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(server::handlers::index))
            .route("/upload", web::post().to(server::handlers::upload))
            .route("/edit", web::post().to(server::handlers::edit))
            .service(fs::Files::new("/script", "./src/static/"))
            .service(fs::Files::new("/img", "./img/"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}