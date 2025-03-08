use ::serde::{Deserialize, Serialize};

use crate::*;

impl<const A: u8, const B: u8> Serialize for PaddedNumber<A, B> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, const A: u8, const B: u8> Deserialize<'de> for PaddedNumber<A, B> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let str = std::borrow::Cow::<str>::deserialize(deserializer)?;
        str.as_ref().parse().map_err(::serde::de::Error::custom)
    }
}
