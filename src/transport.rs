use crate::error::{BinanceResponse, Error};
use chrono::Utc;
use failure::Fallible;
use futures::prelude::*;
use headers::*;
use hex::encode as hexify;
use hmac::{Hmac, Mac};
use http::Method;
use once_cell::sync::OnceCell;
use reqwest_ext::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{to_string, to_value, Value};
use sha2::Sha256;
use std::str::FromStr;
use tracing::*;
use url::Url;

const BASE: &str = "https://www.binance.com";
const RECV_WINDOW: usize = 5000;

pub struct BinanceApiKey(pub String);

impl headers::Header for BinanceApiKey {
    fn name() -> &'static HeaderName {
        static H: OnceCell<HeaderName> = OnceCell::new();

        H.get_or_init(|| HeaderName::from_str("X-MBX-APIKEY").unwrap())
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .next()
            .and_then(|v| v.to_str().map(ToString::to_string).ok())
            .map(Self)
            .ok_or_else(headers::Error::invalid)
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend(Some(self.0.parse().unwrap()));
    }
}

#[derive(Clone)]
pub struct Transport {
    credential: Option<(String, String)>,
    client: reqwest::Client,
    pub recv_window: usize,
}

impl Default for Transport {
    fn default() -> Self {
        Self::new()
    }
}

impl Transport {
    pub fn new() -> Self {
        Self {
            credential: None,
            client: reqwest::Client::builder().build().unwrap(),
            recv_window: RECV_WINDOW,
        }
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        Self {
            client: reqwest::Client::builder().build().unwrap(),
            credential: Some((api_key.into(), api_secret.into())),
            recv_window: RECV_WINDOW,
        }
    }

    pub fn get<O, Q>(
        &self,
        endpoint: &str,
        params: Option<Q>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        self.request::<_, _, ()>(Method::GET, endpoint, params, None)
    }

    pub fn post<O, D>(
        &self,
        endpoint: &str,
        data: Option<D>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        self.request::<_, (), _>(Method::POST, endpoint, None, data)
    }

    pub fn put<O, D>(
        &self,
        endpoint: &str,
        data: Option<D>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        self.request::<_, (), _>(Method::PUT, endpoint, None, data)
    }

    pub fn delete<O, Q>(
        &self,
        endpoint: &str,
        params: Option<Q>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        self.request::<_, _, ()>(Method::DELETE, endpoint, params, None)
    }

    pub fn signed_get<O, Q>(
        &self,
        endpoint: &str,
        params: Option<Q>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        self.signed_request::<_, _, ()>(Method::GET, endpoint, params, None)
    }

    pub fn signed_post<O, D>(
        &self,
        endpoint: &str,
        data: Option<D>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        self.signed_request::<_, (), _>(Method::POST, endpoint, None, data)
    }

    pub fn signed_put<O, Q>(
        &self,
        endpoint: &str,
        params: Option<Q>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        self.signed_request::<_, _, ()>(Method::PUT, endpoint, params, None)
    }

    pub fn signed_delete<O, Q>(
        &self,
        endpoint: &str,
        params: Option<Q>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        self.signed_request::<_, _, ()>(Method::DELETE, endpoint, params, None)
    }

