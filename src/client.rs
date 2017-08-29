use hyper::Client;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::{Core, Handle, Timeout};
use futures::future::result;
use futures::{Future, Stream};
use serde_json;
use url::form_urlencoded;
use url::Url;

use std::cell::RefCell;
use std::time::Duration;

use error::Error;
use future::PushoverFuture;
use requests::{Request, Response};

pub const API_URL: &'static str = "https://api.pushover.net";
pub const API_VERSION: &'static str = "1";

#[derive(Debug, Clone)]
pub struct AsyncAPI {
    client: Client<HttpsConnector<HttpConnector>>,
    base_url: String,
    handle: Handle,
}

impl AsyncAPI {
    pub fn new(handle: &Handle) -> Result<Self, Error> {
        let connector = HttpsConnector::new(4, handle)?;

        let client = Client::configure().connector(connector).build(handle);

        Ok(Self {
               base_url: API_URL.to_owned(),
               client: client,
               handle: handle.clone(),
           })
    }

    pub fn set_base_url(&mut self, url: &str) {
        self.base_url = url.to_owned()
    }

    pub fn send<R: Request>(&self, request: &R) -> PushoverFuture<<R as Request>::ResponseType> {
        let mut url = Url::parse(&self.base_url).unwrap();
        url.set_path(API_VERSION);

        request.build_url(&mut url);

        let url = url.as_str().parse::<::hyper::Uri>().unwrap();

        let mut client_request = ::hyper::Request::new(request.get_method(), url);

        if let Some(params) = request.get_form_parameters() {
            let encoded: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(params.into_iter())
                .finish();

            client_request.set_body(encoded);
        }

        let response = self.client
            .request(client_request)
            .and_then(|res| res.body().concat2())
            .map_err(From::from);

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

    pub fn send_with_timeout<R: Request>
        (&self,
         req: &R,
         duration: Duration)
         -> PushoverFuture<Option<<R as Request>::ResponseType>> {
        let timeout_future = result(Timeout::new(duration, &self.handle))
            .flatten()
            .map_err(From::from)
            .map(|_| None);

        let get = self.send(req).map(Some);

        let future = timeout_future
            .select(get)
            .map(|(item, _next)| item)
            .map_err(|(item, _next)| item);

        PushoverFuture::new(Box::new(future))
    }

    pub fn spawn<R: Request>(&self, request: &R) {
        self.handle.spawn(self.send(request).then(|_| Ok(())));
    }
}

#[derive(Debug)]
pub struct SyncAPI {
    core: RefCell<Core>,
    api: AsyncAPI,
}

impl SyncAPI {
    pub fn new() -> Result<Self, Error> {
        let core = Core::new()?;
        let handle = core.handle();

        let api = AsyncAPI::new(&handle)?;

        Ok(SyncAPI {
               core: RefCell::new(core),
               api: api,
           })
    }

    pub fn set_base_url(&mut self, url: &str) {
        self.api.set_base_url(url);
    }

    pub fn send<R: Request>(&self, request: &R) -> Result<<R as Request>::ResponseType, Error> {
        let future = self.api.send(request);

        self.core.borrow_mut().run(future)
    }

    pub fn send_with_timeout<R: Request>(&self,
                                         req: &R,
                                         duration: Duration)
                                         -> Result<Option<<R as Request>::ResponseType>, Error> {
        let future = self.api.send_with_timeout(req, duration);

        self.core.borrow_mut().run(future)
    }
}