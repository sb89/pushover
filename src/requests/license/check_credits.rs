use reqwest::Method;
use serde::Deserialize;
use url::Url;

use crate::requests::base::{RawResponse, Request};

/// Check license credits
///
/// Return type is [CheckCreditsResponse](struct.CheckCreditsResponse.html).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckCredits {
    pub token: String,
}

impl CheckCredits {
    pub fn new<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            token: token.into(),
        }
    }
}

impl Request for CheckCredits {
    type ResponseType = CheckCreditsResponse;
    type RawResponseType = RawCheckCreditsResponse;

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut().unwrap().push("licenses.json");
        let mut params = url.query_pairs_mut();

        params.append_pair("token", &self.token);
    }

    fn get_method(&self) -> Method {
        Method::GET
    }

    fn map(raw: Self::RawResponseType) -> Self::ResponseType {
        Self::ResponseType {
            request: raw.request,
            credits: raw.credits.unwrap(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
/// Return type for [CheckCredits](struct.CheckCredits.html) and [Assign](struct.Assign.html)
pub struct CheckCreditsResponse {
    pub request: String,
    pub credits: u16,
}

#[derive(Debug, Deserialize)]
pub struct RawCheckCreditsResponse {
    pub status: i32,
    pub request: String,
    pub errors: Option<Vec<String>>,
    pub credits: Option<u16>,
}

impl RawResponse for RawCheckCreditsResponse {
    raw_response_basic_getters!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::assert_req_url;

    #[test]
    fn get_url() {
        let req = CheckCredits::new("check_token");

        assert_req_url(&req, "licenses.json", Some(&[("token", &req.token)]));
    }
}
