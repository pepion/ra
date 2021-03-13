use actix_web::{web, get, Result};
use actix_files as fs;

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_favicon)
        .service(get_favicon_ico);
}

/// favicon handler
#[get("/favicon")]
async fn get_favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

#[get("/favicon.ico")]
async fn get_favicon_ico() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}
