#![doc = include_str!(concat!("../", env!("CARGO_PKG_README")))]

pub use padded_number_internal::{PaddedNumber, ParsePaddedNumberError};
#[cfg(feature = "macros")]
pub use padded_number_macros::{bound_padded_number, padded_number};
