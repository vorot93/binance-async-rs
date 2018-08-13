use failure::Error;
use futures::Future;

use client::Binance;
use error::Result;
use model::{Success, UserDataStream};
use transport::Dummy;

static USER_DATA_STREAM: &'static str = "/api/v1/userDataStream";

impl Binance {
    // User Stream
    pub fn start(&self) -> Result<impl Future<Item = UserDataStream, Error = Error>> {
        let user_data_stream = self.transport.signed_post::<_, Dummy, _, _>(USER_DATA_STREAM, None)?;
        Ok(user_data_stream)
    }

    // Current open orders on a symbol
    pub fn keep_alive(&self, listen_key: &str) -> Result<impl Future<Item = Success, Error = Error>> {
        let success = self.transport.signed_put(USER_DATA_STREAM, Some(vec![("listen_key", listen_key.to_string())]))?;
        Ok(success)
    }

    pub fn close(&self, listen_key: &str) -> Result<impl Future<Item = Success, Error = Error>> {
        let success = self.transport.signed_delete(USER_DATA_STREAM, Some(vec![("listen_key", listen_key.to_string())]))?;
        Ok(success)
    }
}
