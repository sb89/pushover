use std::fmt;

use crate::error::ErrorKind;
use reqwest::Method;
use serde::de::{DeserializeOwned, Deserializer};
use serde::{Deserialize, Serialize};
use url::form_urlencoded::Serializer;
use url::{Url, UrlQuery};

use std::fmt::Debug;

pub trait RawResponse: DeserializeOwned + 'static {
    fn get_error(&self) -> Option<crate::error::ErrorKind> {
        if self.status() != 1 {
            Some(crate::error::ErrorKind::PushoverError {
                status: self.status(),
                request: self.request().to_string(),
                errors: self
                    .errors()
                    .clone()
                    .expect("Expected error array from Pushover API"),
            })
        } else {
            None
        }
    }

    fn status(&self) -> i32;

    fn request(&self) -> &str;

    fn errors(&self) -> &Option<Vec<String>>;
}

macro_rules! raw_response_basic_getters {
    () => {
        fn status(&self) -> i32 {
            self.status
        }

        fn request(&self) -> &str {
            &self.request
        }

        fn errors(&self) -> &Option<Vec<String>> {
            &self.errors
        }
    };
}

pub trait Request {
    type ResponseType: Debug + 'static;
    type RawResponseType: RawResponse;

    fn build_url(&self, url: &mut Url);

    fn map(raw: Self::RawResponseType) -> Self::ResponseType;

    fn get_method(&self) -> Method;

    fn get_form_parameters(&self) -> Option<Vec<(&str, &str)>> {
        None
    }
}

#[derive(Debug)]
pub enum Response<T: Request> {
    Error(ErrorKind),
    Success(T::RawResponseType),
}

impl<'de, T: Request> Deserialize<'de> for Response<T> {
    fn deserialize<D>(deserializer: D) -> Result<Response<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: T::RawResponseType = Deserialize::deserialize(deserializer)?;

        if let Some(err) = raw.get_error() {
            Ok(Response::Error(err))
        } else {
            Ok(Response::Success(raw))
        }
    }
}

#[derive(Deserialize)]
pub struct RawBasicResponse {
    pub status: i32,
    pub request: String,
    pub errors: Option<Vec<String>>,
}

impl RawResponse for RawBasicResponse {
    raw_response_basic_getters!();
}

pub fn add_optional_param<V>(params: &mut Serializer<UrlQuery>, key: &str, value: &Option<V>)
where
    V: fmt::Display,
{
    if let Some(ref a) = *value {
        params.append_pair(key, &a.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct TestRequest {}

    impl Request for TestRequest {
        type ResponseType = TestResponse;
        type RawResponseType = RawTestResponse;

        fn get_method(&self) -> Method {
            Method::POST
        }

        fn build_url(&self, _: &mut Url) {}

        fn map(_: Self::RawResponseType) -> Self::ResponseType {
            Self::ResponseType {}
        }
    }

    #[derive(Debug)]
    pub struct TestResponse {}

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct RawTestResponse {
        status: i32,
        request: String,
        errors: Option<Vec<String>>,
    }

    impl RawResponse for RawTestResponse {
        raw_response_basic_getters!();
    }

    #[test]
    fn deserialize_with_error() {
        let raw_response = RawTestResponse {
            status: 0,
            request: String::from("example_request"),
            errors: Some(vec![String::from("error1"), String::from("error2")]),
        };
        let raw_response_str = ::serde_json::to_string(&raw_response).unwrap();

        let resp: Response<TestRequest> = ::serde_json::from_str(&raw_response_str).unwrap();

        match resp {
            Response::Error::<TestRequest>(ErrorKind::PushoverError {
                status,
                errors,
                request,
            }) => {
                assert_eq!(status, raw_response.status);
                assert_eq!(request, raw_response.request);
                assert_eq!(Some(errors), raw_response.errors);
            }
            _ => panic!("Didn't receive error"),
        }
    }

    #[test]
    fn deserialize_no_error() {
        let raw_response = RawTestResponse {
            status: 1,
            request: String::from("example_request"),
            errors: None,
        };
        let raw_response_str = ::serde_json::to_string(&raw_response).unwrap();

        let resp: Response<TestRequest> = ::serde_json::from_str(&raw_response_str).unwrap();

        match resp {
            Response::Success::<TestRequest>(response) => assert_eq!(response, raw_response),
            _ => panic!("Received error"),
        }
    }
}
