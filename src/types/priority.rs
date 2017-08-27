use std::fmt;

use serde::de::{self, Deserialize, Deserializer};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Priority {
    Lowest,
    Low,
    Normal,
    High,
    Emergency {
        retry: u32,
        expire: u32,
        callback_url: Option<String>,
    },
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Priority::Lowest => "-2",
            Priority::Low => "-1",
            Priority::Normal => "0",
            Priority::High => "1",
            Priority::Emergency { .. } => "2",
        };

        write!(f, "{}", printable)
    }
}

impl Priority {
    pub fn from_int(i: i8) -> Option<Self> {
        match i {
            -2 => Some(Priority::Lowest),
            -1 => Some(Priority::Low),
            0 => Some(Priority::Normal),
            1 => Some(Priority::High),
            2 => {
                Some(Priority::Emergency {
                         retry: 0,
                         expire: 0,
                         callback_url: None,
                     })
            }
            _ => None,
        }
    }
}

impl<'de> Deserialize<'de> for Priority {
    fn deserialize<D>(deserializer: D) -> Result<Priority, D::Error>
        where D: Deserializer<'de>
    {
        let raw: i8 = Deserialize::deserialize(deserializer)?;

        Priority::from_int(raw)
            .ok_or_else(|| de::Error::invalid_value(de::Unexpected::Signed(raw as i64), &"-2, -1, 0, 1, 2"))
    }
}