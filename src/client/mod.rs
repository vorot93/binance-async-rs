mod account;
mod general;
mod market;
mod userstream;
pub mod websocket;

use crate::transport::Transport;

#[derive(Clone, Default)]
pub struct Binance {
    pub transport: Transport,
}

impl Binance {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        Self {
            transport: Transport::with_credential(api_key, api_secret),
        }
    }
}
