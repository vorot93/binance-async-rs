#![deny(unstable_features, unused_must_use, unused_mut, unused_imports, unused_import_braces)]
#[macro_use]
extern crate failure;
extern crate hex;
extern crate hmac;
extern crate hyper;
extern crate serde;
extern crate sha2;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate futures;
extern crate hyper_tls;
extern crate tokio;
extern crate tungstenite;
extern crate url;

mod client;
pub mod error;
pub mod model;
mod transport;
pub mod websockets;

pub use client::Binance;
