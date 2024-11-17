use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;
use std::env;

mod routes;
mod config;

#[derive(Deserialize)]
struct FileInfo {
    name: String,
    content: Vec<u8>,
}

async fn handle_action(action: web::Path<String>, file: web::Json<FileInfo>) -> impl Responder {
    match action.as_str() {
        "upload" => routes::upload::handle_upload(file.into_inner()).await,
        "compress" => routes::compress::handle_compress(file.into_inner()).await,
        "decompress" => routes::decompress::handle_decompress(file.into_inner()).await,
        _ => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let config = config::Config::from_env().expect("Server configuration error");

    HttpServer::new(|| {
        App::new()
            .route("/{action}", web::post().to(handle_action))
    })
    .bind(format!("{}:{}", config.server_host, config.server_port))?
    .run()
    .await
}