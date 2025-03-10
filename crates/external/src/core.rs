use crate::*;

/// Newtype encapsulating the padded number invariants
///
/// Consists only of an `u8` and an `u64` which keep track of the
/// leading zeros count and the remaining number value respectively.
///
/// Check out the crate-level documentation for an introduction.
///
/// `PaddedNumber` uses const generic parameters for setting lower (inclusive)
/// and upper (inclusive) length bounds. These parameters are by default set to
/// 1 and 255 (u8::MAX) respectively.
///
/// - `MIN < MAX` allows for variable digit length.
/// - `MIN == MAX` requires the digit to be exactly of length MIN/MAX.
/// - `MIN == 0` results in empty values ("") being allowed as valid numbers.
/// - `MIN > MAX, where MIN, MAX > 0` is technically declarable, but any
///   attempts at constructing such a padded number will fail.
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct PaddedNumber<const A: u8 = 1, const B: u8 = { u8::MAX }> {
    pub(crate) leading_zeros: u8,
    pub(crate) number: u64,
}

impl<const A: u8, const B: u8> PaddedNumber<A, B> {
    #[doc(hidden)]
    pub const unsafe fn new_unchecked(leading_zeros: u8, number: u64) -> Self {
        Self { leading_zeros, number }
    }

    /// Create a new [`PaddedNumber`]
    pub const fn try_new(str: &str) -> Result<Self, ParsePaddedNumberError> {
        let (leading_zeros, remaining_number) = konst::try_!(padded_number_internal::parse(A, B, str));

        Ok(Self { leading_zeros, number: remaining_number })
    }

    /// Calculate the length of the padded number, including any leading zeros
    ///
    /// ```rust
    /// # use padded_number::*;
    /// assert_eq!(2, padded_number!("01").len());
    /// assert_eq!(3, padded_number!("123").len());
    /// ```
    pub const fn len(&self) -> u8 {
        self.leading_zeros + utils::number_len(self.number)
    }

    /// Check if the number if empty, e.g. if and only if it is `""`.
    /// ```rust
    /// # use padded_number::*;
    /// assert!(bound_padded_number!(0, 1, "").is_empty());
    /// assert!(!padded_number!("01").is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.leading_zeros == 0 && self.number == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::mock_from_str;

    #[test]
    fn new_with_leading_zeros() {
        let number = mock_from_str::<1, 3>("001");
        assert_eq!(2, number.leading_zeros);
    }

    #[test]
    fn new_with_leading_zeros_only() {
        let number = mock_from_str::<1, 3>("000");
        let expected = PaddedNumber { leading_zeros: 3, number: 0 };
        assert_eq!(expected, number)
    }

    #[test]
    fn new_with_empty_str() {
        let number = mock_from_str::<0, 0>("");
        assert!(number.is_empty());
    }

    #[test]
    fn too_long_error() {
        let invalid_number = "123";

        let actual_err = invalid_number.parse::<PaddedNumber<1, 2>>().unwrap_err();

        assert_eq!(ParsePaddedNumberError::TooLong(2, 3), actual_err);
    }

    #[test]
    fn too_short_error() {
        let invalid_number = "";

        let actual_err = invalid_number.parse::<PaddedNumber<1, 2>>().unwrap_err();

        assert_eq!(ParsePaddedNumberError::TooShort(1, 0), actual_err);
    }

    #[test]
    fn non_ascii_digits_error() {
        let invalid_number = "123abc";

        let actual_err = invalid_number.parse::<PaddedNumber<0, 10>>().unwrap_err();

        assert!(matches!(actual_err, ParsePaddedNumberError::InvalidNumber(_)));
    }

    #[test]
    fn is_empty() {
        let number = PaddedNumber::<0, 0> { leading_zeros: 0, number: 0 };
        assert!(number.is_empty())
    }

    #[test]
    fn length() {
        assert_len(0, "");
        assert_len(1, "0");
        assert_len(3, "000");
        assert_len(3, "467");
        assert_len(5, "00467");

        fn assert_len(expected_length: u8, number_str: &str) {
            let number = mock_from_str::<0, 10>(number_str);
            assert_eq!(expected_length, number.len());
        }
    }
}
