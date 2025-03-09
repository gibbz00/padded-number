use std::ops::{Add, Sub};

use crate::*;

impl<const A: u8, const B: u8> PaddedNumber<A, B> {
    /// Wrapping addition with u64 as right-hand side
    ///
    /// Used internally for the `impl Add<u64> for PaddedNumber` implementation.
    ///
    /// ```rust
    /// # use padded_number_macros::*;
    /// assert_eq!(padded_number!("0").wrapping_add(1), padded_number!("0") + 1);
    ///
    /// // Within bounds
    /// assert_eq!(padded_number!("9") + 1, padded_number!("00"));
    /// assert_eq!(padded_number!("80") + 11, padded_number!("91"));
    ///
    /// // Wrapped
    /// assert_eq!(
    ///     bound_padded_number!(2, 3, "999") + 2,
    ///     bound_padded_number!(2, 3, "01")
    /// );
    /// ```
    pub fn wrapping_add(self, rhs: u64) -> Self {
        self.add_impl(rhs, |new_number, max_for_length_number| {
            let start = Self::min_number_for_min_length();

            match B == 0 {
                true => start,
                // -1 because '0' counts as a step
                false => start.wrapping_add(new_number - max_for_length_number - 1),
            }
        })
    }

    /// Saturating addition with u64 as right-hand side
    ///
    /// ```rust
    /// # use padded_number_macros::*;
    /// assert_eq!(
    ///     bound_padded_number!(2, 3, "990").saturating_add(1000),
    ///     bound_padded_number!(2, 3, "999") // saturated
    /// );
    /// ```
    ///
    /// Addition within bounds behaves the same as in [`Self::wrapping_add`].
    pub fn saturating_add(self, rhs: u64) -> Self {
        self.add_impl(rhs, |_new_number, max_for_length_number| Self {
            leading_zeros: 0,
            number: max_for_length_number,
        })
    }

    /// Wrapping subtraction with u64 as right-hand side
    ///
    /// Used internally for the `impl Sub<u64> for PaddedNumber` implementation.
    ///
    /// ```rust
    /// # use padded_number_macros::*;
    /// assert_eq!(padded_number!("9").wrapping_sub(1), padded_number!("9") - 1);
    ///
    /// // Within bounds
    /// assert_eq!(padded_number!("9") + 1, padded_number!("00"));
    /// assert_eq!(padded_number!("80") + 11, padded_number!("91"));
    ///
    /// // Wrapped
    /// assert_eq!(
    ///     bound_padded_number!(2, 3, "999") + 2,
    ///     bound_padded_number!(2, 3, "01")
    /// );
    /// ```
    pub fn wrapping_sub(self, rhs: u64) -> Self {
        self.sub_impl(rhs, |remaining_difference| {
            Self::max_number_for_max_length().wrapping_sub(remaining_difference)
        })
    }

    /// Saturating subtraction with u64 as right-hand side
    ///
    /// ```rust
    /// # use padded_number_macros::*;
    /// assert_eq!(
    ///     bound_padded_number!(1, 2, "99").saturating_sub(1000),
    ///     bound_padded_number!(1, 2, "0") // saturated
    /// );
    /// ```
    ///
    /// Subtraction within bounds behaves the same as in [`Self::wrapping_sub`].
    pub fn saturating_sub(self, rhs: u64) -> Self {
        self.sub_impl(rhs, |_| Self::min_number_for_min_length())
    }

