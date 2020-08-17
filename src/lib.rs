//! # Pushover API Wrapper
//!
//! https://pushover.net/api
//!
//! ## Usage
//! Add the following to `Cargo.toml`:
//!
//! ```rust,ignore
//! [dependencies]
//! pushover = "0.4.0"
//! ```
//!
//! Synchronous example:
//!
//! ```rust,no_run
//!
//! use pushover::API;
//! use pushover::requests::message::SendMessage;
//!
//! fn send_message() {
//!     let api = API::new();
//!
//!     let msg = SendMessage::new("token", "user_key", "hello");
//!
//!     let response = api.send(&msg);
//!     println!("{:?}", response.expect("Error sending message"));
//! }
//!
//! ```
//!
//! Asynchronous example:
//!
//! ```rust,no_run
//!
//! use pushover::API;
//! use pushover::requests::message::SendMessage;
//!
//! async fn send_message() {
//!     let api = API::new();
//!
//!     let msg = SendMessage::new("token", "user_key", "hello");
//!     let response = api.send_async(&msg).await;
//!
//!     println!("{:?}", response.expect("Error sending message"));
//! }
//! ```

mod client;
mod deserializers;
mod error;
pub mod requests;
mod types;

pub use self::client::API;
pub use self::error::{Error, ErrorKind};
pub use self::types::{OperatingSystem, Priority, Sound, User, UserType};

#[cfg(test)]
mod test {
    use crate::client::{API_URL, API_VERSION};
    use crate::requests::Request;
    use url::Url;

    pub fn assert_req_url<R>(req: &R, path: &str, iter: Option<&[(&str, &str)]>)
    where
        R: Request,
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
