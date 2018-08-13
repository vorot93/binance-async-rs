use std::borrow::Borrow;
use std::collections::BTreeMap;

use chrono::Utc;
use failure::Error;
use futures::{Future, Stream};
use hex::encode as hexify;
use hmac::{Hmac, Mac};
use hyper::client::{HttpConnector, ResponseFuture};
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde_json::{from_slice, to_string, to_vec};
use sha2::Sha256;
use url::Url;

use error::{BinanceError, BinanceResponse, Result};

static BASE: &'static str = "https://www.binance.com";
static RECV_WINDOW: usize = 5000;

pub(crate) type Dummy = &'static [(&'static str, &'static str); 0];

#[derive(Clone)]
pub struct Transport {
    credential: Option<(String, String)>,
    client: Client<HttpsConnector<HttpConnector>>,
    pub recv_window: usize,
}

impl Transport {
    pub fn new() -> Self {
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, Body>(https);

        Transport {
            credential: None,
            client: client,
            recv_window: RECV_WINDOW,
        }
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, Body>(https);

        Transport {
            client: client,
            credential: Some((api_key.into(), api_secret.into())),
            recv_window: RECV_WINDOW,
        }
    }

    pub fn get<O: DeserializeOwned, I, K, V>(&self, endpoint: &str, params: Option<I>) -> Result<impl Future<Item = O, Error = Error>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.request::<_, _, Dummy, _, _, _, _>(Method::GET, endpoint, params, None)
    }

    pub fn post<O: DeserializeOwned, I, K, V>(&self, endpoint: &str, data: Option<I>) -> Result<impl Future<Item = O, Error = Error>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.request::<_, Dummy, _, _, _, _, _>(Method::GET, endpoint, None, data)
    }

    pub fn put<O: DeserializeOwned, I, K, V>(&self, endpoint: &str, data: Option<I>) -> Result<impl Future<Item = O, Error = Error>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.request::<_, Dummy, _, _, _, _, _>(Method::GET, endpoint, None, data)
    }
    pub fn signed_get<O: DeserializeOwned, I, K, V>(&self, endpoint: &str, params: Option<I>) -> Result<impl Future<Item = O, Error = Error>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.signed_request::<_, _, Dummy, _, _, _, _>(Method::GET, endpoint, params, None)
    }

    pub fn signed_post<O: DeserializeOwned, I, K, V>(&self, endpoint: &str, data: Option<I>) -> Result<impl Future<Item = O, Error = Error>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.signed_request::<_, Dummy, _, _, _, _, _>(Method::POST, endpoint, None, data)
    }

    pub fn signed_put<O: DeserializeOwned, I, K, V>(&self, endpoint: &str, params: Option<I>) -> Result<impl Future<Item = O, Error = Error>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.signed_request::<_, _, Dummy, _, _, _, _>(Method::PUT, endpoint, params, None)
    }

    pub fn signed_delete<O: DeserializeOwned, I, K, V>(&self, endpoint: &str, params: Option<I>) -> Result<impl Future<Item = O, Error = Error>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.signed_request::<_, _, Dummy, _, _, _, _>(Method::DELETE, endpoint, params, None)
    }

    pub fn request<O: DeserializeOwned, I, J, K1, V1, K2, V2>(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<I>,
        data: Option<J>,
    ) -> Result<impl Future<Item = O, Error = Error>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K1, V1)>,
        K1: AsRef<str>,
        V1: AsRef<str>,
        J: IntoIterator,
        J::Item: Borrow<(K2, V2)>,
        K2: AsRef<str>,
        V2: AsRef<str>,
    {
        let url = format!("{}{}", BASE, endpoint);
        let url = match params {
            Some(p) => Url::parse_with_params(&url, p)?,
            None => Url::parse(&url)?,
        };

        let body = match data {
            Some(data) => {
                let bt = data
                    .into_iter()
                    .map(|i| {
                        let (a, b) = i.borrow();
                        (a.as_ref().to_string(), b.as_ref().to_string())
                    })
                    .collect::<BTreeMap<_, _>>();
                Body::from(to_vec(&bt)?)
            }
            None => Body::empty(),
        };

        let req = Request::builder()
            .method(method)
            .uri(url.as_str())
            .header("user-agent", "binance-rs")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(body)?;
        Ok(self.handle_response(self.client.request(req)))
    }

    pub fn signed_request<O: DeserializeOwned, I, J, K1, V1, K2, V2>(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<I>,
        data: Option<J>,
    ) -> Result<impl Future<Item = O, Error = Error>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K1, V1)>,
        K1: AsRef<str>,
        V1: AsRef<str>,
        J: IntoIterator,
        J::Item: Borrow<(K2, V2)>,
        K2: AsRef<str>,
        V2: AsRef<str>,
    {
        let url = format!("{}{}", BASE, endpoint);
        let mut url = match params {
            Some(p) => Url::parse_with_params(&url, p)?,
            None => Url::parse(&url)?,
        };

        url.query_pairs_mut().append_pair("timestamp", &Utc::now().timestamp_millis().to_string());
        url.query_pairs_mut().append_pair("recvWindow", &self.recv_window.to_string());

        let body = match data {
            Some(data) => {
                let bt = data
                    .into_iter()
                    .map(|i| {
                        let (a, b) = i.borrow();
                        (a.as_ref().to_string(), b.as_ref().to_string())
                    })
                    .collect::<BTreeMap<_, _>>();
                to_string(&bt)?
            }
            None => "".to_string(),
        };

        let (key, signature) = self.signature(&url, &body)?;

        url.query_pairs_mut().append_pair("signature", &signature);

        let req = Request::builder()
            .method(method)
            .uri(url.as_str())
            .header("user-agent", "binance-rs")
            .header("X-MBX-APIKEY", key)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(body))?;

        Ok(self.handle_response(self.client.request(req)))
    }

    fn check_key(&self) -> Result<(&str, &str)> {
        match self.credential.as_ref() {
            None => Err(BinanceError::NoApiKeySet)?,
            Some((k, s)) => Ok((k, s)),
        }
    }

    pub(self) fn signature(&self, url: &Url, body: &str) -> Result<(&str, String)> {
        let (key, secret) = self.check_key()?;
        // Signature: hex(HMAC_SHA256(queries + data))
        let mut mac = Hmac::<Sha256>::new_varkey(secret.as_bytes()).unwrap();
        let sign_message = match url.query() {
            Some(query) => format!("{}{}", query, body),
            None => format!("{}", body),
        };
        println!("{}", sign_message);
        mac.input(sign_message.as_bytes());
        let signature = hexify(mac.result().code());
        Ok((key, signature))
    }

    fn handle_response<O: DeserializeOwned>(&self, fut: ResponseFuture) -> impl Future<Item = O, Error = Error> {
        fut.from_err::<Error>()
            .and_then(|resp| resp.into_body().concat2().from_err::<Error>())
            .map(|chunk| {
                trace!("{}", String::from_utf8_lossy(&*chunk));
                chunk
            })
            .and_then(|chunk| Ok(from_slice(&chunk)?))
            .and_then(|resp: BinanceResponse<O>| Ok(resp.to_result()?))
    }
}

