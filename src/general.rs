use serde_json::Value;

use client::*;
use errors::*;
use model::{ExchangeInfo, ExchangeInformation, ServerTime};

#[derive(Clone)]
pub struct General {
    pub client: Client,
}

impl General {
    // Test connectivity
    pub fn ping(&self) -> Result<(String)> {
        let _: Value = self.client.get("/api/v1/ping", None)?;

        Ok("pong".into())
    }

    // Check server time
    pub fn get_server_time(&self) -> Result<ServerTime> {
        Ok(self.client.get("/api/v1/time", None)?)
    }

    pub fn get_exchange_info(&self) -> Result<ExchangeInfo> {
        Ok(self.client.get("/api/v1/exchangeInfo", None)?)
    }

    // Obtain exchange information (rate limits, symbol metadata etc)
    pub fn exchange_info(&self) -> Result<ExchangeInformation> {
        let info: ExchangeInformation = self.client.get("/api/v1/exchangeInfo", None)?;
        Ok(info)
    }
}
