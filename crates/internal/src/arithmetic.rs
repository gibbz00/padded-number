use std::ops::{Add, Sub};

use crate::*;

impl<const A: u8, const B: u8> PaddedNumber<A, B> {
    const fn max_number_for_length(&self) -> PaddedNumber<A, B> {
        PaddedNumber { leading_zeros: 0, number: Self::max_number_for_length_impl(self.len()) }
    }

    const fn max_number_for_length_impl(length: u8) -> u64 {
        (10_u64.pow(length as u32)) - 1
    }

    pub const fn saturating_add(self, rhs: u64) -> Self {
        if rhs == 0 {
            return self;
        }

        let new_number = self.number + rhs;

        let max_for_length = self.max_number_for_length();
        let max_for_length_number = max_for_length.number;

        // check for overflow
        if new_number > max_for_length_number {
            // saturate
            if self.len() == B {
                max_for_length
            }
            // recursively add one leading zero
            else {
                let next_number = Self { leading_zeros: self.len() + 1, number: 0 };

                let diff_to_next_increase = max_for_length_number - self.number;
                let rhs_next = rhs - diff_to_next_increase - 1;

                next_number.saturating_add(rhs_next)
            }
        }
        // no overflow, preserve number of digits
        else {
            let new_number_digits = digit_length(new_number);
            let current_digits = digit_length(self.number);
            let leading_zeroes_to_remove = new_number_digits - current_digits;

            Self {
                leading_zeros: self.leading_zeros - leading_zeroes_to_remove,
                number: new_number,
            }
        }
    }

    pub const fn saturating_sub(self, rhs: u64) -> Self {
        if rhs == 0 || self.is_empty() {
            return self;
        }

        match self.number.checked_sub(rhs) {
            // within does not overflow, preserve length
            Some(new_number) => {
                let current_digits = digit_length(self.number);
                let new_number_digits = digit_length(new_number);
                let leading_zeroes_to_add = current_digits - new_number_digits;

                Self {
                    leading_zeros: self.leading_zeros + leading_zeroes_to_add,
                    number: new_number,
                }
            }
            // within overflows
            None => {
                let length = self.len();

                // saturate
                if length == A {
                    PaddedNumber { leading_zeros: A, number: 0 }
                } else {
                    let next_number =
                        PaddedNumber { leading_zeros: 0, number: Self::max_number_for_length_impl(length - 1) };

                    let overflow_diff = rhs - self.number;
                    let next_rhs = overflow_diff - 1;

                    next_number.saturating_sub(next_rhs)
                }
            }
        }
    }
}

impl<const A: u8, const B: u8> Add<u64> for PaddedNumber<A, B> {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self::saturating_add(self, rhs)
    }
}

impl<const A: u8, const B: u8> Sub<u64> for PaddedNumber<A, B> {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        Self::saturating_sub(self, rhs)
    }
}

const fn digit_length(n: u64) -> u8 {
    if n == 0 {
        0
    } else {
        let mut count = 1;
        let mut current = n;

        while current >= 10 {
            count += 1;
            current /= 10
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::mock_from_str;

    #[test]
    fn max_number_for_length() {
        assert_impl("", "");
        assert_impl("9", "0");
        assert_impl("99", "00");
        assert_impl("999", "001");
        assert_impl("9999", "1234");

        fn assert_impl(max_number_str: &str, number_str: &str) {
            let expected_max_number = mock_from_str::<0, 10>(max_number_str);
            let actual_max_number = mock_from_str::<0, 10>(number_str).max_number_for_length();
            assert_eq!(expected_max_number, actual_max_number);
        }
    }

    #[test]
    fn saturating_add() {
        // zero is no-op
        assert_saturating_add("00", ("00", 0));

        // from empty
        assert_saturating_add_impl::<0, 1>("0", ("", 1));

        // adds leading zero on increment
        assert_saturating_add("00", ("9", 1));
        assert_saturating_add("0000", ("999", 1));
        assert_saturating_add("000", ("9", 101));

        // inner decimal increment does not add leading zero
        assert_saturating_add("99", ("00", 99));
        assert_saturating_add("001", ("000", 1));
        assert_saturating_add("100", ("099", 1));
        assert_saturating_add("1099", ("0099", 1000));

        // saturates at next
        assert_saturating_add("9999999999", ("9999999999", 1));
        // saturates until last
        assert_saturating_add("9999999999", ("9999999900", 1000));
        // saturates at empty
        assert_saturating_add_impl::<0, 0>("", ("", 10));

        // doesn't saturate just because of max length
        assert_saturating_add("9999999992", ("9999999990", 2));

        fn assert_saturating_add(expected: &str, (lhs, rhs): (&str, u64)) {
            assert_saturating_add_impl::<0, 10>(expected, (lhs, rhs));
        }

        fn assert_saturating_add_impl<const A: u8, const B: u8>(expected: &str, (lhs, rhs): (&str, u64)) {
            let expected = mock_from_str::<A, B>(expected);
            let actual = mock_from_str::<A, B>(lhs) + rhs;
            assert_eq!(expected, actual, "failed to add '{}' with {}", lhs, rhs);
        }
    }

    #[test]
    fn saturating_sub() {
        // zero is no-op
        assert_saturating_sub("00", ("00", 0));

        // saturates at next
        assert_saturating_sub_impl::<0, 1>("", ("0", 1));
        assert_saturating_sub_impl::<1, 1>("0", ("0", 1));
        assert_saturating_sub_impl::<2, 2>("00", ("00", 1));
        // saturates until last
        assert_saturating_sub_impl::<0, 2>("", ("00", 1000));
        assert_saturating_sub_impl::<1, 2>("0", ("00", 1000));
        assert_saturating_sub_impl::<1, 2>("0", ("90", 1000));
        assert_saturating_sub_impl::<1, 2>("0", ("99", 1000));

        // within overflows, decrease digit count
        assert_saturating_sub("", ("0", 1));
        assert_saturating_sub("9", ("00", 1));
        assert_saturating_sub("09", ("090", 181));
        assert_saturating_sub("999", ("0100", 101));
        assert_saturating_sub("995", ("0100", 105));

        // within does not overflow, digit count is preserved
        assert_saturating_sub("0", ("9", 9));
        assert_saturating_sub("0000", ("0100", 100));
        assert_saturating_sub("099", ("100", 1));
        assert_saturating_sub("050", ("100", 50));

        fn assert_saturating_sub(expected: &str, (lhs, rhs): (&str, u64)) {
            assert_saturating_sub_impl::<0, 10>(expected, (lhs, rhs));
        }

        fn assert_saturating_sub_impl<const A: u8, const B: u8>(expected: &str, (lhs, rhs): (&str, u64)) {
            let expected = mock_from_str::<A, B>(expected);
            let lhs = mock_from_str::<A, B>(lhs);
            assert_eq!(expected, lhs - rhs);
        }
    }
}
