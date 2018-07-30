use client::*;
use errors::*;
use model::*;

static USER_DATA_STREAM: &'static str = "/api/v1/userDataStream";

#[derive(Clone)]
pub struct UserStream {
    pub client: Client,
    pub recv_window: u64,
}

impl UserStream {
    // User Stream
    pub fn start(&self) -> Result<UserDataStream> {
        let user_data_stream: UserDataStream = self.client.post(USER_DATA_STREAM)?;

        Ok(user_data_stream)
    }

    // Current open orders on a symbol
    pub fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        let success: Success = self.client.put(USER_DATA_STREAM, listen_key)?;

        Ok(success)
    }

    pub fn close(&self, listen_key: &str) -> Result<Success> {
        let success: Success = self.client.put(USER_DATA_STREAM, listen_key)?;

        Ok(success)
    }
}
