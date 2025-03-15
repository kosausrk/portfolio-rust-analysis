//portfolio analysis from spreadsheet data 

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_portfolio(file_path: &str) -> Result<Vec<(String, f64, f64)>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut portfolio = Vec::new();

    for line in reader.lines().skip(1) { // Skip header
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() >= 3 {
            let symbol = fields[0].to_string();
            let quantity = fields[1].parse::<f64>()?;
            let purchase_price = fields[2].parse::<f64>()?;
            portfolio.push((symbol, quantity, purchase_price));
        }
    }
    Ok(portfolio)
}
