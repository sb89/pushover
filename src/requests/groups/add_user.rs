use reqwest::Method;
use url::Url;

use crate::requests::base::{add_optional_param, RawBasicResponse, Request};
use crate::types::User;

/// Add a user to a group
///
/// Return type is `String` which is the request parameter (https://pushover.net/api#response).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddUser {
    pub token: String,
    pub group_key: String,
    pub user: User,
}

impl AddUser {
    pub fn new<R, T>(token: T, group_key: R, user: &User) -> Self
    where
        R: Into<String>,
        T: Into<String>,
    {
        Self {
            token: token.into(),
            group_key: group_key.into(),
            user: user.clone(),
        }
    }
}

impl Request for AddUser {
    type ResponseType = String;
    type RawResponseType = RawBasicResponse;

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut()
            .unwrap()
            .push("groups")
            .push(&self.group_key)
            .push("add_user.json");

        let mut params = url.query_pairs_mut();
        params.append_pair("token", &self.token);
        params.append_pair("user", &self.user.user);
        add_optional_param(&mut params, "device", &self.user.device);
        add_optional_param(&mut params, "memo", &self.user.memo);
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
    fn get_url_with_all_fields() {
        let mut user = User::new("user_key");
        user.set_device("device name");
        user.set_memo("memo name");

        let req = AddUser::new("add_token", "add_group_key", &user);

        assert_req_url(
            &req,
            &format!("groups/{}/add_user.json", req.group_key),
            Some(&[
                ("token", &req.token),
                ("user", &req.user.user),
                ("device", req.user.device.as_ref().unwrap()),
                ("memo", req.user.memo.as_ref().unwrap()),
            ]),
        );
    }

    #[test]
    fn get_url_with_mandatory_fields() {
        let user = User::new("user_key");

        let req = AddUser::new("add_token", "add_group_key", &user);

        assert_req_url(
            &req,
            &format!("groups/{}/add_user.json", req.group_key),
            Some(&[("token", &req.token), ("user", &req.user.user)]),
        );
    }
}
