use serde::de::{self, Deserialize, Deserializer, Unexpected};

/// Deserialize 0 => false, 1 => true
pub(crate) fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

/// Deserialize null/false => false, true => true
pub(crate) fn bool_from_option_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match <Option<bool>>::deserialize(deserializer)? {
        Some(true) => Ok(true),
        _ => Ok(false),
    }
}
