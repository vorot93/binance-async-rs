use crate::{
    error::Error,
    model::websocket::{AccountUpdate, BinanceWebsocketMessage, Subscription, UserOrderUpdate},
};
use failure::Fallible;
use futures::{prelude::*, stream::SplitStream};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, Poll},
};
use streamunordered::{StreamUnordered, StreamYield};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::*;
use tungstenite::Message;
use url::Url;

const WS_URL: &str = "wss://stream.binance.com:9443/ws";

#[allow(dead_code)]
type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub type StoredStream = SplitStream<WSStream>;

#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct BinanceWebsocket {
    subscriptions: HashMap<Subscription, usize>,
    tokens: HashMap<usize, Subscription>,
    streams: StreamUnordered<StoredStream>,
}

impl BinanceWebsocket {
    pub async fn subscribe(&mut self, subscription: Subscription) -> Fallible<()> {
        let sub = match subscription {
            Subscription::AggregateTrade(ref symbol) => format!("{}@aggTrade", symbol),
            Subscription::Candlestick(ref symbol, ref interval) => {
                format!("{}@kline_{}", symbol, interval)
            }
            Subscription::Depth(ref symbol) => format!("{}@depth", symbol),
            Subscription::MiniTicker(ref symbol) => format!("{}@miniTicker", symbol),
            Subscription::MiniTickerAll => "!miniTicker@arr".to_string(),
            Subscription::OrderBook(ref symbol, depth) => format!("{}@depth{}", symbol, depth),
            Subscription::Ticker(ref symbol) => format!("{}@ticker", symbol),
            Subscription::TickerAll => "!ticker@arr".to_string(),
            Subscription::Trade(ref symbol) => format!("{}@trade", symbol),
            Subscription::UserData(ref key) => key.clone(),
        };

        trace!("[Websocket] Subscribing to '{:?}'", subscription);

        let endpoint = Url::parse(&format!("{}/{}", WS_URL, sub)).unwrap();

        let token = self
            .streams
            .push(connect_async(endpoint).await?.0.split().1);

        self.subscriptions.insert(subscription.clone(), token);
        self.tokens.insert(token, subscription);
        Ok(())
    }

    pub fn unsubscribe(&mut self, subscription: &Subscription) -> Option<StoredStream> {
        let streams = Pin::new(&mut self.streams);
        self.subscriptions
            .get(subscription)
            .and_then(|token| StreamUnordered::take(streams, *token))
    }
}

impl Stream for BinanceWebsocket {
    type Item = Fallible<BinanceWebsocketMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.as_mut().get_mut().streams).poll_next(cx) {
            Poll::Ready(Some((y, token))) => match y {
                StreamYield::Item(item) => {
                    let sub = self.tokens.get(&token).unwrap();
                    Poll::Ready({
                        Some(
                            item.map_err(failure::Error::from)
                                .and_then(|m| parse_message(sub, m)),
                        )
                    })
                }
                StreamYield::Finished(_) => Poll::Pending,
            },
            Poll::Ready(None) => Poll::Ready(Some(Err(Error::NoStreamSubscribed.into()))),
            Poll::Pending => Poll::Pending,
        }
    }
}

fn parse_message(sub: &Subscription, msg: Message) -> Fallible<BinanceWebsocketMessage> {
    let msg = match msg {
        Message::Text(msg) => msg,
        Message::Binary(b) => return Ok(BinanceWebsocketMessage::Binary(b)),
        Message::Pong(..) => return Ok(BinanceWebsocketMessage::Pong),
        Message::Ping(..) => return Ok(BinanceWebsocketMessage::Ping),
        Message::Close(..) => return Err(failure::format_err!("Socket closed")),
    };

    trace!("Incoming websocket message {}", msg);
    let message = match sub {
        Subscription::AggregateTrade(..) => {
            BinanceWebsocketMessage::AggregateTrade(from_str(&msg)?)
        }
        Subscription::Candlestick(..) => BinanceWebsocketMessage::Candlestick(from_str(&msg)?),
        Subscription::Depth(..) => BinanceWebsocketMessage::Depth(from_str(&msg)?),
        Subscription::MiniTicker(..) => BinanceWebsocketMessage::MiniTicker(from_str(&msg)?),
        Subscription::MiniTickerAll => BinanceWebsocketMessage::MiniTickerAll(from_str(&msg)?),
        Subscription::OrderBook(..) => BinanceWebsocketMessage::OrderBook(from_str(&msg)?),
        Subscription::Ticker(..) => BinanceWebsocketMessage::Ticker(from_str(&msg)?),
        Subscription::TickerAll => BinanceWebsocketMessage::TickerAll(from_str(&msg)?),
        Subscription::Trade(..) => BinanceWebsocketMessage::Trade(from_str(&msg)?),
        Subscription::UserData(..) => {
            let msg: Either<AccountUpdate, UserOrderUpdate> = from_str(&msg)?;
            match msg {
                Either::Left(m) => BinanceWebsocketMessage::UserAccountUpdate(m),
                Either::Right(m) => BinanceWebsocketMessage::UserOrderUpdate(m),
            }
        }
    };
    Ok(message)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}
