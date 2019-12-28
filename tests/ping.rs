use binance_async as binance;

use failure::Fallible;

use crate::binance::Binance;

#[tokio::test]
async fn ping() -> Fallible<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();

    let binance = Binance::new();

    binance.ping()?.await?;

    Ok(())
}
