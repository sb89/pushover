use hyper::Method;
use url::Url;

use types::{OperatingSystem, UserType};
use requests::license::check_credits::{CheckCreditsResponse, RawCheckCreditsResponse};
use requests::base::{Request, add_optional_param};

/// Assign a license
///
/// Return type is [CheckCreditsResponse](struct.CheckCreditsResponse.html).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Assign {
    pub token: String,
    pub os: Option<OperatingSystem>,
    pub user_type: UserType,
}

impl Assign {
    pub fn new<T>(token: T, user_type: UserType) -> Self
        where T: Into<String>
    {
        Self {
            token: token.into(),
            os: None,
            user_type: user_type,
        }
    }

    pub fn set_os(&mut self, os: OperatingSystem) {
        self.os = Some(os);
    }
}

impl Request for Assign {
    type ResponseType = CheckCreditsResponse;
    type RawResponseType = RawCheckCreditsResponse;

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut()
            .unwrap()
            .push("licenses")
            .push("assign.json");
        let mut params = url.query_pairs_mut();

        params.append_pair("token", &self.token);

        match self.user_type {
            UserType::Email(ref email) => {
                params.append_pair("email", email);
            }
            UserType::UserKey(ref key) => {
                params.append_pair("user", key);
            }
        }

        add_optional_param(&mut params, "os", &self.os);
    }

    fn get_method(&self) -> Method {
        Method::Post
    }

    fn map(raw: Self::RawResponseType) -> Self::ResponseType {
        Self::ResponseType {
            request: raw.request,
            credits: raw.credits.unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::assert_req_url;

    #[test]
    fn get_url_with_all_fields() {
        let mut req = Assign::new("assign_token",
                                  UserType::Email(String::from("email@email.com")));
        req.set_os(OperatingSystem::iOS);

        assert_req_url(&req,
                       "licenses/assign.json",
                       Some(&[("token", &req.token),
                              ("email", "email@email.com"),
                              ("os", &req.os.as_ref().unwrap().to_string())]));
    }

    #[test]
    fn get_url_with_mandatory_fields() {
        let req = Assign::new("assign_token", UserType::UserKey(String::from("user_key")));

        assert_req_url(&req,
                       "licenses/assign.json",
                       Some(&[("token", &req.token), ("user", "user_key")]));
    }
}