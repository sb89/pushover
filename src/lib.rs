//! # Pushover API Wrapper
//!
//! https://pushover.net/api
//!
//! ## Usage
//! Add the following to `Cargo.toml`:
//!
//! ```rust,ignore
//! [dependencies]
//! pushover = "0.3.0"
//! ```
//!
//! Synchronous example:
//!
//! ```rust,no_run
//!
//! extern crate pushover;
//!
//! use pushover::SyncAPIBuilder;
//! use pushover::requests::message::SendMessage;
//!
//! fn main() {
//!     let api = SyncAPIBuilder::new().build().expect("Error creating API");
//!
//!     let msg = SendMessage::new("token", "user_key", "hello");
//!
//!     let response = api.send(&msg);
//!     println!("{:?}", response);
//! }
//!
//! ```
//!
//! Asynchronous example:
//!
//! ```rust,no_run
//!
//! extern crate pushover;
//! extern crate tokio_core;
//!
//! use pushover::AsyncAPIBuilder;
//! use pushover::requests::message::SendMessage;
//! use tokio_core::reactor::Core;
//!
//! fn main() {
//!     let mut core = Core::new().expect("Error creating core");
//!     let handle = core.handle();
//!
//!     let api = AsyncAPIBuilder::new().build(&handle).expect("Error creating API");
//!
//!     let msg = SendMessage::new("token", "user_key", "hello");
//!     let work = api.send(&msg);
//!
//!     println!("{:?}", core.run(work).expect("Error sending message"));
//! }
//! ```

extern crate tokio_core;
#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate futures;
extern crate urlencoding;
extern crate url;
extern crate reqwest;

mod client;
mod error;
mod future;
mod types;
mod deserializers;
pub mod requests;

pub use self::client::{AsyncAPI, AsyncAPIBuilder, SyncAPI, SyncAPIBuilder};
pub use self::future::PushoverFuture;
pub use self::types::{Priority, OperatingSystem, Sound, User, UserType};
pub use self::error::{Error, ErrorKind};

#[cfg(test)]
mod test {
    use requests::Request;
    use url::Url;
    use client::{API_URL, API_VERSION};

    pub fn assert_req_url<R>(req: &R, path: &str, iter: Option<&[(&str, &str)]>)
        where R: Request
    {
        let mut url = Url::parse(&format!("{}/{}", API_URL, API_VERSION)).unwrap();
        req.build_url(&mut url);

        let expected_url = match iter {
            Some(x) => {
                Url::parse_with_params(&format!("{}/{}/{}", API_URL, API_VERSION, path), x).unwrap()
            }
            None => Url::parse(&format!("{}/{}/{}", API_URL, API_VERSION, path)).unwrap(),
        };

        assert_eq!(expected_url, url);
    }
}