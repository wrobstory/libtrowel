use futures::join;
use hyper::body::Bytes;
use hyper::client::HttpConnector;
use hyper::client::ResponseFuture;
use hyper::Body;
use std::fmt;
use std::future::Future;
use std::time::Instant;

use hyper::{Client, Response};
use hyper_tls::HttpsConnector;

use libtrowel::{parse_color_guide, parse_known_colors};
use nipper::Document;

type HttpsClient = Client<HttpsConnector<HttpConnector>>;

enum PartPageType {
    Color,
    Price,
}

impl From<PartPageType> for String {
    fn from(part_page_type: PartPageType) -> Self {
        match part_page_type {
            PartPageType::Color => "C".to_string(),
            PartPageType::Price => "P".to_string(),
        }
    }
}

async fn fetch_page(client: &HttpsClient, uri: &String) -> Result<Bytes, hyper::Error> {
    let resp = client.get(uri.parse().unwrap()).await?;
    hyper::body::to_bytes(resp.into_body()).await
}

async fn fetch_part_page(
    client: &HttpsClient,
    part_number: i32,
    part_page_type: PartPageType,
) -> Result<Bytes, hyper::Error> {
    let part_uri = format!(
        "https://www.bricklink.com/v2/catalog/catalogitem.page?P={}#T={}",
        part_number,
        String::from(part_page_type)
    );
    fetch_page(client, &part_uri).await
}

async fn fetch_color_guide_page(client: &HttpsClient) -> Result<Bytes, hyper::Error> {
    let color_guide_uri = "https://www.bricklink.com/catalogColors.asp";
    fetch_page(client, &color_guide_uri.to_string()).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let part_page = fetch_part_page(&client, 92593, PartPageType::Color).await?;

    let colors_html = Document::from(&String::from_utf8(part_page.to_vec()).unwrap());
    let colors = parse_known_colors(&colors_html);
    println!("{:?}", colors);

    let color_guide = fetch_color_guide_page(&client).await?;
    let color_guide_html = Document::from(&String::from_utf8(color_guide.to_vec()).unwrap());
    let color_guide = parse_color_guide(&color_guide_html);
    println!("{:?}", color_guide);

    Ok(())
}
