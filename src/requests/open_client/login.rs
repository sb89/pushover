use reqwest::Method;
use url::Url;

use requests::base::{RawResponse, Request};

/// Login user
///
/// Return type is [LoginResponse](struct.LoginResponse.html).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Login {
    pub email: String,
    pub password: String,
}

impl Login {
    pub fn new<E, P>(email: E, password: P) -> Self
        where E: Into<String>,
              P: Into<String>
    {
        Self {
            email: email.into(),
            password: password.into(),
        }
    }
}

impl Request for Login {
    type ResponseType = LoginResponse;
    type RawResponseType = RawLoginResponse;

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut().unwrap().push("users").push("login.json");
    }

    fn get_method(&self) -> Method {
        Method::Post
    }

    fn map(raw: Self::RawResponseType) -> Self::ResponseType {
        Self::ResponseType {
            request: raw.request,
            id: raw.id.unwrap(),
            secret: raw.secret.unwrap(),
        }
    }

    fn get_form_parameters(&self) -> Option<Vec<(&str, &str)>> {
        Some(vec![("email", &self.email),
                  ("password", &self.password)])
    }
}

/// Return type for [Login](struct.Login.html)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoginResponse {
    pub id: String,
    pub secret: String,
    pub request: String,
}

#[derive(Deserialize)]
pub struct RawLoginResponse {
    pub status: i32,
    pub request: String,
    pub id: Option<String>,
    pub secret: Option<String>,
    pub errors: Option<Vec<String>>,
}

impl RawResponse for RawLoginResponse {
    raw_response_basic_getters!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::assert_req_url;

    #[test]
    fn get_url(){
        let req = Login::new("email@email.com", "Password!@%d");

        assert_req_url(&req,
                       "users/login.json",
                       None);
    }

    #[test]
    fn get_form_parameters(){
        let req = Login::new("email@email.com", "Password!@%d");

        assert_eq!(Some(vec![("email", req.email.as_ref()),("password", req.password.as_ref())]), req.get_form_parameters());
    }
}