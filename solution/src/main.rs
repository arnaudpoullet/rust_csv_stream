use serde::Deserialize;

use crate::util::error::ResponseError;
use actix_web::{web, App, HttpResponse, HttpServer};
use futures_util::{StreamExt, TryStreamExt};

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

    let url =
        "https://raw.githubusercontent.com/arnaudpoullet/rust_csv_stream/master/static/myFile0.csv"
            .to_string();
    let res = reqwest::get(&url).await?;
    let body = res
        .bytes_stream()
        .map(|result| {
            result.map_err(|error| {
                std::io::Error::new(std::io::ErrorKind::Other, format!("{}", error))
            })
        })
        .into_async_read();
    //Read the body of the csv line by line and deserialize into Element if possible
    let mut rdr = csv_async::AsyncDeserializer::from_reader(body);
    let mut records = rdr.deserialize::<Element>();
    while let Some(record) = records.next().await {
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
