//! # `padded-number-internal` - Internal crate whose items are then re-exported in `padded-number`

#[doc(hidden)]
pub mod parse;

mod error;
pub use error::ParsePaddedNumberError;
