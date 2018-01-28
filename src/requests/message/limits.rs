use reqwest::Method;
use url::Url;

use requests::base::{Request, RawResponse};

/// Get limitations
///
/// Return type is [LimitsResponse](struct.LimitsResponse.html).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Limits {
    pub token: String,
}

impl Limits {
    pub fn new<T>(token: T) -> Self
        where T: Into<String>
    {
        Self { token: token.into() }
    }
}

impl Request for Limits {
    type ResponseType = LimitsResponse;
    type RawResponseType = RawLimitsResponse;

    fn get_method(&self) -> Method {
        Method::Get
    }

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut()
            .unwrap()
            .push("apps")
            .push("limits.json");

        let mut params = url.query_pairs_mut();
        params.append_pair("token", &self.token);
    }

    fn map(raw: Self::RawResponseType) -> Self::ResponseType {
        Self::ResponseType {
            request: raw.request,
            limit: raw.limit.unwrap(),
            remaining: raw.remaining.unwrap(),
            reset: raw.reset.unwrap(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
/// Return type for [Limits](struct.Limits.html)
pub struct LimitsResponse {
    pub request: String,
    pub limit: u32,
    pub remaining: u32,
    pub reset: u32,
}

#[derive(Debug, Deserialize)]
pub struct RawLimitsResponse {
    pub status: i32,
    pub request: String,
    pub errors: Option<Vec<String>>,
    pub limit: Option<u32>,
    pub remaining: Option<u32>,
    pub reset: Option<u32>,
}

impl RawResponse for RawLimitsResponse {
    raw_response_basic_getters!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::assert_req_url;

    #[test]
    fn get_url() {
        let req = Limits::new("canc_token");

        assert_req_url(&req, "apps/limits.json", Some(&[("token", &req.token)]));
    }
}