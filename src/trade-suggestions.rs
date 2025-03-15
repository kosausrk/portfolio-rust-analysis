//suggestions from algorithim 
pub fn suggest_trade(symbol: &str, ratio: f64) {
    if ratio > 2.0 {
        println!("Consider buying {}: favorable risk-reward ratio of {:.2}", symbol, ratio);
    } else {
        println!("Avoid buying {}: unfavorable risk-reward ratio of {:.2}", symbol, ratio);
    }
}
