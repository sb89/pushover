use reqwest::Method;
use serde::Deserialize;
use url::Url;

use crate::requests::base::{add_optional_param, RawResponse, Request};

/// Verify user/group
///
/// Return type is [VerificationResponse](struct.VerificationResponse.html).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Verification {
    pub token: String,
    pub user: String,
    pub device: Option<String>,
}

impl Verification {
    pub fn new<R, T>(token: T, user: R) -> Self
    where
        R: Into<String>,
        T: Into<String>,
    {
        Self {
            token: token.into(),
            user: user.into(),
            device: None,
        }
    }

    pub fn set_device<T: Into<String>>(&mut self, device: T) {
        self.device = Some(device.into());
    }
}

impl Request for Verification {
    type ResponseType = VerificationResponse;
    type RawResponseType = RawVerificationResponse;

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut()
            .unwrap()
            .push("users")
            .push("validate.json");

        let mut params = url.query_pairs_mut();
        params.append_pair("token", &self.token);
        params.append_pair("user", &self.user);
        add_optional_param(&mut params, "device", &self.device);
    }

    fn get_method(&self) -> Method {
        Method::POST
    }

    fn map(raw: Self::RawResponseType) -> Self::ResponseType {
        Self::ResponseType {
            devices: raw.devices.unwrap(),
            request: raw.request,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VerificationResponse {
    pub devices: Vec<String>,
    pub request: String,
}

#[derive(Deserialize)]
pub struct RawVerificationResponse {
    pub status: i32,
    pub request: String,
    pub errors: Option<Vec<String>>,
    pub devices: Option<Vec<String>>,
}

impl RawResponse for RawVerificationResponse {
    raw_response_basic_getters!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::assert_req_url;

    #[test]
    fn get_url_with_all_fields() {
        let mut req = Verification::new("ver_token", "ver user");
        req.set_device("ver device");

        assert_req_url(
            &req,
            "users/validate.json",
            Some(&[
                ("token", &req.token),
                ("user", &req.user),
                ("device", &req.device.as_ref().unwrap()),
            ]),
        );
    }

    #[test]
    fn get_url_with_mandatory_fields() {
        let req = Verification::new("ver_token", "ver user");

        assert_req_url(
            &req,
            "users/validate.json",
            Some(&[("token", &req.token), ("user", &req.user)]),
        );
    }
}
