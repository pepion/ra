use std::time::Duration;

//use actix_rt;
use actix_utils::mpsc;
use actix_web::{Error, HttpResponse, web};
use futures::future::FutureExt;
use futures_timer::Delay;

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/async-body/{name}").route(web::get().to(response_body)));
}

async fn response_body(path: web::Path<String>) -> HttpResponse {
    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(web::Bytes::from(format!("START {}!", *path))));

    let now_future = Delay::new(Duration::from_secs(5));

    actix_web::rt::spawn(now_future.map(|_x| {
        println!("waited for 5 secs");
    }));

    let tx_future = async { tx };
    actix_web::rt::spawn(tx_future.map(move |x| {
        let _ = x.send(Ok::<_, Error>(web::Bytes::from(format!("END {}!", *path))));
        println!("END was send");
    }));

    HttpResponse::Ok().streaming(rx_body)
}
