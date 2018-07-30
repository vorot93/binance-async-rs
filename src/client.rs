// use account::*;
// use general::*;
// use market::*;
use transport::{ApiKeyAuth, Transport};
// use userstream::*;

pub trait Scope {
    type Auth;
}

impl Scope for General {
    type Auth = ();
}

impl Scope for Account {
    type Auth = ApiKeyAuth;
}

impl Scope for Market {
    type Auth = ();
}

impl Scope for UserStream {
    type Auth = ApiKeyAuth;
}

pub enum General {}
pub enum Account {}
pub enum Market {}
pub enum UserStream {}

pub struct Binance<S: Scope> {
    pub transport: Transport<S::Auth>,
    pub recv_window: u64,
}

impl Binance<General> {
    pub fn new() -> Self {
        Binance {
            transport: Transport::<()>::new(),
            recv_window: 5000,
        }
    }
}

impl Binance<Account> {
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        Binance {
            transport: Transport::<ApiKeyAuth>::new(api_key, secret_key),
            recv_window: 5000,
        }
    }
}

impl Binance<Market> {
    pub fn new() -> Self {
        Binance {
            transport: Transport::<()>::new(),
            recv_window: 5000,
        }
    }
}

impl Binance<UserStream> {
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        Binance {
            transport: Transport::<ApiKeyAuth>::new(api_key, secret_key),
            recv_window: 5000,
        }
    }
}
