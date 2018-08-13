extern crate binance_async as binance;
extern crate tokio;

use std::env::var;

use tokio::runtime::current_thread::Runtime;

use binance::error::Result;
use binance::Binance;

fn main() -> Result<()> {
    let mut rt = Runtime::new()?;
    general(&mut rt)?;
    account(&mut rt)?;
    market_data(&mut rt)?;
    Ok(())
}

fn general(rt: &mut Runtime) -> Result<()> {
    let binance = Binance::new();

    let ping = rt.block_on(binance.ping()?);
    match ping {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    let result = rt.block_on(binance.get_server_time()?);
    match result {
        Ok(answer) => println!("Server Time: {}", answer.server_time),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

fn account(rt: &mut Runtime) -> Result<()> {
    let api_key = var("BINANCE_API_KEY")?;
    let secret_key = var("BINANCE_SECRET_KEY")?;

    let binance = Binance::with_credential(&api_key, &secret_key);

    match rt.block_on(binance.get_account()?) {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(binance.get_open_orders("WTCETH")?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(binance.limit_buy("WTCETH", 10., 0.014000)?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(binance.market_buy("WTCETH", 5.)?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(binance.limit_sell("WTCETH", 10., 0.035000)?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(binance.market_sell("WTCETH", 5.)?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    let order_id = 1_957_528;
    match rt.block_on(binance.order_status("WTCETH", order_id)?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(binance.cancel_order("WTCETH", order_id)?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(binance.get_balance("KNC")?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(binance.trade_history("WTCETH")?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

fn market_data(rt: &mut Runtime) -> Result<()> {
    let binance = Binance::new();

    // Order book
    match rt.block_on(binance.get_depth("BNBETH", None)?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ALL symbols
    match rt.block_on(binance.get_all_prices()?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ONE symbol
    match rt.block_on(binance.get_price("KNCETH")?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match rt.block_on(binance.get_all_book_tickers()?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match rt.block_on(binance.get_book_ticker("BNBETH")?) {
        Ok(answer) => println!("Bid Price: {}, Ask Price: {}", answer.bid_price, answer.ask_price),
        Err(e) => println!("Error: {}", e),
    }

    // 24hr ticker price change statistics
    match rt.block_on(binance.get_24h_price_stats("BNBETH")?) {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // last 10 5min klines (candlesticks) for a symbol:
    match rt.block_on(binance.get_klines("BNBETH", "5m", 10, None, None)?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}