    // can't yet be const due to Rust currently missing const closures
    fn add_impl(self, rhs: u64, overflow_fn: fn(u64, u64) -> Self) -> Self {
        if rhs == 0 {
            return self;
        }

        let new_number = self.number + rhs;

        let max_for_length = self.max_number_for_current_length();
        let max_for_length_number = max_for_length.number;

        // check for overflow
        if new_number > max_for_length_number {
            // handle overflow
            if self.len() == B {
                overflow_fn(new_number, max_for_length_number)
            }
            // recursively add one leading zero
            else {
                let next_number = Self { leading_zeros: self.len() + 1, number: 0 };

                let diff_to_next_increase = max_for_length_number - self.number;
                let rhs_next = rhs - diff_to_next_increase - 1;

                next_number.add_impl(rhs_next, overflow_fn)
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

    fn sub_impl(self, rhs: u64, overflow_fn: impl FnOnce(u64) -> Self) -> Self {
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

                if length == A {
                    // -1 because '0' counts as a step
                    overflow_fn(rhs - self.number - 1)
                } else {
                    let next_number =
                        PaddedNumber { leading_zeros: 0, number: Self::max_number_for_length_impl(length - 1) };

                    let overflow_diff = rhs - self.number;
                    let next_rhs = overflow_diff - 1;

                    next_number.sub_impl(next_rhs, overflow_fn)
                }
            }
        }
    }

    const fn max_number_for_current_length(&self) -> PaddedNumber<A, B> {
        PaddedNumber { leading_zeros: 0, number: Self::max_number_for_length_impl(self.len()) }
    }

    const fn max_number_for_max_length() -> PaddedNumber<A, B> {
        PaddedNumber { leading_zeros: 0, number: Self::max_number_for_length_impl(B) }
    }

    const fn min_number_for_min_length() -> Self {
        Self { leading_zeros: A, number: 0 }
    }

    const fn max_number_for_length_impl(length: u8) -> u64 {
        (10_u64.pow(length as u32)) - 1
    }
}

impl<const A: u8, const B: u8> Add<u64> for PaddedNumber<A, B> {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self::wrapping_add(self, rhs)
    }
}

impl<const A: u8, const B: u8> Sub<u64> for PaddedNumber<A, B> {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        Self::wrapping_sub(self, rhs)
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
    use crate::{tests::mock_from_str, *};

    #[test]
    fn max_number_for_current_length() {
        assert_impl("", "");
        assert_impl("9", "0");
        assert_impl("99", "00");
        assert_impl("999", "001");
        assert_impl("9999", "1234");

        fn assert_impl(max_number_str: &str, number_str: &str) {
            let expected_max_number = mock_from_str::<0, 10>(max_number_str);
            let actual_max_number = mock_from_str::<0, 10>(number_str).max_number_for_current_length();
            assert_eq!(expected_max_number, actual_max_number);
        }
    }

    #[test]
    fn max_number_for_max_length() {
        assert_impl::<0>("");
        assert_impl::<1>("9");
        assert_impl::<2>("99");

        fn assert_impl<const B: u8>(expected_max_number_str: &str) {
            let expected_max_number = mock_from_str::<0, B>(expected_max_number_str);
            let actual_max_number = PaddedNumber::<0, B>::max_number_for_max_length();
            assert_eq!(expected_max_number, actual_max_number);
        }
    }

    #[test]
    fn non_overflowing_add() {
        // zero is no-op
        assert_non_overflowing_add("00", ("00", 0));

        // from empty
        assert_non_overflowing_add("0", ("", 1));

        // adds leading zero on increment
        assert_non_overflowing_add("00", ("9", 1));
        assert_non_overflowing_add("0000", ("999", 1));
        assert_non_overflowing_add("000", ("9", 101));

        // inner decimal increment does not add leading zero
        assert_non_overflowing_add("99", ("00", 99));
        assert_non_overflowing_add("001", ("000", 1));
        assert_non_overflowing_add("100", ("099", 1));
        assert_non_overflowing_add("1099", ("0099", 1000));

        fn assert_non_overflowing_add(expected: &str, (lhs, rhs): (&str, u64)) {
            assert_arithmetic::<0, 10>(expected, (lhs, rhs), |lhs, rhs| {
                lhs.add_impl(rhs, |_, _| panic!("overflow occurred when testing non overflows"))
            });
        }
    }

    #[test]
    fn wrapping_add() {
        // wrappes at next
        assert_wrapping_add::<0, 10>("", ("9999999999", 1));
        // wrappes at next to min for length
        assert_wrapping_add::<2, 10>("00", ("9999999999", 1));
        // wrappes past last
        assert_wrapping_add::<1, 10>("01", ("9999999900", 111));
        // wrappes at empty
        assert_wrapping_add::<0, 0>("", ("", 10));

        fn assert_wrapping_add<const A: u8, const B: u8>(expected: &str, (lhs, rhs): (&str, u64)) {
            assert_arithmetic(expected, (lhs, rhs), PaddedNumber::<A, B>::wrapping_add);
        }
    }

