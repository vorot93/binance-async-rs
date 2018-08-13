extern crate binance_async as binance;
extern crate dotenv;
extern crate env_logger;
extern crate tokio;

use std::env::var;

use binance::error::Result;
use binance::Binance;
use tokio::runtime::Runtime;

#[test]
fn get_account() -> Result<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    let mut rt = Runtime::new()?;

    let binance = Binance::with_credential(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);
    let fut = binance.get_account()?;

    let _ = rt.block_on(fut)?;
    Ok(())
}
