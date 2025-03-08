// TEMP: remove before first publish
#![allow(missing_docs)]

//! # `padded-number-padded-number` - Unsigned number with significant leading zeros
//!
//! Used when "0" and "00" should be considered as distinct values. Comes with
//! ordering, addition, subtraction, and length bounds features included.
//!
//!
//! Generic parameter `A` denotes the minimum digit count, inclusive.
//! Generic parameter `B` denotes maximum digit count, inclusive.
//!
//! `A < B` allows for variable digit length.
//! `A == B` requires the digit to exactly of length A (or B).
//! `A > B, where A, B > 0` is technically declarable, but attempts at
//! constructing a value will result in a runtime error.
//! `A == 0` results in empty values being allowed as valid numbers. ("")

mod core;
pub use core::PaddedNumber;

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