    #[test]
    fn saturating_add() {
        // saturates at next
        assert_saturating_add::<0, 10>("9999999999", ("9999999999", 1));
        // saturates until last
        assert_saturating_add::<0, 10>("9999999999", ("9999999900", 1000));
        // saturates at empty
        assert_saturating_add::<0, 0>("", ("", 10));

        // doesn't saturate just because of max length
        assert_saturating_add::<0, 10>("9999999992", ("9999999990", 2));

        fn assert_saturating_add<const A: u8, const B: u8>(expected: &str, (lhs, rhs): (&str, u64)) {
            assert_arithmetic(expected, (lhs, rhs), PaddedNumber::<A, B>::saturating_add);
        }
    }

    #[test]
    fn non_overflowing_sub() {
        // zero is no-op
        assert_non_overflowing_sub("00", ("00", 0));

        // within does not overflow, digit count is preserved
        assert_non_overflowing_sub("0", ("9", 9));
        assert_non_overflowing_sub("0000", ("0100", 100));
        assert_non_overflowing_sub("099", ("100", 1));
        assert_non_overflowing_sub("050", ("100", 50));

        // within overflows, decrease digit count
        assert_non_overflowing_sub("", ("0", 1));
        assert_non_overflowing_sub("9", ("00", 1));
        assert_non_overflowing_sub("09", ("090", 181));
        assert_non_overflowing_sub("999", ("0100", 101));
        assert_non_overflowing_sub("995", ("0100", 105));

        fn assert_non_overflowing_sub(expected: &str, (lhs, rhs): (&str, u64)) {
            assert_arithmetic::<0, 10>(expected, (lhs, rhs), |lhs, rhs| {
                lhs.sub_impl(rhs, |_| panic!("overflow occurred when testing non overflows"))
            });
        }
    }

    #[test]
    fn wrapping_sub() {
        // wrappes at next
        assert_wrapping_sub::<1, 1>("9", ("0", 1));
        // wrappes to max digit count
        assert_wrapping_sub::<1, 3>("999", ("0", 1));
        // wrappes past last
        assert_wrapping_sub::<1, 3>("995", ("01", 16));
        // wrappes at empty
        assert_wrapping_sub::<0, 0>("", ("", 10));

        fn assert_wrapping_sub<const A: u8, const B: u8>(expected: &str, (lhs, rhs): (&str, u64)) {
            assert_arithmetic(expected, (lhs, rhs), PaddedNumber::<A, B>::wrapping_sub);
        }
    }

    #[test]
    fn saturating_sub() {
        // saturates at next
        assert_saturating_sub::<1, 1>("0", ("0", 1));
        assert_saturating_sub::<2, 2>("00", ("00", 1));
        // saturates until last
        assert_saturating_sub::<0, 2>("", ("00", 1000));
        assert_saturating_sub::<1, 2>("0", ("00", 1000));
        assert_saturating_sub::<1, 2>("0", ("90", 1000));
        assert_saturating_sub::<1, 2>("0", ("99", 1000));

        fn assert_saturating_sub<const A: u8, const B: u8>(expected: &str, (lhs, rhs): (&str, u64)) {
            assert_arithmetic(expected, (lhs, rhs), PaddedNumber::<A, B>::saturating_sub);
        }
    }

    fn assert_arithmetic<const A: u8, const B: u8>(
        expected: &str,
        (lhs, rhs): (&str, u64),
        operator: impl Fn(PaddedNumber<A, B>, u64) -> PaddedNumber<A, B>,
    ) {
        let expected = mock_from_str::<A, B>(expected);
        let actual = operator(mock_from_str::<A, B>(lhs), rhs);
        assert_eq!(
            expected, actual,
            "failed to operate on '{}' with {} to in the end expect '{}'",
            lhs, rhs, expected
        );
    }
}
