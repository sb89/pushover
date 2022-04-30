use reqwest::Method;
use serde::Deserialize;
use url::Url;

use crate::requests::base::{RawResponse, Request};
use crate::types::User;

/// Retrieve users of a group
///
/// Return type is [ListUsersResponse](struct.ListUsersResponse.html)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListUsers {
    pub token: String,
    pub group_key: String,
}

impl ListUsers {
    pub fn new<R, T>(token: T, group_key: R) -> Self
    where
        R: Into<String>,
        T: Into<String>,
    {
        Self {
            token: token.into(),
            group_key: group_key.into(),
        }
    }
}

impl Request for ListUsers {
    type ResponseType = ListUsersResponse;
    type RawResponseType = RawListUsersResponse;

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut()
            .unwrap()
            .push("groups")
            .push(&format!("{}.json", &self.group_key));

        let mut params = url.query_pairs_mut();
        params.append_pair("token", &self.token);
    }

    fn get_method(&self) -> Method {
        Method::GET
    }

    fn map(raw: Self::RawResponseType) -> Self::ResponseType {
        Self::ResponseType {
            request: raw.request,
            name: raw.name.unwrap(),
            users: raw.users.unwrap(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
/// Return type for [ListUsers](struct.ListUsers.html).
pub struct ListUsersResponse {
    pub request: String,
    pub name: String,
    pub users: Vec<User>,
}

#[derive(Deserialize)]
pub struct RawListUsersResponse {
    pub status: i32,
    pub request: String,
    pub errors: Option<Vec<String>>,
    pub name: Option<String>,
    pub users: Option<Vec<User>>,
}

impl RawResponse for RawListUsersResponse {
    raw_response_basic_getters!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::assert_req_url;

    #[test]
    fn get_url() {
        let req = ListUsers::new("get_token", "get_group_key");

        assert_req_url(
            &req,
            &format!("groups/{}.json", req.group_key),
            Some(&[("token", &req.token)]),
        );
    }
}
