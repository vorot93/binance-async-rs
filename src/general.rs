use serde_json::Value;

use client::{Binance, General};
use errors::Result;
use model::{ExchangeInfo, ExchangeInformation, ServerTime};

impl Binance<General> {
    // Test connectivity
    pub fn ping(&self) -> Result<String> {
        let _: Value = self.transport.get("/api/v1/ping", None)?;

        Ok("pong".into())
    }

    // Check server time
    pub fn get_server_time(&self) -> Result<ServerTime> {
        Ok(self.transport.get("/api/v1/time", None)?)
    }

    pub fn get_exchange_info(&self) -> Result<ExchangeInfo> {
        Ok(self.transport.get("/api/v1/exchangeInfo", None)?)
    }

    // Obtain exchange information (rate limits, symbol metadata etc)
    pub fn exchange_info(&self) -> Result<ExchangeInformation> {
        let info: ExchangeInformation = self.transport.get("/api/v1/exchangeInfo", None)?;
        Ok(info)
    }
}
