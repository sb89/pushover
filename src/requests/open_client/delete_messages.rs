use reqwest::Method;
use url::Url;

use crate::requests::base::{RawBasicResponse, Request};

/// Delete Messages
///
/// Return type is `String` which is the request parameter (https://pushover.net/api#response).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteMessages {
    pub device_id: String,
    pub secret: String,
    pub message: u32,
}

impl DeleteMessages {
    pub fn new<D, S>(secret: S, device_id: D, message: u32) -> Self
    where
        D: Into<String>,
        S: Into<String>,
    {
        Self {
            device_id: device_id.into(),
            secret: secret.into(),
            message,
        }
    }
}

impl Request for DeleteMessages {
    type ResponseType = String;
    type RawResponseType = RawBasicResponse;

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut()
            .unwrap()
            .push("devices")
            .push(&self.device_id)
            .push("update_highest_message.json");

        let mut params = url.query_pairs_mut();
        params.append_pair("secret", &self.secret);
        params.append_pair("message", &self.message.to_string());
    }

    fn get_method(&self) -> Method {
        Method::POST
    }

    fn map(raw: Self::RawResponseType) -> Self::ResponseType {
        raw.request
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::assert_req_url;

    #[test]
    fn get_url() {
        let req = DeleteMessages::new("del_secret", "del_device", 10);

        assert_req_url(
            &req,
            &format!("devices/{}/update_highest_message.json", req.device_id),
            Some(&[
                ("secret", &req.secret),
                ("message", &req.message.to_string()),
            ]),
        );
    }
}
