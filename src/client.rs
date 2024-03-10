use url::form_urlencoded;
use url::Url;

use std::time::Duration;

use crate::error::Error;
use crate::requests::{Request, Response};

pub const API_URL: &str = "https://api.pushover.net";
pub const API_VERSION: &str = "1";
const DEFAULT_TIMEOUT: u64 = 30;

pub struct API {
    base_url: String,
    timeout: Duration,
}

impl Default for API {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(DEFAULT_TIMEOUT),
            base_url: API_URL.to_owned(),
        }
    }
}

impl API {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = url.to_owned();
        self
    }

    pub fn send<R: Request>(&self, request: &R) -> Result<<R as Request>::ResponseType, Error> {
        let mut url = Url::parse(&self.base_url).unwrap();
        url.set_path(API_VERSION);

        let client_builder = reqwest::blocking::ClientBuilder::new().timeout(self.timeout);

        request.build_url(&mut url);

        let req = client_builder.build()?.request(request.get_method(), url);

        let req = if let Some(params) = request.get_form_parameters() {
            let encoded: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(params)
                .finish();

            req.body(encoded)
        } else {
            req
        };

        let res = req.send()?;

        res.json()
            .map_err(From::from)
            .and_then(|value| match value {
                Response::Success::<R>(raw) => Ok(R::map(raw)),
                Response::Error::<R>(err) => Err(err.into()),
            })
    }

    pub async fn send_async<R: Request>(
        &self,
        request: &R,
    ) -> Result<<R as Request>::ResponseType, Error> {
        let mut url = Url::parse(&self.base_url).unwrap();
        url.set_path(API_VERSION);

        let client_builder = reqwest::ClientBuilder::new().timeout(self.timeout);

        request.build_url(&mut url);

        let req = client_builder.build()?.request(request.get_method(), url);

        let req = if let Some(params) = request.get_form_parameters() {
            let encoded: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(params.into_iter())
                .finish();

            req.body(encoded)
        } else {
            req
        };

        let res = req.send().await?;

        res.json()
            .await
            .map_err(From::from)
            .and_then(|value| match value {
                Response::Success::<R>(raw) => Ok(R::map(raw)),
                Response::Error::<R>(err) => Err(err.into()),
            })
    }
}
