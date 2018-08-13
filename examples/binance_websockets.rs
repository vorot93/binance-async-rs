extern crate binance_async as binance;
extern crate tokio;

use std::env::var;

use tokio::runtime::current_thread::Runtime;

use binance::error::Result;
use binance::model::{AccountUpdateEvent, DayTickerEvent, DepthOrderBookEvent, KlineEvent, OrderBook, OrderTradeEvent, TradesEvent};
use binance::websockets::*;
use binance::Binance;

fn main() -> Result<()> {
    let mut rt = Runtime::new()?;

    user_stream(&mut rt)?;
    user_stream_websocket(&mut rt)?;
    market_websocket()?;
    kline_websocket()?;
    all_trades_websocket()?;
    Ok(())
}

fn user_stream(rt: &mut Runtime) -> Result<()> {
    let api_key_user = var("BINANCE_API_KEY")?;
    let api_secret_user = var("BINANCE_API_SECRET")?;

    let user_stream = Binance::with_credential(&api_key_user, &api_secret_user);

    if let Ok(answer) = rt.block_on(user_stream.start()?) {
        println!("Data Stream Started ...");
        let listen_key = answer.listen_key;

        match rt.block_on(user_stream.keep_alive(&listen_key)?) {
            Ok(msg) => println!("Keepalive user data stream: {:?}", msg),
            Err(e) => println!("Error: {}", e),
        }

        match rt.block_on(user_stream.close(&listen_key)?) {
            Ok(msg) => println!("Close user data stream: {:?}", msg),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
    Ok(())
}

fn user_stream_websocket(rt: &mut Runtime) -> Result<()> {
    struct WebSocketHandler;

    impl UserStreamEventHandler for WebSocketHandler {
        fn account_update_handler(&self, event: &AccountUpdateEvent) {
            for balance in &event.balance {
                println!("Asset: {}, free: {}, locked: {}", balance.asset, balance.free, balance.locked);
            }
        }

        fn order_trade_handler(&self, event: &OrderTradeEvent) {
            println!(
                "Symbol: {}, Side: {}, Price: {}, Execution Type: {}",
                event.symbol, event.side, event.price, event.execution_type
            );
        }
    }

    let api_key_user = var("YOUR_KEY")?;
    let api_secret_user = var("YOUR_SECRET")?;
    let user_stream = Binance::with_credential(&api_key_user, &api_secret_user);

    if let Ok(answer) = rt.block_on(user_stream.start()?) {
        let listen_key = answer.listen_key;

        let mut web_socket: WebSockets = WebSockets::new();
        web_socket.add_user_stream_handler(WebSocketHandler);
        web_socket.connect(&listen_key).unwrap(); // check error
        web_socket.event_loop()?;
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
    Ok(())
}

fn market_websocket() -> Result<()> {
    struct WebSocketHandler;

    impl MarketEventHandler for WebSocketHandler {
        fn aggregated_trades_handler(&self, event: &TradesEvent) {
            println!("Symbol: {}, price: {}, qty: {}", event.symbol, event.price, event.qty);
        }

        fn depth_orderbook_handler(&self, event: &DepthOrderBookEvent) {
            println!("Symbol: {}, Bids: {:?}, Ask: {:?}", event.symbol, event.bids, event.asks);
        }

        fn partial_orderbook_handler(&self, order_book: &OrderBook) {
            println!("last_update_id: {}, Bids: {:?}, Ask: {:?}", order_book.last_update_id, order_book.bids, order_book.asks);
        }
    }

    let agg_trade: String = format!("{}@aggTrade", "ethbtc");
    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.add_market_handler(WebSocketHandler);
    web_socket.connect(&agg_trade).unwrap(); // check error
    web_socket.event_loop()
}

fn all_trades_websocket() -> Result<()> {
    struct WebSocketHandler;

    impl DayTickerEventHandler for WebSocketHandler {
        fn day_ticker_handler(&self, events: &[DayTickerEvent]) {
            for event in events {
                println!("Symbol: {}, price: {}, qty: {}", event.symbol, event.best_bid, event.best_bid_qty);
            }
        }
    }

    let agg_trade: String = format!("!ticker@arr");
    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.add_day_ticker_handler(WebSocketHandler);
    web_socket.connect(&agg_trade).unwrap(); // check error
    web_socket.event_loop()?;
    Ok(())
}

fn kline_websocket() -> Result<()> {
    struct WebSocketHandler;

    impl KlineEventHandler for WebSocketHandler {
        fn kline_handler(&self, event: &KlineEvent) {
            println!("Symbol: {}, high: {}, low: {}", event.kline.symbol, event.kline.low, event.kline.high);
        }
    }

    let kline: String = format!("{}", "ethbtc@kline_1m");
    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.add_kline_handler(WebSocketHandler);
    web_socket.connect(&kline).unwrap(); // check error
    web_socket.event_loop()
}
