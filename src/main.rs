use hyper::Client;
use hyper_tls::HttpsConnector;


use std::default::Default;
use std::io::{self, Write};

use scraper::html::Html;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let part_uri = "https://www.bricklink.com/v2/catalog/catalogitem.page?P=92593".parse()?;
    let resp = client.get(part_uri).await?;
    let doc = hyper::body::to_bytes(resp.into_body()).await?;
    println!("{:?}", Html::parse_document(&String::from_utf8(doc.to_vec()).unwrap()));

    Ok(())
}