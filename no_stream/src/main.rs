use std::sync::Arc;

use hyper::body::Buf;
use hyper::{body, client::Client, Body, Request};
use hyper_tls::HttpsConnector;

use serde::Deserialize;

use crate::util::error::ResponseError;
use actix_web::{web, App, HttpResponse, HttpServer};

mod util;

#[derive(Debug, Deserialize)]
struct Element {
    id: u32,
    firstname: String,
    lastname: String,
    description: String,
    email: String,
    email2: String,
    profession: String,
}

async fn download_and_parse_csv() -> Result<(), ResponseError> {
    let mut elements = Vec::new();

    let https = HttpsConnector::new();
    let client = Arc::new(Client::builder().build::<_, hyper::Body>(https));

    let url =
        "https://raw.githubusercontent.com/arnaudpoullet/rust_csv_stream/master/static/myFile0.csv"
            .to_string();
    let req = Request::get(url)
        .body(Body::empty())
        .expect("Request builder");
    // Fetch the url...
    let res = client.request(req).await?;
    let body = body::to_bytes(res.into_body()).await?;
    //Read the body of the csv line by line and deserialize into Element if possible
    let mut rdr = csv::Reader::from_reader(body.reader());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: csv::Result<Element> = result;
        match record {
            Ok(element) => {
                //Process each element as it comes in
                elements.push(element);
            }
            Err(err) => return Err(ResponseError::from(err)),
        }
    }
    println!("Number of elements: {}", elements.len());
    println!("{:?}", elements.pop());
    Ok(())
}

async fn handler() -> HttpResponse {
    if let Err(err) = download_and_parse_csv().await {
        println!("Error: {}", err);
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .route("/", web::get().to(handler))
    })
    .bind(("127.0.0.1", "8085".parse().unwrap()))?
    .workers(1)
    .run()
    .await
}
