use reqwest::Method;
use url::Url;

use requests::base::{Request, RawBasicResponse};

/// Acknowledge an emergency-priority message
///
/// Return type is `String` which is the request parameter (https://pushover.net/api#response).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Acknowledge {
    pub secret: String,
    pub receipt: String,
}

impl Acknowledge {
    pub fn new<S, R>(secret: S, receipt: R) -> Self
        where S: Into<String>,
              R: Into<String>
    {
        Self {
            receipt: receipt.into(),
            secret: secret.into(),
        }
    }
}

impl Request for Acknowledge {
    type ResponseType = String;
    type RawResponseType = RawBasicResponse;

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut()
            .unwrap()
            .push("receipts")
            .push(&self.receipt)
            .push("acknowledge.json");

        let mut params = url.query_pairs_mut();
        params.append_pair("secret", &self.secret);

    }

    fn get_method(&self) -> Method {
        Method::Post
    }

    fn map(raw: Self::RawResponseType) -> Self::ResponseType {
        raw.request
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::assert_req_url;

    #[test]
    fn get_url() {
        let req = Acknowledge::new("ack_secret", "ack_receipt");

        assert_req_url(&req,
                       &format!("receipts/{}/acknowledge.json", req.receipt),
                       Some(&[("secret", &req.secret)]));
    }
}