#[cfg(test)]
mod test {
    use super::Transport;
    use error::Result;
    use url::form_urlencoded::Serializer;
    use url::Url;

    #[test]
    fn signature_query() -> Result<()> {
        let tr = Transport::with_credential(
            "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
            "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j",
        );
        let (_, sig) = tr.signature(
            &Url::parse_with_params(
                "http://a.com/api/v1/test",
                &[
                    ("symbol", "LTCBTC"),
                    ("side", "BUY"),
                    ("type", "LIMIT"),
                    ("timeInForce", "GTC"),
                    ("quantity", "1"),
                    ("price", "0.1"),
                    ("recvWindow", "5000"),
                    ("timestamp", "1499827319559"),
                ],
            )?,
            "",
        )?;
        assert_eq!(sig, "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71");
        Ok(())
    }

    #[test]
    fn signature_body() -> Result<()> {
        let tr = Transport::with_credential(
            "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
            "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j",
        );
        let mut s = Serializer::new(String::new());
        s.extend_pairs(&[
            ("symbol", "LTCBTC"),
            ("side", "BUY"),
            ("type", "LIMIT"),
            ("timeInForce", "GTC"),
            ("quantity", "1"),
            ("price", "0.1"),
            ("recvWindow", "5000"),
            ("timestamp", "1499827319559"),
        ]);

        let (_, sig) = tr.signature(&Url::parse("http://a.com/api/v1/test")?, &s.finish())?;
        assert_eq!(sig, "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71");
        Ok(())
    }

    #[test]
    fn signature_query_body() -> Result<()> {
        let tr = Transport::with_credential(
            "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
            "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j",
        );

        let mut s = Serializer::new(String::new());
        s.extend_pairs(&[("quantity", "1"), ("price", "0.1"), ("recvWindow", "5000"), ("timestamp", "1499827319559")]);

        let (_, sig) = tr.signature(
            &Url::parse_with_params(
                "http://a.com/api/v1/order",
                &[("symbol", "LTCBTC"), ("side", "BUY"), ("type", "LIMIT"), ("timeInForce", "GTC")],
            )?,
            &s.finish(),
        )?;
        assert_eq!(sig, "0fd168b8ddb4876a0358a8d14d0c9f3da0e9b20c5d52b2a00fcf7d1c602f9a77");
        Ok(())
    }
}
