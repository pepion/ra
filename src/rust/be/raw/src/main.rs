use std::{env, io};
use std::time::Duration;

use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_web::{
    App, error, get, guard, HttpRequest, HttpResponse, HttpServer, middleware,
    Result, web,
};
use actix_web::client::{ClientBuilder, Connector};
use actix_web::http::{header, Method, StatusCode};

use raw::routes;

/// simple index handler
#[get("/welcome")]
async fn welcome(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    // set counter to session
    session.set("counter", counter)?;

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/welcome.html")))
}

/// 404 handler
async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

/// handler with path parameters like `/user/{name}/`
async fn with_param(
    req: HttpRequest,
    web::Path((name, )): web::Path<(String, )>,
) -> HttpResponse {
    println!("{:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", name))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    let num_cpus = num_cpus::get();
    println!("num_cpus={}", num_cpus);

    HttpServer::new(|| {
        App::new()
            .data(ClientBuilder::new()
                .no_default_headers()
                .disable_timeout()  // we will not receive a response
                .connector(Connector::new()
                    .timeout(Duration::from_secs(5))
                    //.conn_keep_alive(Duration::from_secs(200))
                    //.conn_lifetime(Duration::from_secs(3600))
                    .finish())
                .finish()
            )
            .data(routes::tridonic::hc::CustomClient::new())
            // cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register favicon
            .configure(routes::core::services)
            .configure(routes::stream::services)
            .configure(routes::tridonic::ac::services)
            .configure(routes::tridonic::ah::services)
            // register simple route, handle all methods
            .service(welcome)
            // with path parameters
            .service(web::resource("/user/{name}").route(web::get().to(with_param)))
            .service(
                web::resource("/test").to(|req: HttpRequest| match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }),
            )
            .service(web::resource("/error").to(|| async {
                error::InternalError::new(
                    io::Error::new(io::ErrorKind::Other, "test"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }))
            // static files
            .service(fs::Files::new("/static", "static").show_files_listing())
            // redirect
            .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
                println!("{:?}", req);
                HttpResponse::Found()
                    .header(header::LOCATION, "static/welcome.html")
                    .finish()
            })))
            // default
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
        .bind("127.0.0.1:8080")?
        .workers(4)
        .run()
        .await
}
