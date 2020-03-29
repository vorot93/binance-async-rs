use crate::binance::{model::websocket::Subscription, Binance, BinanceWebsocket};
use binance_async as binance;
use failure::Fallible;
use std::env::var;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Fallible<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();

    let api_key_user = var("BINANCE_KEY")?;
    let api_secret_user = var("BINANCE_SECRET")?;

    let bn = Binance::with_credential(&api_key_user, &api_secret_user);
    match bn.user_stream_start()?.await {
        Ok(answer) => {
            println!("Data Stream Started ...");
            let listen_key = answer.listen_key;

            let mut ws = BinanceWebsocket::default();

            for sub in vec![
                Subscription::Ticker("ethbtc".to_string()),
                Subscription::AggregateTrade("eosbtc".to_string()),
                Subscription::Candlestick("ethbtc".to_string(), "1m".to_string()),
                Subscription::Depth("xrpbtc".to_string()),
                Subscription::MiniTicker("zrxbtc".to_string()),
                Subscription::OrderBook("trxbtc".to_string(), 5),
                Subscription::Trade("adabtc".to_string()),
                Subscription::UserData(listen_key),
                Subscription::MiniTickerAll,
                Subscription::TickerAll,
            ] {
                ws.subscribe(sub).await?;
            }

            while let Some(msg) = ws.try_next().await? {
                println!("{:?}", msg)
            }
        }
        Err(e) => println!("Error obtaining userstream: {}", e),
    }

    Ok(())
}
