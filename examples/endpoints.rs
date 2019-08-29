use binance_async as binance;

use std::env::var;

use failure::Fallible;
use futures::compat::*;

use crate::binance::Binance;

#[tokio::main(single_thread)]
async fn main() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();
    let api_key = var("BINANCE_KEY")?;
    let secret_key = var("BINANCE_SECRET")?;

    let bn = Binance::with_credential(&api_key, &secret_key);

    // General
    match bn.ping()?.compat().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.get_server_time()?.compat().await {
        Ok(answer) => println!("Server Time: {}", answer.server_time),
        Err(e) => println!("Error: {}", e),
    }

    // Account
    match bn.get_account()?.compat().await {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {}", e),
    }

    match bn.get_open_orders("WTCETH")?.compat().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.limit_buy("ETHBTC", 1., 0.1)?.compat().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.market_buy("WTCETH", 5.)?.compat().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.limit_sell("WTCETH", 10., 0.035000)?.compat().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.market_sell("WTCETH", 5.)?.compat().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.order_status("WTCETH", 1_957_528)?.compat().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.cancel_order("WTCETH", 1_957_528)?.compat().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.get_balance("KNC")?.compat().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match bn.trade_history("WTCETH")?.compat().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Market

    // Order book
    match bn.get_depth("BNBETH", None)?.compat().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ALL symbols
    match bn.get_all_prices()?.compat().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ONE symbol
    match bn.get_price("KNCETH")?.compat().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match bn.get_all_book_tickers()?.compat().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match bn.get_book_ticker("BNBETH")?.compat().await {
        Ok(answer) => println!(
            "Bid Price: {}, Ask Price: {}",
            answer.bid_price, answer.ask_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // 24hr ticker price change statistics
    match bn.get_24h_price_stats("BNBETH")?.compat().await {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // last 10 5min klines (candlesticks) for a symbol:
    match bn
        .get_klines("BNBETH", "5m", 10, None, None)?
        .compat()
        .await
    {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
