// Used for the `unstable-nightly` features
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
//
#![doc = include_str!(concat!("../", env!("CARGO_PKG_README")))]

#[cfg(feature = "macros")]
pub use padded_number_macros::{bound_padded_number, padded_number};

mod core;
pub use core::PaddedNumber;

pub use padded_number_internal::ParsePaddedNumberError;

mod arithmetic;
mod display;
mod from_str;
mod ordering;

#[cfg(feature = "serde")]
mod serde;
#[cfg(test)]
mod serde_tests;

#[cfg(feature = "unstable-nightly")]
mod section;
#[cfg(test)]
mod section_tests;

#[cfg(feature = "unstable-nightly")]
mod resize;
#[cfg(feature = "unstable-nightly")]
pub use resize::ResizePaddedNumber;

// TEMP:
#[cfg(test)]
mod tests {
    use crate::*;

    pub fn mock_from_str<const A: u8, const B: u8>(number_str: &str) -> PaddedNumber<A, B> {
        number_str.parse().unwrap()
    }
}
