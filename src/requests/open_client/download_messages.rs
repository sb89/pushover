use reqwest::Method;
use serde::Deserialize;
use url::Url;

use crate::requests::base::{RawResponse, Request};
use crate::types::Message;

/// Download messages
///
/// Return type is [DownloadMessagesResponse](struct.DownloadMessagesResponse.html).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DownloadMessages {
    pub secret: String,
    pub device_id: String,
}

impl DownloadMessages {
    pub fn new<D, S>(secret: S, device_id: D) -> Self
    where
        D: Into<String>,
        S: Into<String>,
    {
        Self {
            secret: secret.into(),
            device_id: device_id.into(),
        }
    }
}

impl Request for DownloadMessages {
    type ResponseType = DownloadMessagesResponse;
    type RawResponseType = RawDownloadMessagesResponse;

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut().unwrap().push("messages.json");

        let mut params = url.query_pairs_mut();
        params.append_pair("secret", &self.secret);
        params.append_pair("device_id", &self.device_id);
    }

    fn get_method(&self) -> Method {
        Method::GET
    }

    fn map(raw: Self::RawResponseType) -> Self::ResponseType {
        Self::ResponseType {
            request: raw.request,
            messages: raw.messages.unwrap(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
/// Return type for [DownloadMessages](struct.DownloadMessages.html)
pub struct DownloadMessagesResponse {
    pub request: String,
    pub messages: Vec<Message>,
}

#[derive(Deserialize)]
pub struct RawDownloadMessagesResponse {
    pub request: String,
    pub status: i32,
    pub errors: Option<Vec<String>>,
    pub messages: Option<Vec<Message>>,
}

impl RawResponse for RawDownloadMessagesResponse {
    raw_response_basic_getters!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::assert_req_url;

    #[test]
    fn get_url() {
        let req = DownloadMessages::new("down_secret", "down_device");

        assert_req_url(
            &req,
            "messages.json",
            Some(&[("secret", &req.secret), ("device_id", &req.device_id)]),
        );
    }
}
