use hyper::Method;
use url::Url;

use requests::base::{Request, RawBasicResponse};

/// Remove a user from a group
///
/// Return type is `String` which is the request parameter (https://pushover.net/api#response).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RemoveUser {
    pub token: String,
    pub group_key: String,
    pub user_key: String,
}

impl RemoveUser {
    pub fn new<G, T, U>(token: T, group_key: G, user_key: U) -> Self
        where G: Into<String>,
              T: Into<String>,
              U: Into<String>
    {
        Self {
            token: token.into(),
            group_key: group_key.into(),
            user_key: user_key.into(),
        }
    }
}

impl Request for RemoveUser {
    type ResponseType = String;
    type RawResponseType = RawBasicResponse;

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut()
            .unwrap()
            .push("groups")
            .push(&self.group_key)
            .push("delete_user.json");

        let mut params = url.query_pairs_mut();
        params.append_pair("token", &self.token);
        params.append_pair("user", &self.user_key);
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
        let req = RemoveUser::new("remove_token", "remove_group_key", "remove_user_key");

        assert_req_url(&req,
                       &format!("groups/{}/delete_user.json", req.group_key),
                       Some(&[("token", &req.token), ("user", &req.user_key)]));
    }
}