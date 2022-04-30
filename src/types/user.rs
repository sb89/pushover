use crate::deserializers::deserialize_option_empty_string;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd)]
pub struct User {
    pub user: String,
    #[serde(deserialize_with = "deserialize_option_empty_string")]
    pub device: Option<String>,
    #[serde(deserialize_with = "deserialize_option_empty_string")]
    pub memo: Option<String>,
    pub disabled: bool,
}

impl User {
    pub fn new<R>(user: R) -> Self
    where
        R: Into<String>,
    {
        Self {
            user: user.into(),
            device: None,
            memo: None,
            disabled: false,
        }
    }

    pub fn set_device<R>(&mut self, device: R)
    where
        R: Into<String>,
    {
        self.device = Some(device.into());
    }

    pub fn set_memo<R>(&mut self, memo: R)
    where
        R: Into<String>,
    {
        self.memo = Some(memo.into());
    }
}
