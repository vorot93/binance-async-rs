//#![deny(unstable_features, unused_must_use, unused_mut, unused_imports, unused_import_braces)]
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

mod client;
pub mod error;
pub mod model;
mod transport;

pub use crate::client::Binance;
