use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use std::env;

mod routes;
mod config;

#[derive(Deserialize)]
struct FileInfo {
    name: String,
    content: Vec<u8>,
}

async fn upload(file: web::Json<FileInfo>) -> impl Responder {
    routes::upload::handle_upload(file.into_inner()).await
}

async fn compress(file: web::Json<FileInfo>) -> impl Responder {
    routes::compress::handle_compress(file.into_inner()).await
}

async fn decompress(file: web::Json<FileInfo>) -> impl Responder {
    routes::decompress::handle_decompress(file.into_inner()).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let config = config::Config::from_env().expect("Server configuration error");

    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(upload))
            .route("/compress", web::post().to(compress))
            .route("/decompress", web::post().to(decompress))
    })
    .bind(format!("{}:{}", config.server_host, config.server_port))?
    .run()
    .await
}