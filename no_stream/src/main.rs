mod util;

use crate::util::error::ResponseError;
use std::sync::Arc;
use hyper::Body;

struct Element {
    id: u32,
    firstname: String,
    lastname: String,
    description: String,
    email: String,
    email2: String,
    profession: String
}

async fn download_and_parse_csv() -> Result<(), ResponseError> {
    let https = HttpsConnector::new();
    let client = Arc::new(Client::builder()
        .build::<_, hyper::Body>(https));

    let url = "url".to_string();
    let req = Request::get(url).body(Body::empty()).expect("Request builder");
    // Fetch the url...
    let res = client.request(req).await?;
    let body= body::to_bytes(res.into_body()).await?;
    //Read the body of the csv line by line and deserialize into Element if possible
    let mut rdr = csv::Reader::from_reader(body.reader());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: csv::Result<Element> = result;
        match record {
            Ok(element) => {
                println!("{:?}",element.email2);
            },
            Err(err) => return Err(ResponseError::from(err)),
        }
    }
    println!("testing");
    Ok(())
}

#[actix_rt::main]
async fn main() -> Result<(), ResponseError> {
    download_and_parse_csv().await
}
