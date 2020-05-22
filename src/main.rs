use hyper::Client;
use hyper_tls::HttpsConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let part_uri = "https://www.bricklink.com/v2/catalog/catalogitem.page?P=92593".parse()?;
    let resp = client.get(part_uri).await?;
    println!("{:?}", resp);

    Ok(())
}