use crate::*;

impl<const MIN_LENGTH: u8, const MAX_LENGTH: u8> PaddedNumber<MIN_LENGTH, MAX_LENGTH> {
    /// Get a exact section of a padded number, checking for overflows
    ///
    /// First generic parameter is the start index, inclusive. Second parameter
    /// denotes the end index, exclusive. Remaining bound checks are enforced by
    /// the type system. E.g. end >= start and end <= max length.
    ///
    /// Returns `None` if the end index overflowed for a padded whose length is
    /// is not equal to it's max length.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![feature(generic_const_exprs)]
    ///
    /// # use padded_number_macros::*;
    /// let section = padded_number!("00123")
    ///     .checked_section::<2, 5>()
    ///     .expect("section should not have overflowed");
    ///
    /// assert_eq!(section, bound_padded_number!(3, 3, "123"));
    ///
    /// let section = bound_padded_number!(1, 3, "0").checked_section::<1, 3>();
    /// // overflowed, missing two digits after "0"
    /// assert!(section.is_none());
    /// ```
    ///
    /// ```compile_fail
    /// #![feature(generic_const_exprs)]
    ///
    /// # use padded_number_macros::*;
    /// let section = bound_padded_number!(3, 3, "123");
    /// section.checked_section::<0, 4>(); // <-- END_INDEX '4' > MAX_LENGTH '3'
    /// ```
    ///
    /// ```compile_fail
    /// #![feature(generic_const_exprs)]
    ///
    /// # use padded_number_macros::*;
    /// let padded_number = bound_padded_number!(3, 3, "123");
    /// section.checked_section::<2, 1>(); // <-- END_INDEX '1' < START_INDEX '2'
    /// ```
    pub fn checked_section<const START_INDEX: u8, const END_INDEX: u8>(
        &self,
    ) -> Option<PaddedNumber<{ END_INDEX - START_INDEX }, { END_INDEX - START_INDEX }>>
    where
        // Expresses "END_INDEX <= MAX_LENGTH" with current
        // `generic_const_exprs` unstable feature capabilities
        [(); { MAX_LENGTH - END_INDEX } as usize]:,
    {
        if END_INDEX > self.len() {
            return None;
        }

        Some(self.section_impl())
    }

    /// Get a section from the minimum length of a padded number
    ///
    /// Unlike [`PaddedNumber::section`], this does not need to return an
    /// option. Type system ensures that END_INDEX <= MIN_LENGTH.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![feature(generic_const_exprs)]
    ///
    /// # use padded_number_macros::*;
    /// let section = bound_padded_number!(3, 5, "00123").expected_section::<0, 3>();
    /// assert_eq!(section, bound_padded_number!(3, 3, "001"));
    /// ```
    ///
    /// ```compile_fail
    /// #![feature(generic_const_exprs)]
    ///
    /// # use padded_number_macros::*;
    /// let section = bound_padded_number!(3, 5, "00123");
    /// section.expected_section::<0, 4>(); // <-- END_INDEX '4' > MIN_LENGTH '3'
    /// ```
    pub fn expected_section<const START_INDEX: u8, const END_INDEX: u8>(
        &self,
    ) -> PaddedNumber<{ END_INDEX - START_INDEX }, { END_INDEX - START_INDEX }>
    where
        // Expresses "END_INDEX <= MIN_LENGTH" with current
        // `generic_const_exprs` unstable feature capabilities
        [(); { MIN_LENGTH - END_INDEX } as usize]:,
        // Bound required to call `Self::section_impl`
        [(); { MAX_LENGTH - END_INDEX } as usize]:,
    {
        // save to call since END_INDEX <= MIN_LENGTH
        self.section_impl()
    }

    /// Assumes:
    /// - END_INDEX <= self.len()
    fn section_impl<const START_INDEX: u8, const END_INDEX: u8>(
        &self,
    ) -> PaddedNumber<{ END_INDEX - START_INDEX }, { END_INDEX - START_INDEX }>
    where
        // Expresses "END_INDEX <= MAX_LENGTH" with current
        // `generic_const_exprs` unstable feature capabilities
        [(); { MAX_LENGTH - END_INDEX } as usize]:,
    {
        let leading_zeros = self.leading_zeros;

        let (new_leading_zeros, new_number) = match (
            START_INDEX.checked_sub(leading_zeros),
            END_INDEX.checked_sub(leading_zeros),
        ) {
            (Some(translated_start), Some(translated_end)) => {
                (0, self.number_subsection(translated_start, translated_end))
            }
            (None, Some(translated_end)) => (leading_zeros - START_INDEX, self.number_subsection(0, translated_end)),
            (None, None) => {
                let leading_zero_start = leading_zeros - START_INDEX;
                let leading_zero_end = leading_zeros - END_INDEX;
                (leading_zero_start - leading_zero_end, 0)
            }
            (Some(_), None) => {
                // should not be possible with const generic expression
                // `{ END_INDEX - START_INDEX }` from method signature
                unreachable!("encountered start > end")
            }
        };

        PaddedNumber { leading_zeros: new_leading_zeros, number: new_number }
    }

    /// Assumes:
    /// - start <= number_length
    /// - end <= number_length
    pub(crate) const fn number_subsection(&self, start: u8, end: u8) -> u64 {
        let number_length = self.number_len();

        if number_length == 0 {
            return 0;
        }

        let left_shifts = start;
        let right_shifts = number_length - end;

        let mut number = self.number;
        number = left_shift_repeated(number, number_length, left_shifts);
        number = right_shift_repeated(number, right_shifts);

        return number;

        // assumes repetitions <= number_length, and number_length > 0
        const fn left_shift_repeated(number: u64, number_length: u8, repetitions: u8) -> u64 {
            let mut current = number;
            let mut current_length = number_length;
            let mut repetitions_left = repetitions;

            while repetitions_left > 0 {
                current = left_shift(current, current_length);
                current_length -= 1;
                repetitions_left -= 1;
            }

            current
        }

        // assumes number_length > 0
        const fn left_shift(number: u64, number_length: u8) -> u64 {
            let decimal = 10u64.pow((number_length - 1) as u32);
            number - (number / decimal) * decimal
        }

        const fn right_shift_repeated(number: u64, repetitions: u8) -> u64 {
            number / 10u64.pow(repetitions as u32)
        }
    }
}
