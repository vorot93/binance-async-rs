use std::result::Result as StdResult;

use failure::Error;
use serde::Deserialize;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Deserialize, Serialize, Debug, Clone, Fail)]
#[fail(display = "Binance returns error: {}", msg)]
pub struct BinanceResponseError {
    pub code: i64,
    pub msg: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum BinanceResponse<T> {
    Success(T),
    Error(BinanceResponseError),
}

impl<T: for<'a> Deserialize<'a>> BinanceResponse<T> {
    pub fn to_result(self) -> StdResult<T, BinanceResponseError> {
        match self {
            BinanceResponse::Success(t) => StdResult::Ok(t),
            BinanceResponse::Error(e) => StdResult::Err(e),
        }
    }
}

#[derive(Debug, Fail, Serialize, Deserialize, Clone)]
pub enum BinanceError {
    #[fail(display = "Assets not found")]
    AssetsNotFound,
    #[fail(display = "Symbol not found")]
    SymbolNotFound,
    #[fail(display = "Hand shake error: {}", _0)]
    HandShakeError(String),
}
