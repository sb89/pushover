use reqwest::Method;
use url::Url;

use crate::requests::base::{RawBasicResponse, Request};

/// Cancel an emergency priority notification
///
/// Return type is `String` which is the request parameter (https://pushover.net/api#response).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CancelEmergency {
    pub token: String,
    pub receipt: String,
}

impl CancelEmergency {
    pub fn new<R, T>(token: T, receipt: R) -> Self
    where
        R: Into<String>,
        T: Into<String>,
    {
        Self {
            token: token.into(),
            receipt: receipt.into(),
        }
    }
}

impl Request for CancelEmergency {
    type ResponseType = String;
    type RawResponseType = RawBasicResponse;

    fn get_method(&self) -> Method {
        Method::POST
    }

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut()
            .unwrap()
            .push("receipts")
            .push(&self.receipt)
            .push("cancel.json");

        let mut params = url.query_pairs_mut();
        params.append_pair("token", &self.token);
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
        let req = CancelEmergency::new("canc_token", "canc_receipt");

        assert_req_url(
            &req,
            &format!("receipts/{}/cancel.json", req.receipt),
            Some(&[("token", &req.token)]),
        );
    }
}
