use crate::binance::{model::websocket::Subscription, Binance, BinanceWebsocket};
use binance_async as binance;
use failure::Fallible;
use futures::TryStreamExt;
use std::env::var;

#[tokio::main]
async fn main() -> Fallible<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();

    let api_key_user = var("BINANCE_KEY")?;
    let api_secret_user = var("BINANCE_SECRET")?;

    let bn = Binance::with_credential(&api_key_user, &api_secret_user);
    match bn.user_stream_start()?.await {
        Ok(_) => {
            println!("Data Stream Started ...");

            let mut ws = BinanceWebsocket::default();

            for sub in vec![
                // Subscription::Ticker("btcusdt.to_string()),
                // Subscription::AggregateTrade("btcusdt.to_string()),
                // Subscription::Candlestick("btcusdt".to_string(), "1m".to_string()),
                // Subscription::Depth("btcusdt".to_string()),
                // Subscription::MiniTicker("btcusdt".to_string()),
                // Subscription::OrderBook("btcusdt".to_string(), 10),
                Subscription::Trade("btcusdt".to_string()),
                // Subscription::UserData(listen_key),
                // Subscription::MiniTickerAll,
                // Subscription::TickerAll,
            ] {
                ws.subscribe(sub).await?;
            }

            while let Some(msg) = ws.try_next().await? {
                println!("\n\n{:#?}", msg)
            }
        }
        Err(e) => println!("Error obtaining userstream: {}", e),
    }

    Ok(())
}
