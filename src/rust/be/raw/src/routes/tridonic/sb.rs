use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web::client::Client;
use actix_web::web::BytesMut;
use futures::future::join_all;
use futures::StreamExt;

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/tr/{name}").route(web::get().to(forward)));
    cfg.service(web::resource("/tr2/{name}").route(web::get().to(forward2)));
}


// TODO melisearch from files

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
    println!("{:?} END: {}/{} was saved to file. buf={:?}", elapsed, element, lang, buf);

    Ok(lang)
}

async fn run_wait(start: Instant, client: web::Data<Client>, element: String) {
    let mut all_fn = Vec::new();
    let langs = vec!("en", "de", "fr", "es", "it", "sv", "cn", "ch", "pl", "mena");
    for lang in langs {
        all_fn.push(get_to_file(start.clone(), client.clone(), element.clone(), lang.to_string()));
    }
    let responses = join_all(all_fn.into_iter()).await;


    // successful operations return lang, so confirm that all returned as so
    if !responses.iter().all(|res| match res {
        Ok(lang) => {
            println!("OK {}", lang);
            true
        }
        _ => false,
    }) {
        let elapsed = start.elapsed();
        println!("{:?} END: InternalServerError on get {}::{:?}", elapsed, element, responses);
    } else {
        let elapsed = start.elapsed();
        println!("{:?} END: all {} was saved to file::{:?}", elapsed, element, responses);
    }
}

async fn spawn_fn(start: Instant, client: web::Data<Client>, element: String) {
    actix_web::rt::spawn(async move {
        run_wait(start, client.clone(), element).await;
        //run_wait(start, client.clone(), "drivers".to_string()).await;
    });
}

async fn forward2(
    _path: web::Path<String>,
    _req: HttpRequest,
    _body: web::Bytes,
    //url: web::Data<Url>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let start = Instant::now();
    spawn_fn(start.clone(), client.clone(), "modules".to_string()).await;
    // TODO: timeout problem 10 modules ok in 84-100 sec
    // + 6 driver ok + 4 driver Err(Connect(Timeout)) even before 67 sec, when others in 100 sec !!!
    spawn_fn(start.clone(), client.clone(), "drivers".to_string()).await;
    Ok(HttpResponse::Ok().body("Started..."))
}

async fn forward(
    _path: web::Path<String>,
    _req: HttpRequest,
    _body: web::Bytes,
    //url: web::Data<Url>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let timeout = Duration::new(1000, 0);
    let mut response = client
        .get("http://setbuilderapi.tridonic.com/getdrivers/pl")
        .header("User-Agent", "actix-web/3.0")
        .header("Accept", "application/json")
        .timeout(timeout)
        .send()
        .await
        .map_err(Error::from)?;

    //println!("Response: {:?}", response);

    let body = response.body().limit(100_000_0000).await?;

    let mut client_resp = HttpResponse::build(response.status());
    // Remove `Connection` as per
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
    for (header_name, header_value) in response.headers().iter().filter(|(h, _)| *h != "connection") {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    Ok(client_resp.body(body))
}
