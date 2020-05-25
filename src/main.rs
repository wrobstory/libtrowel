use hyper::client::ResponseFuture;
use hyper::client::HttpConnector;
use hyper::Body;
use std::future::Future;
use futures::join;
use std::fmt;
use std::time::Instant;

use hyper::{Client, Response};
use hyper_tls::HttpsConnector;

use libtrowel::parse_known_colors;
use scraper::Html;

enum PartPageType {
    Color,
    Price
}

impl From<PartPageType> for &str {
    fn from(part_page_type: PartPageType) -> Self {
        match part_page_type {
            PartPageType::Color => "C",
            PartPageType::Price => "P"
        }
    }
}

async fn fetch_page(client: Client<HttpsConnector<HttpConnector>, Body>, uri: &str) -> Result<Response<Body>, hyper::Error> {
    client.get(uri.parse().unwrap()).await
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let now = Instant::now();

    println!("Starting {}", now.elapsed().as_millis());
    let part_uri = "https://www.bricklink.com/v2/catalog/catalogitem.page?P=92593#T=C";
    let (resp, ) = join!(fetch_page(client, part_uri));
    println!("Page fetch {}", now.elapsed().as_millis());
    let body: hyper::Body = resp?.into_body();
    let doc: hyper::body::Bytes = hyper::body::to_bytes(body)?;
    let foo: String = doc;
    println!("Byte conversion {}", now.elapsed().as_millis());
    let parsed = Html::parse_document(&String::from_utf8(doc.to_vec()).unwrap());
    println!("doc parsing {}", now.elapsed().as_millis());
    let colors = parse_known_colors(&parsed);
    println!("{:?}", colors);

    Ok(())
}
