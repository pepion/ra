use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web::client::Client;
use actix_web::web::BytesMut;
use futures::future::join_all;
use futures::StreamExt;

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ac/{name}").route(web::get().to(actix_client)));
}

// TODO melisearch from files

async fn actix_client(
    _path: web::Path<String>,
    _req: HttpRequest,
    _body: web::Bytes,
    //url: web::Data<Url>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let start = Instant::now();
    spawn_fn(start.clone(), client.clone(), "modules".to_string()).await;
    Ok(HttpResponse::Ok().body("ac:Started..."))
}

async fn spawn_fn(start: Instant, client: web::Data<Client>, element: String) {
    actix_web::rt::spawn(async move {
        run_wait(start, client.clone(), element).await;
    });
}

async fn run_wait(start: Instant, client: web::Data<Client>, _element: String) {
    let mut all_fn = Vec::new();

    let langs = vec!("en", "de", "fr", "es", "it", "sv", "cn", "ch", "pl", "mena");
    let elements = vec!("modules", "drivers", "groups");
    for element in elements {
        for lang in langs.clone() {
            all_fn.push(get_to_file(start.clone(), client.clone(), element.to_string(), lang.to_string()));
        }
    }

    let responses = join_all(all_fn.into_iter()).await;

    // successful operations return lang, so confirm that all returned as so
    if !responses.iter().all(|res| match res {
        Ok(_lang) => {
            //println!("ac:OK {}", lang);
            true
        }
        _ => false,
    }) {
        let elapsed = start.elapsed();
        println!("ac:{:?} END: InternalServerError on get {:?}", elapsed, responses);
    } else {
        let elapsed = start.elapsed();
        println!("ac:{:?} END: all was saved to file::{:?}", elapsed, responses);
    }
}

async fn get_to_file(start: Instant, client: web::Data<Client>, element: String, lang: String) -> Result<String, Error> {
    let url = format!("http://setbuilderapi.tridonic.com/get{}/{}", element, lang);
    let path = format!("{}_{}.json", element, lang);

    let timeout = Duration::new(10000, 0);
    let mut response = client
        .get(url)
        .header("User-Agent", "actix-web/3.0")
        .header("Accept", "application/json")
        .timeout(timeout)
        .send()
        .await
        .map_err(Error::from)?;


    let mut body = BytesMut::new();
    //let mut i = 1;
    while let Some(chunk) = response.next().await {
        // i = i + 1;
        // println!("chunk i=|{:#?}|", i);
        body.extend_from_slice(&chunk?);
    }
    // println!("all body={:#?}", &body);

    let b = body.as_ref();
    let mut file = File::create(path)?;
    let buf = file.write_all(b)?;
    let elapsed = start.elapsed();
    println!("ac:{:?} END: {}/{} was saved to file. buf={:?}", elapsed, element, lang, buf);

    Ok(format!("ac:{}_{}", element, lang))
}




