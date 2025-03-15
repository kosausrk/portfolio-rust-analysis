//risk-reward 
pub fn calculate_risk_reward(entry_price: f64, stop_loss: f64, target_price: f64) -> f64 {
    let risk = entry_price - stop_loss;
    let reward = target_price - entry_price;
    reward / risk
}
