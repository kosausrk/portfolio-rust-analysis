//polygon api integration 

use polygon_client::rest::Client;
use std::error::Error;

pub async fn fetch_latest_price(client: &Client, symbol: &str) -> Result<f64, Box<dyn Error>> {
    let resp = client.get_last_trade(symbol).await?;
    Ok(resp.price)
}
