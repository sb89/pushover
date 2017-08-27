extern crate pushover;
extern crate mockito;

use pushover::{Error, ErrorKind, SyncAPI};
use pushover::requests::message::{Limits, LimitsResponse};
use mockito::{mock, Matcher};

fn get_api() -> SyncAPI {
    let mut api = SyncAPI::new().unwrap();
    api.set_base_url(mockito::SERVER_URL);

    api
}

#[test]
fn test_client_returns_pushover_error() {
    let _m = mock("GET", Matcher::Any)
      .with_body("{\"status\":0, \"request\":\"request_number\", \"errors\": [\"Error 1\", \"Error 2\"]}")
      .create();

    let request = Limits::new("token");
    let response = get_api().send(&request);

    match response.expect_err("Expected error") {
        Error(ErrorKind::PushoverError {
                  status,
                  errors,
                  request,
              },
              _) => {
            assert_eq!(status, 0);
            assert_eq!(errors, vec!["Error 1", "Error 2"]);
            assert_eq!(request, "request_number");
        }
        _ => panic!("Did not receive PushoverError"),
    }
}

#[test]
fn test_client_does_not_return_error() {
    let _m = mock("GET", Matcher::Any)
    .with_body("{\"status\":1, \"request\":\"request_number\", \"limit\": 1, \"remaining\": 2, \"reset\": 3}")
    .create();

    let request = Limits::new("token");
    let response = get_api().send(&request);

    match response {
        Ok(LimitsResponse {
               request,
               limit,
               remaining,
               reset,
           }) => {
            assert_eq!(request, "request_number");
            assert_eq!(limit, 1);
            assert_eq!(remaining, 2);
            assert_eq!(reset, 3);
        }
        _ => panic!("Received error"),
    }
}