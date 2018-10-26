use std::collections::HashMap;

use failure::Error;
use futures::stream::{SplitStream, Stream};
use futures::{Future, Poll};
use serde_json::from_str;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;
use url::Url;

use crate::client::Binance;
use crate::error::{BinanceError, Result};
use crate::model::websocket::{BinanceWebsocketMessage, Subscription};
use crate::model::{AccountUpdateEvent, OrderTradeEvent};

const WS_URL: &'static str = "ws://localhost:9443/ws";

impl Binance {
    pub fn websocket(&self) -> BinanceWebsocket {
        BinanceWebsocket { subscriptions: HashMap::new() }
    }
}

#[allow(dead_code)]
type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct BinanceWebsocket {
    subscriptions: HashMap<Subscription, SplitStream<WSStream>>,
}

impl BinanceWebsocket {
    pub fn subscribe(mut self, subscription: Subscription) -> impl Future<Item = Self, Error = Error> {
        let sub = match subscription {
            Subscription::AggregateTrade(ref symbol) => format!("{}@aggTrade", symbol),
            Subscription::Candlestick(ref symbol, ref interval) => format!("{}@kline_{}", symbol, interval),
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
        connect_async(endpoint)
            .map(|(stream, _)| stream)
            .map(|s| s.split().1)
            .map(|stream| {
                self.subscriptions.insert(subscription, stream);
                self
            }).from_err()
    }

    pub fn unsubscribe(&mut self, subscription: &Subscription) -> Option<SplitStream<WSStream>> {
        self.subscriptions.remove(subscription)
    }
}

impl Stream for BinanceWebsocket {
    type Item = BinanceWebsocketMessage;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let streams: Vec<_> = self
            .subscriptions
            .iter_mut()
            .map(|(sub, stream)| stream.from_err().and_then(move |msg| parse_message(sub.clone(), msg)))
            .collect();

        let streams = streams
            .into_iter()
            .fold(None, |acc: Option<Box<Stream<Item = BinanceWebsocketMessage, Error = Error>>>, elem| match acc {
                Some(stream) => Some(Box::new(stream.select(elem.from_err()))),
                None => Some(Box::new(elem.from_err())),
            });
        match streams {
            Some(mut streams) => streams.poll(),
            None => Err(BinanceError::NoStreamSubscribed)?,
        }
    }
}

fn parse_message(sub: Subscription, msg: Message) -> Result<BinanceWebsocketMessage> {
    let msg = if let Message::Text(msg) = msg { msg } else { unimplemented!() };

    let message = match sub {
        Subscription::Ticker(_) => BinanceWebsocketMessage::Ticker(from_str(&msg)?),
        Subscription::Depth(_) => BinanceWebsocketMessage::Depth(from_str(&msg)?),
        Subscription::AggregateTrade(_) => BinanceWebsocketMessage::AggregateTrade(from_str(&msg)?),
        Subscription::Candlestick(..) => BinanceWebsocketMessage::Candlestick(from_str(&msg)?),
        Subscription::OrderBook(..) => BinanceWebsocketMessage::OrderBook(from_str(&msg)?),
        Subscription::TickerAll => BinanceWebsocketMessage::TickerAll(from_str(&msg)?),
        Subscription::UserData(_) => {
            let msg: Either<AccountUpdateEvent, OrderTradeEvent> = from_str(&msg)?;
            match msg {
                Either::Left(m) => BinanceWebsocketMessage::UserAccountUpdate(m),
                Either::Right(m) => BinanceWebsocketMessage::UserOrderUpdate(m),
            }
        }
        _ => unimplemented!(),
    };
    Ok(message)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}
