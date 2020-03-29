//#![deny(unstable_features, unused_must_use, unused_mut, unused_imports, unused_import_braces)]

#[macro_use]
extern crate tracing;

mod client;
pub mod error;
pub mod model;
mod transport;

pub use crate::client::Binance;
