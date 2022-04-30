use reqwest::Method;
use url::Url;

use crate::requests::base::{RawBasicResponse, Request};

/// Disable/enable a user for a group
///
/// Return type is `String` which is the request parameter (https://pushover.net/api#response).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToggleUser {
    pub token: String,
    pub group_key: String,
    pub user_key: String,
    pub toggle: bool,
}

impl ToggleUser {
    pub fn new<G, T, U>(token: T, group_key: G, user_key: U, toggle: bool) -> Self
    where
        G: Into<String>,
        T: Into<String>,
        U: Into<String>,
    {
        Self {
            token: token.into(),
            group_key: group_key.into(),
            user_key: user_key.into(),
            toggle: toggle,
        }
    }
}

impl Request for ToggleUser {
    type ResponseType = String;
    type RawResponseType = RawBasicResponse;

    fn build_url(&self, url: &mut Url) {
        let end_point = if self.toggle { "enable" } else { "disable" };

        url.path_segments_mut()
            .unwrap()
            .push("groups")
            .push(&self.group_key)
            .push(&format!("{}_user.json", end_point));

        let mut params = url.query_pairs_mut();
        params.append_pair("token", &self.token);
        params.append_pair("user", &self.user_key);
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
    fn get_url_enable() {
        let req = ToggleUser::new("toggle_token", "toggle_group_key", "toggle_user_key", true);

        assert_req_url(
            &req,
            &format!("groups/{}/enable_user.json", req.group_key),
            Some(&[("token", &req.token), ("user", &req.user_key)]),
        );
    }

    #[test]
    fn get_url_disable() {
        let req = ToggleUser::new("toggle_token", "toggle_group_key", "toggle_user_key", false);

        assert_req_url(
            &req,
            &format!("groups/{}/disable_user.json", req.group_key),
            Some(&[("token", &req.token), ("user", &req.user_key)]),
        );
    }
}
