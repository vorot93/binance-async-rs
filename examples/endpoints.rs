use crate::binance::Binance;
use binance_async as binance;
use failure::Fallible;
use std::env::var;

#[tokio::main]
async fn main() -> Fallible<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();

    let api_key = var("BINANCE_KEY")?;
    let secret_key = var("BINANCE_SECRET")?;

    let bn = Binance::with_credential(&api_key, &secret_key);

    // General
    match bn.ping()?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.get_server_time()?.await {
        Ok(answer) => println!("Server Time: {}", answer.server_time),
        Err(e) => println!("Error: {}", e),
    }

    // Account
    match bn.get_account()?.await {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {}", e),
    }

    match bn.get_open_orders("BTCUSDT")?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.limit_buy("BTCUSDT", 1., 0.1)?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.market_buy("BTCUSDT", 5.)?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.limit_sell("BTCUSDT", 10., 0.035_000)?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.market_sell("BTCUSDT", 5.)?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.order_status("BTCUSDT", 1_957_528)?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.cancel_order("BTCUSDT", 1_957_528)?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.get_balance("BTC")?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.trade_history("BTCUSDT")?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Market

    // Order book
    match bn.get_depth("BTCUSDT", None)?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ALL symbols
    match bn.get_all_prices()?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ONE symbol
    match bn.get_price("BTCUSDT")?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match bn.get_all_book_tickers()?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match bn.get_book_ticker("BTCUSDT")?.await {
        Ok(answer) => println!(
            "Bid Price: {}, Ask Price: {}",
            answer.bid_price, answer.ask_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // 24hr ticker price change statistics
    match bn.get_24h_price_stats("BTCUSDT")?.await {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // last 10 5min klines (candlesticks) for a symbol:
    match bn.get_klines("BTCUSDT", "5m", 10, None, None)?.await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
