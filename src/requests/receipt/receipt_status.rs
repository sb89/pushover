use reqwest::Method;
use serde::Deserialize;
use url::Url;

use crate::requests::base::{RawResponse, Request};

/// Retrieve status of emergency notification
///
/// Return type is [ReceiptStatusResponse](struct.ReceiptStatusResponse.html).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ReceiptStatus {
    pub token: String,
    pub receipt: String,
}

impl ReceiptStatus {
    pub fn new<R, T>(token: T, receipt: R) -> Self
    where
        R: Into<String>,
        T: Into<String>,
    {
        Self {
            token: token.into(),
            receipt: receipt.into(),
        }
    }
}

impl Request for ReceiptStatus {
    type ResponseType = ReceiptStatusResponse;
    type RawResponseType = RawReceiptStatusResponse;

    fn get_method(&self) -> Method {
        Method::GET
    }

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut()
            .unwrap()
            .push("receipts")
            .push(&format!("{}.json", &self.receipt));

        let mut params = url.query_pairs_mut();
        params.append_pair("token", &self.token);
    }

    fn map(raw: Self::RawResponseType) -> Self::ResponseType {
        Self::ResponseType {
            called_back: raw.called_back.unwrap(),
            called_back_at: raw.called_back_at.unwrap(),
            acknowledged: raw.acknowledged.unwrap(),
            acknowledged_at: raw.acknowledged_at.unwrap(),
            acknowledged_by: raw.acknowledged_by.unwrap(),
            acknowledged_by_device: raw.acknowledged_by_device.unwrap(),
            last_delivered_at: raw.last_delivered_at.unwrap(),
            expired: raw.expired.unwrap(),
            expires_at: raw.expires_at.unwrap(),
            request: raw.request,
        }
    }
}

/// Return type for [ReceiptStatus](struct.ReceiptStatus.html)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ReceiptStatusResponse {
    pub called_back: u8,
    pub called_back_at: u32,
    pub acknowledged: u8,
    pub acknowledged_at: u32,
    pub acknowledged_by: String,
    pub acknowledged_by_device: String,
    pub last_delivered_at: u32,
    pub expired: u8,
    pub expires_at: u32,
    pub request: String,
}

#[derive(Deserialize)]
pub struct RawReceiptStatusResponse {
    pub status: i32,
    pub request: String,
    pub errors: Option<Vec<String>>,
    pub called_back: Option<u8>,
    pub called_back_at: Option<u32>,
    pub acknowledged: Option<u8>,
    pub acknowledged_at: Option<u32>,
    pub acknowledged_by: Option<String>,
    pub acknowledged_by_device: Option<String>,
    pub last_delivered_at: Option<u32>,
    pub expired: Option<u8>,
    pub expires_at: Option<u32>,
}

impl RawResponse for RawReceiptStatusResponse {
    raw_response_basic_getters!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::assert_req_url;

    #[test]
    fn get_url_() {
        let req = ReceiptStatus::new("receipt_token", "receipt");

        assert_req_url(
            &req,
            &format!("receipts/{}.json", req.receipt),
            Some(&[("token", &req.token)]),
        );
    }
}
