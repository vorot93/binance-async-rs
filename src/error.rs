use serde::{Deserialize, Serialize};
use snafu::Snafu;

#[allow(clippy::pub_enum_variant_names)]
#[derive(Deserialize, Serialize, Debug, Clone, Snafu)]
pub enum Error {
    #[snafu(display("Binance error: {}: {}", code, msg))]
    BinanceError { code: i64, msg: String },
    #[snafu(display("Assets not found"))]
    AssetsNotFound,
    #[snafu(display("Symbol not found"))]
    SymbolNotFound,
    #[snafu(display("No Api key set for private api"))]
    NoApiKeySet,
    #[snafu(display("No stream is subscribed"))]
    NoStreamSubscribed,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BinanceErrorData {
    pub code: i64,
    pub msg: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum BinanceResponse<T> {
    Success(T),
    Error(BinanceErrorData),
}

impl<T: for<'a> Deserialize<'a>> BinanceResponse<T> {
    pub fn into_result(self) -> Result<T, Error> {
        match self {
            Self::Success(t) => Result::Ok(t),
            Self::Error(BinanceErrorData { code, msg }) => {
                Result::Err(Error::BinanceError { code, msg })
            }
        }
    }
}
