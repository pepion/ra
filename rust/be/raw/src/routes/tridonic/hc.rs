use std::sync::Arc;
use std::time::Instant;

use actix_web::web::Data;
use hyper::{Body, Method, Request, Response};
use hyper::body::HttpBody as _;
use hyper::client::HttpConnector;
use tokio::fs::File;
use tokio::io::AsyncWriteExt as _;
use tokio::sync::Semaphore;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

//https://github.com/hyperium/hyper/issues/2419
#[derive(Clone)]
pub struct CustomClient {
    client: hyper::client::Client<HttpConnector>,
    guard: Arc<Semaphore>,
}

impl CustomClient {
    pub fn new() -> Self {
        let client = hyper::client::Client::new();
        let guard = Arc::new(Semaphore::new(240)); //Harcoded limit just below threshold 250

        CustomClient {
            client,
            guard,
        }
    }

    pub async fn request(&self, req: Request<Body>) -> Result<Response<Body>> {
        //Is allowed to make request? If not then wait.
        let _permit = self.guard.acquire().await?;

        let rsp = self.client.request(req).await?;
        Ok(rsp)
    }

    pub async fn get(&self, url: String) -> Result<Response<Body>> {
        let req = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header("content-type", "application/json")
            .body(Body::empty())?;
        let rsp = self.request(req).await?;
        Ok(rsp)
    }
}

pub async fn get_to_file(start: Instant, client: Data<CustomClient>, element: String, lang: String) -> Result<String> {
    let url = format!("http://setbuilderapi-dev.tridonic.com/get{}/{}", element, lang);
    let path = format!("{}_{}_dev.json", element, lang);

    let mut res = client.get(url).await?;

    //println!("hc:Response: {}", res.status());
    //println!("hc:Headers: {:#?}\n", res.headers());

    let mut file = File::create(path).await?;
    // Stream the body, writing each chunk to file as we get it
    let mut i = 1;
    while let Some(next) = res.data().await {
        let chunk = next?;
        i = i + 1;
        //println!("hc:Done! i={}", i);
        let _buf = file.write_all(&chunk).await?;
    }

    let elapsed = start.elapsed();
    println!("hc:{:?} END: {}/{} was saved to file with {} chanks", elapsed, element, lang, i);

    Ok(format!("hc:{}_{}", element, lang))
}

