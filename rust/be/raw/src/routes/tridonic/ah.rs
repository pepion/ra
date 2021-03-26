use std::time::Instant;

use actix_web::{Error, HttpRequest, HttpResponse, web};
use futures::future::join_all;

use crate::routes::tridonic::hc;
use crate::routes::tridonic::hc::CustomClient;

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/hc/{name}").route(web::get().to(hyper_client)));
}

async fn hyper_client(
    _path: web::Path<String>,
    _req: HttpRequest,
    _body: web::Bytes,
    client: web::Data<CustomClient>,
) -> Result<HttpResponse, Error> {
    let start = Instant::now();
    spawn_fn(start.clone(), client.clone(), "modules".to_string()).await;
    Ok(HttpResponse::Ok().body("Started..."))
}

async fn spawn_fn(start: Instant, client: web::Data<CustomClient>, element: String) {
    actix_web::rt::spawn(async move {
        // to samo dodaje #[tokio::main] do main :)
        use tokio::runtime::Runtime;
        let rt = Runtime::new().unwrap();
        // Spawn a future onto the runtime
        rt.block_on(async {
            run_wait(start, client.clone(), element).await;
        });
        //
    });
}

async fn run_wait(start: Instant, client: web::Data<CustomClient>, _element: String) {
    let mut all_fn = Vec::new();
    let langs = vec!("en", "de", "fr", "es", "it", "sv", "cn", "ch", "pl", "mena");
    let elements = vec!("modules", "drivers", "groups");
    for element in elements {
        for lang in langs.clone() {
            all_fn.push(hc::get_to_file(start.clone(), client.clone(), element.to_string(), lang.to_string()));
        }
    }
    let responses = join_all(all_fn.into_iter()).await;

    // successful operations return lang, so confirm that all returned as so
    if !responses.iter().all(|res| match res {
        Ok(_lang) => {
            //println!("hc:OK {}", lang);
            true
        }
        _ => false,
    }) {
        let elapsed = start.elapsed();
        println!("hc:{:?} END: InternalServerError on get {:?}", elapsed, responses);
    } else {
        let elapsed = start.elapsed();
        println!("hc:{:?} END: all was saved to file::{:?}", elapsed, responses);
    }
}

