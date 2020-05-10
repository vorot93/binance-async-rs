use crate::{
    client::Binance,
    model::{ExchangeInfo, ExchangeInformation, ServerTime},
};
use failure::Fallible;
use futures::prelude::*;
use serde_json::Value;

impl Binance {
    // Test connectivity
    pub fn ping(&self) -> Fallible<impl Future<Output = Fallible<String>>> {
        Ok(self
            .transport
            .get::<_, ()>("/api/v1/ping", None)?
            .map_ok(|_: Value| "pong".into()))
    }

    // Check server time
    pub fn get_server_time(&self) -> Fallible<impl Future<Output = Fallible<ServerTime>>> {
        Ok(self.transport.get::<_, ()>("/api/v1/time", None)?)
    }

    pub fn get_exchange_info(&self) -> Fallible<impl Future<Output = Fallible<ExchangeInfo>>> {
        Ok(self.transport.get::<_, ()>("/api/v3/exchangeInfo", None)?)
    }

    // Obtain exchange information (rate limits, symbol metadata etc)
    pub fn exchange_info(&self) -> Fallible<impl Future<Output = Fallible<ExchangeInformation>>> {
        let info = self.transport.get::<_, ()>("/api/v3/exchangeInfo", None)?;
        Ok(info)
    }
}
