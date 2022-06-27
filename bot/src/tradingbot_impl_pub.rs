impl TradingBot {
    // main trading logic
    // high sell, low buy
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let current_price = self.get_market_price().await?;
        info!("[PRICE] current market price: {:?} $", current_price);

        let percentage_diff = (current_price - self.trading_config.last_operation_price)
            / self.trading_config.last_operation_price
            * 100 as f32;

        info!("[PRICE] percentage_diff: {:?} $", percentage_diff);
 
        // based on operation state for the buy and sell action
        match self.trading_config.next_operation {
            State::BUY => {
                self.trading_config.last_operation_price = self.try_to_buy(percentage_diff).await?;
            }
            State::SELL => {
                self.trading_config.last_operation_price = self.try_to_sell(percentage_diff).await?;
            }
        }

        Ok(())
    }
    
    pub fn new(trading_config: TradingConfig, market: Box<dyn Market>) -> Self {
        TradingBot {
            trading_config,
            market,
        }
    }
}