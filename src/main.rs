mod utils;
mod endpoints;

use actix_web::{HttpServer, App};

use actix_files as fs;

use crate::utils::config::Config;

use endpoints::{upload, uploads, index};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load();
    let sora_host: &str = &config.get_string("sora.host").unwrap();
    let sora_port: u16 = config.get_int("sora.port").unwrap().try_into().unwrap();

    HttpServer::new(|| {
        App::new()
            .service(upload::upload)
            .service(uploads::uploads)
            .service(index::index)
            .service(fs::Files::new("/files", "./files"))
            .service(fs::Files::new("/static", "./static"))
    })
    .bind((sora_host, sora_port))?
    .run()
    .await
}
