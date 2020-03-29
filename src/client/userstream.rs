use crate::{
    client::Binance,
    model::{Success, UserDataStream},
};
use failure::Fallible;
use futures::prelude::*;

const USER_DATA_STREAM: &str = "/api/v1/userDataStream";

impl Binance {
    // User Stream
    pub fn user_stream_start(&self) -> Fallible<impl Future<Output = Fallible<UserDataStream>>> {
        let user_data_stream = self.transport.post::<_, ()>(USER_DATA_STREAM, None)?;
        Ok(user_data_stream)
    }

    // Current open orders on a symbol
    pub fn user_stream_keep_alive(
        &self,
        listen_key: &str,
    ) -> Fallible<impl Future<Output = Fallible<Success>>> {
        let success = self.transport.put(
            USER_DATA_STREAM,
            Some(vec![("listen_key", listen_key.to_string())]),
        )?;
        Ok(success)
    }

    pub fn user_stream_close(
        &self,
        listen_key: &str,
    ) -> Fallible<impl Future<Output = Fallible<Success>>> {
        let success = self.transport.delete(
            USER_DATA_STREAM,
            Some(vec![("listen_key", listen_key.to_string())]),
        )?;
        Ok(success)
    }
}
