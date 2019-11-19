use binance_async as binance;

use failure::Fallible;
use std::env::var;

use crate::binance::Binance;

#[tokio::test]
async fn get_account() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    let binance = Binance::with_credential(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);

    binance.get_account()?.await?;

    Ok(())
}
