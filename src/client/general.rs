use failure::Error;
use futures::Future;
use serde_json::Value;

use client::Binance;
use error::Result;
use model::{ExchangeInfo, ExchangeInformation, ServerTime};

impl Binance {
    // Test connectivity
    pub fn ping(&self) -> Result<impl Future<Item = String, Error = Error>> {
        Ok(self
            .transport
            .get::<_, ()>("/api/v1/ping", None)?
            .map(|_: Value| "pong".into()))
    }

    // Check server time
    pub fn get_server_time(&self) -> Result<impl Future<Item = ServerTime, Error = Error>> {
        Ok(self.transport.get::<_, ()>("/api/v1/time", None)?)
    }

    pub fn get_exchange_info(&self) -> Result<impl Future<Item = ExchangeInfo, Error = Error>> {
        Ok(self.transport.get::<_, ()>("/api/v1/exchangeInfo", None)?)
    }

    // Obtain exchange information (rate limits, symbol metadata etc)
    pub fn exchange_info(&self) -> Result<impl Future<Item = ExchangeInformation, Error = Error>> {
        let info = self.transport.get::<_, ()>("/api/v1/exchangeInfo", None)?;
        Ok(info)
    }
}
