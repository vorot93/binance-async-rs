use client::{Binance, UserStream};
use errors::Result;
use model::{Success, UserDataStream};

static USER_DATA_STREAM: &'static str = "/api/v1/userDataStream";

impl Binance<UserStream> {
    // User Stream
    pub fn start(&self) -> Result<UserDataStream> {
        let user_data_stream: UserDataStream = self.transport.post(USER_DATA_STREAM)?;

        Ok(user_data_stream)
    }

    // Current open orders on a symbol
    pub fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        let success: Success = self.transport.put(USER_DATA_STREAM, listen_key)?;

        Ok(success)
    }

    pub fn close(&self, listen_key: &str) -> Result<Success> {
        let success: Success = self.transport.put(USER_DATA_STREAM, listen_key)?;

        Ok(success)
    }
}
