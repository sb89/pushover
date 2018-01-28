use reqwest::Method;
use url::Url;

use requests::base::{Request, RawBasicResponse, add_optional_param};

/// Send a Glance request
///
/// Return type is `String` which is the request parameter (https://pushover.net/api#response).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Glance {
    pub token: String,
    pub user_key: String,
    pub device: Option<String>,
    pub title: Option<String>,
    pub text: Option<String>,
    pub subtext: Option<String>,
    pub count: Option<i32>,
    pub percent: Option<u8>,
}

impl Glance {
    pub fn new<T, U>(token: T, user_key: U) -> Self
        where T: Into<String>,
              U: Into<String>
    {
        Self {
            token: token.into(),
            user_key: user_key.into(),
            device: None,
            title: None,
            text: None,
            subtext: None,
            count: None,
            percent: None,
        }
    }

    pub fn set_device<T: Into<String>>(&mut self, device: T) {
        self.device = Some(device.into());
    }

    pub fn set_title<T: Into<String>>(&mut self, title: T) {
        self.title = Some(title.into());
    }

    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = Some(text.into());
    }

    pub fn set_subtext<T: Into<String>>(&mut self, subtext: T) {
        self.subtext = Some(subtext.into());
    }

    pub fn set_count(&mut self, count: i32) {
        self.count = Some(count);
    }

    pub fn set_percent(&mut self, percent: u8) {
        self.percent = Some(percent);
    }
}

impl Request for Glance {
    type ResponseType = String;
    type RawResponseType = RawBasicResponse;

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut().unwrap().push("glances.json");

        let mut params = url.query_pairs_mut();
        params.append_pair("token", &self.token);
        params.append_pair("user", &self.user_key);
        add_optional_param(&mut params, "device", &self.device);
        add_optional_param(&mut params, "title", &self.title);
        add_optional_param(&mut params, "text", &self.text);
        add_optional_param(&mut params, "subtext", &self.subtext);
        add_optional_param(&mut params, "count", &self.count);
        add_optional_param(&mut params, "percent", &self.percent);
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
    fn get_url_with_all_fields() {
        let mut req = Glance::new("glance_token", "glance_user_key");
        req.set_device("glance device");
        req.set_title("glance title");
        req.set_text("glance text");
        req.set_subtext("glance subtext");
        req.set_count(10);
        req.set_percent(80);

        assert_req_url(&req,
                       "glances.json",
                       Some(&[("token", &req.token) ,("user", &req.user_key), ("device", &req.device.as_ref().unwrap()), ("title", &req.title.as_ref().unwrap()),
                       ("text", &req.text.as_ref().unwrap()), ("subtext", &req.subtext.as_ref().unwrap()), ("count", &req.count.unwrap().to_string()), ("percent", &req.percent.unwrap().to_string())]));
    }

    #[test]
    fn get_url_with_mandatory_fields() {
        let req = Glance::new("glance_token", "glance_user_key");

        assert_req_url(&req,
                       "glances.json",
                       Some(&[("token", &req.token) ,("user", &req.user_key)]));
    }
}