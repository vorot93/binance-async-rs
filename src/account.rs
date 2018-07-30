use std::collections::BTreeMap;

use client::{Account, Binance};
use errors::{BinanceError, Result};
use model::{AccountInformation, Balance, Order, OrderCanceled, TradeHistory, Transaction};
use util::build_signed_request;

static ORDER_TYPE_LIMIT: &'static str = "LIMIT";
static ORDER_TYPE_MARKET: &'static str = "MARKET";
static ORDER_SIDE_BUY: &'static str = "BUY";
static ORDER_SIDE_SELL: &'static str = "SELL";
static TIME_IN_FORCE_GTC: &'static str = "GTC";

static API_V3_ORDER: &'static str = "/api/v3/order";

struct OrderRequest {
    pub symbol: String,
    pub qty: f64,
    pub price: f64,
    pub order_side: String,
    pub order_type: String,
    pub time_in_force: String,
}

impl Binance<Account> {
    // Account Information
    pub fn get_account(&self) -> Result<AccountInformation> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        let account_info: AccountInformation = self.transport.get_signed("/api/v3/account", &request)?;

        Ok(account_info)
    }

    // Balance for ONE Asset
    pub fn get_balance<S>(&self, asset: S) -> Result<Balance>
    where
        S: Into<String>,
    {
        match self.get_account() {
            Ok(account) => {
                let cmp_asset = asset.into();
                for balance in account.balances {
                    if balance.asset == cmp_asset {
                        return Ok(balance);
                    }
                }
                Err(BinanceError::AssetsNotFound)?
            }
            Err(e) => Err(e),
        }
    }

    // Current open orders for ONE symbol
    pub fn get_open_orders<S>(&self, symbol: S) -> Result<Vec<Order>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        let orders: Vec<Order> = self.transport.get_signed("/api/v3/openOrders", &request)?;
        Ok(orders)
    }

    // All current open orders
    pub fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        let orders: Vec<Order> = self.transport.get_signed("/api/v3/openOrders", &request)?;
        Ok(orders)
    }

    // Check an order's status
    pub fn order_status<S>(&self, symbol: S, order_id: u64) -> Result<Order>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        let order: Order = self.transport.get_signed(API_V3_ORDER, &request)?;
        Ok(order)
    }

    // Place a LIMIT order - BUY
    pub fn limit_buy<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: price,
            order_side: ORDER_SIDE_BUY.to_string(),
            order_type: ORDER_TYPE_LIMIT.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        let transaction: Transaction = self.transport.post_signed(API_V3_ORDER, &request)?;

        Ok(transaction)
    }

    // Place a LIMIT order - SELL
    pub fn limit_sell<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: price,
            order_side: ORDER_SIDE_SELL.to_string(),
            order_type: ORDER_TYPE_LIMIT.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        let transaction: Transaction = self.transport.post_signed(API_V3_ORDER, &request)?;

        Ok(transaction)
    }

    // Place a MARKET order - BUY
    pub fn market_buy<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            order_side: ORDER_SIDE_BUY.to_string(),
            order_type: ORDER_TYPE_MARKET.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        let transaction: Transaction = self.transport.post_signed(API_V3_ORDER, &request)?;

        Ok(transaction)
    }

    // Place a MARKET order - SELL
    pub fn market_sell<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            order_side: ORDER_SIDE_SELL.to_string(),
            order_type: ORDER_TYPE_MARKET.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        let transaction: Transaction = self.transport.post_signed(API_V3_ORDER, &request)?;

        Ok(transaction)
    }

    // Check an order's status
    pub fn cancel_order<S>(&self, symbol: S, order_id: u64) -> Result<OrderCanceled>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        let order_canceled: OrderCanceled = self.transport.delete_signed(API_V3_ORDER, &request)?;

        Ok(order_canceled)
    }

    // Trade history
    pub fn trade_history<S>(&self, symbol: S) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        let trade_history: Vec<TradeHistory> = self.transport.get_signed("/api/v3/myTrades", &request)?;

        Ok(trade_history)
    }

    fn build_order(&self, order: OrderRequest) -> BTreeMap<String, String> {
        let mut parameters: BTreeMap<String, String> = convert_args! {
            btreemap! (
                "symbol" => order.symbol,
                "side" => order.order_side,
                "type" => order.order_type,
                "quantity" => order.qty.to_string(),
            )
        };

        if order.price != 0.0 {
            parameters.insert("price".into(), order.price.to_string());
            parameters.insert("timeInForce".into(), order.time_in_force);
        }

        parameters
    }
}
