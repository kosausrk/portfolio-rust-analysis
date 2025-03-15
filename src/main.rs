//main entry point 

mod portfolio;
mod market_data;
mod risk_analysis;
mod trade_suggestions;

use polygon_client::rest::Client;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "SAMPLE_POLYGON_KEY"; //fix for polygon credentials 
    let client = Client::new(api_key);
    
    let portfolio = portfolio::read_portfolio("data/portfolio.csv")?;

    for (symbol, quantity, purchase_price) in portfolio {
        let latest_price = market_data::fetch_latest_price(&client, &symbol).await?;
        let stop_loss = purchase_price * 0.95; //ex: 5% below purchase price
        let target_price = purchase_price * 1.10; //ex: 10% above purchase price
        let ratio = risk_analysis::calculate_risk_reward(purchase_price, stop_loss, target_price);
        
        trade_suggestions::suggest_trade(&symbol, ratio);
    }

    Ok(())
}
