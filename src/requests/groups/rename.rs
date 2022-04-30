use reqwest::Method;
use url::Url;

use crate::requests::base::{RawBasicResponse, Request};

/// Rename a group
///
/// Return type is `String` which is the request parameter (https://pushover.net/api#response).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Rename {
    pub token: String,
    pub group_key: String,
    pub name: String,
}

impl Rename {
    pub fn new<G, N, T>(token: T, group_key: G, name: N) -> Self
    where
        G: Into<String>,
        N: Into<String>,
        T: Into<String>,
    {
        Self {
            token: token.into(),
            group_key: group_key.into(),
            name: name.into(),
        }
    }
}

impl Request for Rename {
    type ResponseType = String;
    type RawResponseType = RawBasicResponse;

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut()
            .unwrap()
            .push("groups")
            .push(&self.group_key)
            .push("rename.json");

        let mut params = url.query_pairs_mut();
        params.append_pair("token", &self.token);
        params.append_pair("name", &self.name);
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
    fn get_url() {
        let req = Rename::new("rename_token", "rename_group_key", "rename name");

        assert_req_url(
            &req,
            &format!("groups/{}/rename.json", req.group_key),
            Some(&[("token", &req.token), ("name", &req.name)]),
        );
    }
}
