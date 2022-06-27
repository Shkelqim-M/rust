// initial the TradingBot for coinbase context
let mut coinbase_bot = bot::TradingBot::new(new_config, Box::new(bot::coinbase::Coinbase {}));

// set the interval for every 20s
let trading_cadence = env::var("TRADING_CADENCE").unwrap().parse::<u64>().unwrap();
let mut interval = time::interval(time::Duration::from_secs(trading_cadence));

loop {
    // wait every 20s
    interval.tick().await;

    // trading start time
    let start = Instant::now();
    let now = Data<Local> = Local::now().date();

    // trading kick off
    info!("[TRADE] start at {:?}", now);
    coinbase_bot.start().await.unwrap();

    // trading end time
    let duration = start.elapsed();
    info!("[TRADE] end elapsed: {:?}", duration);
    info!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
}