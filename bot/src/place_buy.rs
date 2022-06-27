// get the buy point
// buy action
async fn try_to_buy(&mut self, diff: f32) -> Result<f32, Box<dyn Error>> {
    if diff >= self.trading_config.upward_trend_threshold || diff <= self.trading_config.dip_threshold {
        let current_balance = self.get_balance().await?;
        info!("[BALANCE] current account balance {:?} $ USD", current_balance);
        
        self.trading_config.last_operation_price = self.place_buy_order(current_balance).await?;
        // set to the next action to sell
        self.trading_config.next_operation = State::SELL;
        info!("[BUY] Bought 0.002 BTC for {:?} $ USD", self.trading_config.last_operation_price);

    }

    Ok(self.trading_config.last_operation_price);
}