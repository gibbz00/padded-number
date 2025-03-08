use crate::*;

/// Checkout the crate-level documentation for an introduction
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct PaddedNumber<const A: u8, const B: u8> {
    pub(crate) leading_zeros: u8,
    pub(crate) number: u64,
}

impl<const A: u8, const B: u8> PaddedNumber<A, B> {
    /// Create a new [`PaddedNumber`] in a const context.
    pub const fn try_new(str: &str) -> Result<Self, ParsePaddedNumberError> {
        {
            let str_len = str.len();

            if str_len == 0 && A == 0 {
                return Ok(Self { leading_zeros: 0, number: 0 });
            }

            if str_len < A as usize {
                return Err(ParsePaddedNumberError::TooShort(A, str_len as u8));
            }

            if str_len > B as usize {
                return Err(ParsePaddedNumberError::TooLong(B, str_len as u8));
            }
        }

        let leading_zeros =
            konst::iter::eval!(konst::string::chars(str), take_while(|char| *char == '0'), count()) as u8;

        let remaining_number = konst::try_!(konst::result::map_err!(
            u64::from_str_radix(str, 10),
            ParsePaddedNumberError::InvalidNumber
        ));

        Ok(Self { leading_zeros, number: remaining_number })
    }

    /// Calculate the length of the padded number, including any leading zeros
    pub const fn len(&self) -> u8 {
        if self.number == 0 {
            self.leading_zeros
        } else {
            let mut number_length = 1;
            let mut remaining_number = self.number;

            while remaining_number >= 10 {
                number_length += 1;
                remaining_number /= 10;
            }

            self.leading_zeros + number_length
        }
    }

    /// Check if the number if empty, e.g. when it is ''.
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
