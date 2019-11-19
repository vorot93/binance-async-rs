use failure::Fallible;
use futures01::Future;
use serde_json::Value;

use crate::client::Binance;
use crate::model::{ExchangeInfo, ExchangeInformation, ServerTime};

impl Binance {
    // Test connectivity
    pub fn ping(&self) -> Fallible<impl Future<Item = String, Error = failure::Error>> {
        Ok(self
            .transport
            .get::<_, ()>("/api/v1/ping", None)?
            .map(|_: Value| "pong".into()))
    }

    // Check server time
    pub fn get_server_time(
        &self,
    ) -> Fallible<impl Future<Item = ServerTime, Error = failure::Error>> {
        Ok(self.transport.get::<_, ()>("/api/v1/time", None)?)
    }

    pub fn get_exchange_info(
        &self,
    ) -> Fallible<impl Future<Item = ExchangeInfo, Error = failure::Error>> {
        Ok(self.transport.get::<_, ()>("/api/v1/exchangeInfo", None)?)
    }

    // Obtain exchange information (rate limits, symbol metadata etc)
    pub fn exchange_info(
        &self,
    ) -> Fallible<impl Future<Item = ExchangeInformation, Error = failure::Error>> {
        let info = self.transport.get::<_, ()>("/api/v1/exchangeInfo", None)?;
        Ok(info)
    }
}
