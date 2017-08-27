use hyper::Method;
use url::Url;

use requests::base::{RawResponse, Request};

/// Register desktop device
///
/// Return type is [RegisterDeviceResponse](struct.RegisterDeviceResponse.html).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RegisterDevice {
    pub secret: String,
    pub name: String
}

impl RegisterDevice {
    pub fn new<N, S>(secret: S, name: N) -> Self
        where N: Into<String>,
              S: Into<String>
    {
        Self {
            secret: secret.into(),
            name: name.into(),
        }
    }
}

impl Request for RegisterDevice {
    type ResponseType = RegisterDeviceResponse;
    type RawResponseType = RawRegisterDeviceResponse;

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut().unwrap().push("devices.json");
    }

    fn get_method(&self) -> Method {
        Method::Post
    }

    fn map(raw: Self::RawResponseType) -> Self::ResponseType {
        Self::ResponseType {
            request: raw.request,
            id: raw.id.unwrap()
        }
    }

    fn get_form_parameters(&self) -> Option<Vec<(&str, &str)>> {
        Some(vec![("secret", &self.secret),
                  ("name", &self.name),
                  ("os", "O")])
    }
}

/// Return type for [RegisterDevice](struct.RegisterDevice.html)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RegisterDeviceResponse{
    pub request: String,
    pub id: String,
}

#[derive(Deserialize)]
pub struct RawRegisterDeviceResponse{
    pub status: i32,
    pub request: String,
    pub id: Option<String>,
    pub errors: Option<Vec<String>>,
}

impl RawResponse for RawRegisterDeviceResponse {
    raw_response_basic_getters!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::assert_req_url;

    #[test]
    fn get_url(){
        let req = RegisterDevice::new("reg_secret", "reg_name");

        assert_req_url(&req,
                       "devices.json",
                       None);
    }

    #[test]
    fn get_form_parameters(){
        let req = RegisterDevice::new("reg_secret", "reg_name");

        assert_eq!(Some(vec![("secret", req.secret.as_ref()),("name", req.name.as_ref()), ("os", "O")]), req.get_form_parameters());
    }
}