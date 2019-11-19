mod account;
mod general;
mod market;
mod userstream;
mod websocket;

use crate::transport::Transport;

#[derive(Clone, Default)]
pub struct Binance {
    pub transport: Transport,
}

impl Binance {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        Binance {
            transport: Transport::with_credential(api_key, api_secret),
        }
    }
}
