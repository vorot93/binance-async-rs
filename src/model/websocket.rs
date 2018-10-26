use super::{AccountUpdateEvent, DayTickerEvent, DepthOrderBookEvent, KlineEvent, OrderBook, OrderTradeEvent, TradesEvent};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Subscription {
    UserData(String),            // listen key
    AggregateTrade(String),      //symbol
    Trade(String),               //symbol
    Candlestick(String, String), //symbol, interval
    MiniTicker(String),          //symbol
    MiniTickerAll,
    Ticker(String), // symbol
    TickerAll,
    OrderBook(String, i64), //symbol, depth
    Depth(String),          //symbol
}

#[derive(Debug, Clone)]
pub enum BinanceWebsocketMessage {
    UserOrderUpdate(OrderTradeEvent),
    UserAccountUpdate(AccountUpdateEvent),
    AggregateTrade(TradesEvent),
    Trade,
    Candlestick(KlineEvent),
    MiniTicker,
    MiniTickerAll,
    Ticker(DayTickerEvent),
    TickerAll(Vec<DayTickerEvent>),
    OrderBook(OrderBook),
    Depth(DepthOrderBookEvent),
}
