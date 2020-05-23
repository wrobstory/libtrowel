use std::time::Instant;

use hyper::Client;
use hyper_tls::HttpsConnector;


use std::default::Default;
use std::io::{self, Write};
use std::str::from_utf8;

use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let now = Instant::now();

    println!("Starting {}", now.elapsed().as_millis());
    let part_uri = "https://www.bricklink.com/v2/catalog/catalogitem.page?P=92593".parse()?;
    let resp = client.get(part_uri).await?;
    println!("Page fetch {}", now.elapsed().as_millis());
    let doc = hyper::body::to_bytes(resp.into_body()).await?;
    println!("Byte conversion {}", now.elapsed().as_millis());
    let parsed = Html::parse_document(&String::from_utf8(doc.to_vec()).unwrap());
    println!("doc parsing {}", now.elapsed().as_millis());
    let selector = Selector::parse(r#"table[class="pciColorInfoTable"]"#).unwrap();
    let next_selected = parsed.select(&selector).next().unwrap();
    println!("Selecting {}", now.elapsed().as_millis());
    
    

    Ok(())
}