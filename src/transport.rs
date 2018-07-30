use serde::Deserialize;
use serde_json::from_reader;

use errors::{BinanceResponse, Result};
use hex::encode as hex_encode;
use reqwest::header::{ContentType, Headers, UserAgent};
use reqwest::{Client, Response};
use ring::{digest, hmac};

static API1_HOST: &'static str = "https://www.binance.com";

#[derive(Clone)]
pub struct ApiKeyAuth {
    api_key: String,
    secret_key: String,
}

#[derive(Clone)]
pub struct Transport<Auth = ()> {
    auth: Auth,
    client: Client,
}

impl Transport<()> {
    pub fn new() -> Self {
        Transport { auth: (), client: Client::new() }
    }
}

impl Transport<ApiKeyAuth> {
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        Transport {
            auth: ApiKeyAuth {
                api_key: api_key.into(),
                secret_key: secret_key.into(),
            },
            client: Client::new(),
        }
    }
}

impl<A> Transport<A> {
    pub fn get<'a, T>(&self, endpoint: &str, request: impl Into<Option<&'a str>>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut url: String = format!("{}{}", API1_HOST, endpoint);
        for query in request.into() {
            url.push_str(&format!("?{}", query));
        }

        let response = self.client.get(&url).send()?;

        self.handle_response(response)
    }

    pub fn post<T>(&self, endpoint: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url: String = format!("{}{}", API1_HOST, endpoint);

        let response = self.client.post(&url).headers(self.build_headers()).send()?;

        self.handle_response(response)
    }

    pub fn put<T>(&self, endpoint: &str, listen_key: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url: String = format!("{}{}", API1_HOST, endpoint);
        let data: String = format!("listenKey={}", listen_key);
        let response = self.client.put(&url).headers(self.build_headers()).body(data).send()?;

        self.handle_response(response)
    }

    pub fn delete<T>(&self, endpoint: &str, listen_key: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url: String = format!("{}{}", API1_HOST, endpoint);
        let data: String = format!("listenKey={}", listen_key);
        let response = self.client.delete(url.as_str()).headers(self.build_headers()).body(data).send()?;

        self.handle_response(response)
    }

    fn build_headers(&self) -> Headers {
        let mut custon_headers = Headers::new();

        custon_headers.set(UserAgent::new("binance-rs"));
        // custon_headers.set_raw("X-MBX-APIKEY", self.api_key.as_str());
        custon_headers
    }

    fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let ret: BinanceResponse<T> = from_reader(response)?; // This line handles network errors
        Ok(ret.to_result()?) // This line handles binance errors
    }
}

impl Transport<ApiKeyAuth> {
    pub fn get_signed<'a, T>(&self, endpoint: &str, request: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = self.sign_request(endpoint, request);
        let response = self.client.get(&url).headers(self.build_signed_headers()).send()?;

        self.handle_response(response)
    }

    pub fn post_signed<'a, T>(&self, endpoint: &str, request: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = self.sign_request(endpoint, request);
        let response = self.client.post(&url).headers(self.build_signed_headers()).send()?;

        self.handle_response(response)
    }

    pub fn delete_signed<'a, T>(&self, endpoint: &str, request: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = self.sign_request(endpoint, request);
        let response = self.client.delete(&url).headers(self.build_signed_headers()).send()?;

        self.handle_response(response)
    }

    fn build_signed_headers(&self) -> Headers {
        let mut custon_headers = Headers::new();

        custon_headers.set(UserAgent::new("binance-rs"));
        custon_headers.set(ContentType::form_url_encoded());
        custon_headers.set_raw("X-MBX-APIKEY", self.auth.api_key.as_str());
        custon_headers
    }

    // Request must be signed
    fn sign_request(&self, endpoint: &str, request: &str) -> String {
        let signed_key = hmac::SigningKey::new(&digest::SHA256, self.auth.secret_key.as_bytes());
        let signature = hex_encode(hmac::sign(&signed_key, request.as_bytes()).as_ref());

        let request_body: String = format!("{}&signature={}", request, signature);
        let url: String = format!("{}{}?{}", API1_HOST, endpoint, request_body);

        url
    }
}
