extern crate binance_async as binance;
extern crate dotenv;
extern crate env_logger;
extern crate tokio;

use binance::error::Result;
use binance::Binance;
use tokio::runtime::Runtime;

#[test]
fn ping() -> Result<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    let mut rt = Runtime::new()?;
    let binance = Binance::new();

    let fut = binance.ping()?;

    let _ = rt.block_on(fut)?;
    Ok(())
}