    pub fn request<O, Q, D>(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<Q>,
        data: Option<D>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        Q: Serialize,
        D: Serialize,
    {
        let url = format!("{}{}", BASE, endpoint);
        let url = match params {
            Some(p) => Url::parse_with_params(&url, p.to_url_query())?,
            None => Url::parse(&url)?,
        };

        let body = match data {
            Some(data) => data.to_url_query_string(),
            None => "".to_string(),
        };

        let mut req = self
            .client
            .request(method, url.as_str())
            .typed_header(headers::UserAgent::from_static("binance-rs"))
            .typed_header(headers::ContentType::form_url_encoded());

        if let Ok((key, _)) = self.check_key() {
            // This is for user stream: user stream requests need api key in the header but no signature. WEIRD
            req = req.typed_header(BinanceApiKey(key.to_string()));
        }

        let req = req.body(body);

        Ok(async move {
            Ok(req
                .send()
                .await?
                .json::<BinanceResponse<_>>()
                .await?
                .into_result()?)
        })
    }

    pub fn signed_request<O, Q, D>(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<Q>,
        data: Option<D>,
    ) -> Fallible<impl Future<Output = Fallible<O>>>
    where
        O: DeserializeOwned,
        Q: Serialize,
        D: Serialize,
    {
        let query = params.map_or_else(Vec::new, |q| q.to_url_query());
        let url = format!("{}{}", BASE, endpoint);
        let mut url = Url::parse_with_params(&url, &query)?;
        url.query_pairs_mut()
            .append_pair("timestamp", &Utc::now().timestamp_millis().to_string());
        url.query_pairs_mut()
            .append_pair("recvWindow", &self.recv_window.to_string());

        let body = data.map_or_else(String::new, |data| data.to_url_query_string());

        let (key, signature) = self.signature(&url, &body)?;
        url.query_pairs_mut().append_pair("signature", &signature);

        let req = self
            .client
            .request(method, url.as_str())
            .typed_header(headers::UserAgent::from_static("binance-rs"))
            .typed_header(headers::ContentType::form_url_encoded())
            .typed_header(BinanceApiKey(key.to_string()))
            .body(body);

        Ok(async move {
            Ok(req
                .send()
                .await?
                .json::<BinanceResponse<_>>()
                .await?
                .into_result()?)
        })
    }

    fn check_key(&self) -> Fallible<(&str, &str)> {
        match self.credential.as_ref() {
            None => Err(Error::NoApiKeySet.into()),
            Some((k, s)) => Ok((k, s)),
        }
    }

    pub(self) fn signature(&self, url: &Url, body: &str) -> Fallible<(&str, String)> {
        let (key, secret) = self.check_key()?;
        // Signature: hex(HMAC_SHA256(queries + data))
        let mut mac = Hmac::<Sha256>::new_varkey(secret.as_bytes()).unwrap();
        let sign_message = format!("{}{}", url.query().unwrap_or(""), body);
        trace!("Sign message: {}", sign_message);
        mac.input(sign_message.as_bytes());
        let signature = hexify(mac.result().code());
        Ok((key, signature))
    }
}

trait ToUrlQuery: Serialize {
    fn to_url_query_string(&self) -> String {
        let vec = self.to_url_query();

        vec.into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&")
    }

    fn to_url_query(&self) -> Vec<(String, String)> {
        let v = to_value(self).unwrap();
        let v = v.as_object().unwrap();
        let mut vec = vec![];

        for (key, value) in v {
            match value {
                Value::Null => continue,
                Value::String(s) => vec.push((key.clone(), s.clone())),
                other => vec.push((key.clone(), to_string(other).unwrap())),
            }
        }

        vec
    }
}

impl<S: Serialize> ToUrlQuery for S {}

#[cfg(test)]
mod test {
    use super::Transport;
    use failure::Fallible;
    use url::{form_urlencoded::Serializer, Url};

    #[test]
    fn signature_query() -> Fallible<()> {
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
        assert_eq!(
            sig,
            "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71"
        );
        Ok(())
    }

    #[test]
    fn signature_body() -> Fallible<()> {
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
        assert_eq!(
            sig,
            "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71"
        );
        Ok(())
    }

    #[test]
    fn signature_query_body() -> Fallible<()> {
        let tr = Transport::with_credential(
            "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
            "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j",
        );

        let mut s = Serializer::new(String::new());
        s.extend_pairs(&[
            ("quantity", "1"),
            ("price", "0.1"),
            ("recvWindow", "5000"),
            ("timestamp", "1499827319559"),
        ]);

        let (_, sig) = tr.signature(
            &Url::parse_with_params(
                "http://a.com/api/v1/order",
                &[
                    ("symbol", "LTCBTC"),
                    ("side", "BUY"),
                    ("type", "LIMIT"),
                    ("timeInForce", "GTC"),
                ],
            )?,
            &s.finish(),
        )?;
        assert_eq!(
            sig,
            "0fd168b8ddb4876a0358a8d14d0c9f3da0e9b20c5d52b2a00fcf7d1c602f9a77"
        );
        Ok(())
    }

    #[test]
    fn signature_body2() -> Fallible<()> {
        let tr = Transport::with_credential(
            "vj1e6h50pFN9CsXT5nsL25JkTuBHkKw3zJhsA6OPtruIRalm20vTuXqF3htCZeWW",
            "5Cjj09rLKWNVe7fSalqgpilh5I3y6pPplhOukZChkusLqqi9mQyFk34kJJBTdlEJ",
        );

        let q = &mut [
            ("symbol", "ETHBTC"),
            ("side", "BUY"),
            ("type", "LIMIT"),
            ("timeInForce", "GTC"),
            ("quantity", "1"),
            ("price", "0.1"),
            ("recvWindow", "5000"),
            ("timestamp", "1540687064555"),
        ];
        q.sort();
        let q: Vec<_> = q.iter_mut().map(|(k, v)| format!("{}={}", k, v)).collect();
        let q = q.join("&");
        let (_, sig) = tr.signature(&Url::parse("http://a.com/api/v1/test")?, &q)?;
        assert_eq!(
            sig,
            "1ee5a75760b9496a2144a22116e02bc0b7fdcf828781fa87ca273540dfcf2cb0"
        );
        Ok(())
    }
}
