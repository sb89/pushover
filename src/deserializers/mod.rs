use serde::de::{Deserialize, Deserializer};

pub fn deserialize_option_empty_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let result: Option<String> = Deserialize::deserialize(deserializer)?;

    match result {
        Some(ref x) if x.is_empty() => Ok(None),
        _ => Ok(result),
    }
}
