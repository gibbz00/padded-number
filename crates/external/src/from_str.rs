use std::str::FromStr;

use crate::*;

impl<const A: u8, const B: u8> FromStr for PaddedNumber<A, B> {
    type Err = ParsePaddedNumberError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Self::try_new(str)
    }
}
