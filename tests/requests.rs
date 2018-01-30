extern crate pushover;
extern crate mockito;
extern crate tokio_core;

use pushover::{Error, ErrorKind, SyncAPI, SyncAPIBuilder, AsyncAPI, AsyncAPIBuilder};
use pushover::requests::message::{Limits, LimitsResponse};
use mockito::{mock, Matcher};
use tokio_core::reactor::{Core, Handle};

fn get_sync_api() -> SyncAPI {
    let api = SyncAPIBuilder::new()
        .base_url(mockito::SERVER_URL)
        .build()
        .unwrap();
    
    api
}

fn get_async_api(handle: &Handle) -> AsyncAPI {
    let api = AsyncAPIBuilder::new()
        .base_url(mockito::SERVER_URL)
        .build(handle)
        .unwrap();
    
    api
}

#[test]
fn test_sync_client_returns_pushover_error() {
    let _m = mock("GET", Matcher::Any)
        .with_body("{\"status\":0, \"request\":\"request_number\", \"errors\": [\"Error 1\", \"Error 2\"]}")
        .create();

    let request = Limits::new("token");
    let response = get_sync_api().send(&request);

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
fn test_sync_client_does_not_return_error() {
    let _m = mock("GET", Matcher::Any)
        .with_body("{\"status\":1, \"request\":\"request_number\", \"limit\": 1, \"remaining\": 2, \"reset\": 3}")
        .create();

    let request = Limits::new("token");
    let response = get_sync_api().send(&request);

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

#[test]
fn test_async_client_returns_pushover_error() {
    let _m = mock("GET", Matcher::Any)
        .with_body("{\"status\":0, \"request\":\"request_number\", \"errors\": [\"Error 1\", \"Error 2\"]}")
        .create();

    let mut core = Core::new().expect("Error creating core");
    let api = get_async_api(&core.handle());
    
    let request = Limits::new("token");
    let future = api.send(&request);
    let response = core.run(future);

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
fn test_async_client_does_not_return_error() {
    let _m = mock("GET", Matcher::Any)
        .with_body("{\"status\":1, \"request\":\"request_number\", \"limit\": 1, \"remaining\": 2, \"reset\": 3}")
        .create();
    
    let mut core = Core::new().expect("Error creating core");
    let api = get_async_api(&core.handle());
    
    let request = Limits::new("token");
    let future = api.send(&request);
    let response = core.run(future);
    
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