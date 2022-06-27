pub struct TradingBot {
    pub trading_config: TradingConfig,
    pub market: Box<dyn Market>,
}