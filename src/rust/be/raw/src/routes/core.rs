use actix_web::{web, get, Result};
use actix_files as fs;

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_favicon);
}

/// favicon handler
#[get("/favicon")]
async fn get_favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}
