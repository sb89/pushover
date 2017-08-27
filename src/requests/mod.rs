#[macro_use]
mod base;

pub mod verification;
pub mod groups;
pub mod license;
pub mod open_client;
pub mod receipt;
pub mod message;
pub mod glance;

pub(crate) use self::base::{Request, Response};
