use futures::future::result;
use futures::{Future, Stream};
use serde_json;
use url::form_urlencoded;
use url::Url;
use reqwest;
use reqwest::unstable::async;
use tokio_core::reactor::Handle;

use std::mem;

use error::Error;
use future::PushoverFuture;
use requests::{Request, Response};

pub const API_URL: &'static str = "https://api.pushover.net";
pub const API_VERSION: &'static str = "1";

pub struct AsyncAPI {
    base_url: String,
    client: async::Client
}

impl AsyncAPI {
    pub fn new(handle: &Handle) -> Self {
        AsyncAPI {
            base_url: API_URL.to_owned(),
            client: async::Client::new(handle)
        }
    }

    pub fn send<R: Request>(&self, request: &R) -> PushoverFuture<<R as Request>::ResponseType> {
        let mut url = Url::parse(&self.base_url).unwrap();
        url.set_path(API_VERSION);

        request.build_url(&mut url);

        let mut req = self.client.request(request.get_method(), url);

        if let Some(params) = request.get_form_parameters() {
            let encoded: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(params.into_iter())
                .finish();

            req.body(encoded);
        }

        let response = req.send()
            .map_err(From::from)
            .and_then(|mut res| {
                let body = mem::replace(res.body_mut(), async::Decoder::empty());
                body.concat2().map_err(Into::into)
            });

        let future = response.and_then(|bytes| {
            result(serde_json::from_slice(&bytes)
                       .map_err(From::from)
                       .and_then(|value| match value {
                                     Response::Success::<R>(raw) => Ok(R::map(raw)), 
                                     Response::Error::<R>(err) => Err(err.into()),                        
                                 }))
        });

        PushoverFuture::new(Box::new(future))
    }
}

pub struct SyncAPI {
    base_url: String,
    client: reqwest::Client
}

impl SyncAPI {
    pub fn new() -> Self {
        SyncAPI {
            base_url: API_URL.to_owned(),
            client: reqwest::Client::new()
        }
    }

    pub fn send<R: Request>(&self, request: &R) -> Result<<R as Request>::ResponseType, Error> {
        let mut url = Url::parse(&self.base_url).unwrap();
        url.set_path(API_VERSION);

        request.build_url(&mut url);

        let mut req = self.client.request(request.get_method(), url);

        if let Some(params) = request.get_form_parameters() {
            let encoded: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(params.into_iter())
                .finish();

            req.body(encoded);
        }

        let mut res = req.send()?;

        res.json()
            .map_err(From::from)
            .and_then(|value| match value {
                Response::Success::<R>(raw) => Ok(R::map(raw)), 
                Response::Error::<R>(err) => Err(err.into())
        })

    }
}