use std::sync::Arc;

use hyper::{client::Client, Body, Request};
use hyper_tls::HttpsConnector;

use serde::Deserialize;

use crate::util::error::ResponseError;
use actix_web::{web, App, HttpResponse, HttpServer};
use csv::DeserializeErrorKind;
use futures::TryStreamExt;
use std::io::Cursor;
use tokio::io::AsyncBufReadExt;

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

fn hyper_to_io(hyper: hyper::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Interrupted, hyper.to_string())
}

/// This function avoids keeping the entire csv file in memory by removing
/// the parts that have already been read from it.
fn shorten_vec(vec: &mut Cursor<Vec<u8>>) {
    let position = vec.position();
    println!("pos: {}", position);
    // Remove everything in the vector up to position.
    vec.get_mut().drain(..);
    // Set the position to zero.
    vec.set_position(0);
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
    let mut body = tokio_util::io::StreamReader::new(res.into_body().map_err(hyper_to_io));
    //Read the body of the csv line by line and deserialize into Element if possible
    let mut rdr = csv::Reader::from_reader(Cursor::new(Vec::new()));
    rdr.set_headers(csv::StringRecord::from(vec![
        "id",
        "firstname",
        "lastname",
        "description",
        "email",
        "email2",
        "profession",
    ]));
    let mut first_line = true;
    let mut clear = true;
    let mut iter = rdr.deserialize();
    loop {
        let inner_vec = iter.reader_mut().get_mut();
        if clear {
            shorten_vec(inner_vec);
            clear = true
        }
        let len = body.read_until(b'\n', inner_vec.get_mut()).await?;
        if len == 0 {
            // Reached end of file.
            break;
        }

        if first_line {
            first_line = false;
            continue;
        }
        let record: csv::Result<Element> = iter.next().expect("EOF but not end");
        match record {
            Ok(element) => {
                elements.push(element);
            }
            Err(err) => match err.kind() {
                csv::ErrorKind::Deserialize { pos: _, err: e } => match e.kind() {
                    DeserializeErrorKind::UnexpectedEndOfRow => clear = false,
                    _ => return Err(ResponseError::from(err)),
                },
                _ => return Err(ResponseError::from(err)),
            },
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
