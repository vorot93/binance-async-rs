extern crate binance;

use binance::{Account, Binance, General, Market};

fn main() {
    general();
    account();
    market_data();
}

fn general() {
    let general = Binance::<General>::new();

    let ping = general.ping();
    match ping {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    let result = general.get_server_time();
    match result {
        Ok(answer) => println!("Server Time: {}", answer.server_time),
        Err(e) => println!("Error: {}", e),
    }
}

fn account() {
    let api_key = "YOUR_API_KEY";
    let secret_key = "YOUR_SECRET_KEY";

    let account = Binance::<Account>::new(&api_key, &secret_key);

    match account.get_account() {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {}", e),
    }

    match account.get_open_orders("WTCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.limit_buy("WTCETH", 10, 0.014000) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.market_buy("WTCETH", 5) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.limit_sell("WTCETH", 10, 0.035000) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.market_sell("WTCETH", 5) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    let order_id = 1_957_528;
    match account.order_status("WTCETH", order_id) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.cancel_order("WTCETH", order_id) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.get_balance("KNC") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.trade_history("WTCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}

fn market_data() {
    let market = Binance::<Market>::new();

    // Order book
    match market.get_depth("BNBETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ALL symbols
    match market.get_all_prices() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ONE symbol
    match market.get_price("KNCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match market.get_all_book_tickers() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match market.get_book_ticker("BNBETH") {
        Ok(answer) => println!("Bid Price: {}, Ask Price: {}", answer.bid_price, answer.ask_price),
        Err(e) => println!("Error: {}", e),
    }

    // 24hr ticker price change statistics
    match market.get_24h_price_stats("BNBETH") {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // last 10 5min klines (candlesticks) for a symbol:
    match market.get_klines("BNBETH", "5m", 10, None, None) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}
