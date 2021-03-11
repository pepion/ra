#[macro_use]
extern crate serde_derive;

use serde_json::Value;
use std::collections::HashMap;

use actix_web::client::Client;

use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;

#[actix_web::main]
async fn main() {
    let client = Client::default();

    let timeout = Duration::new(1000, 0);

    let response = client
        .get("http://setbuilderapi.tridonic.com/getdrivers/pl")
        .header("User-Agent", "actix-web/3.0")
        .header("Accept", "application/json")
        .timeout(timeout)
        .send() // <- Send request
        .await; // <- Wait for response

    //println!("Response: {:?}", response);

    let body = response.unwrap().body().limit(100_000_0000).await;

    //println!("Downloaded: {:?} bytes", body);
    let b = body.as_ref().unwrap();
    // let len = b.len();
    // let utf8 = std::str::from_utf8(b).unwrap();
    // // let str_len = utf8.len();
    // //       println!("Downloaded {:?}/{:?}:{}", len, str_len, utf8);
    // //       println!("Downloaded {:?}/{:?}", len, str_len);
    //
    // let modules: TrModules = serde_json::from_str(utf8).unwrap();
    // //   println!("{:#?}", modules);
    //
    // // Serialize it to a JSON string.
    // let str_modules = serde_json::to_string(&modules).unwrap();
    //
    // // Print, write to a file, or send to an HTTP server.
    // println!(
    //     "xxxx: {} :xxxx len={}/{}",
    //     str_modules,
    //     str_modules.len(),
    //     len
    // );

    let mut file = File::create("drivers.json").unwrap();
    file.write_all(b);

}

// #[derive(Deserialize, Debug)]
// struct Item {
//     value: u64,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct TrModules {
    module_complete: Vec<HashMap<String, Value>>,
}

// #[derive(Deserialize, Debug)]
// pub struct TrModules {
//     module_complete: Vec<TrModule>,
// }

impl TrModules {
    pub fn new() -> Self {
        Self {
            module_complete: vec![],
        }
    }
}

// #[derive(Deserialize, Debug)]
// pub struct TrModule {
//     layer: String,
// }
