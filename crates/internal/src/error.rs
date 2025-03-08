/// Error originating from [`PaddedNumber::try_new`]
///
/// [`PaddedNumber::try_new`]: crate::PaddedNumber::try_new
#[derive(Debug, PartialEq, displaydoc::Display, thiserror::Error)]
pub enum ParsePaddedNumberError {
    /// "too few digits provided, expected at least '{0}', received '{1}'"
    TooShort(u8, u8),
    /// "too many digits provided, expected at most '{0}', received '{1}'"
    TooLong(u8, u8),
    /// "integer parse error, encountered non-ascii digit"
    InvalidNumber(#[source] std::num::ParseIntError),
}
