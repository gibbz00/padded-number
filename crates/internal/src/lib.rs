//! # `padded-number-internal` - Internal crate whose items are then re-exported in `padded-number`

mod core;
pub use core::PaddedNumber;

#[doc(hidden)]
pub mod parse;

mod error;
pub use error::ParsePaddedNumberError;

mod arithmetic;
mod display;
mod from_str;
mod ordering;

#[cfg(feature = "serde")]
mod serde;
#[cfg(test)]
mod serde_tests;

// TEMP:
#[cfg(test)]
mod tests {
    use crate::*;

    pub fn mock_from_str<const A: u8, const B: u8>(number_str: &str) -> PaddedNumber<A, B> {
        number_str.parse().unwrap()
    }
}
