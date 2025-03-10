use crate::*;

impl<const MIN: u8, const MAX: u8> PaddedNumber<MIN, MAX> {
    /// Get a section of a padded number, missing digits not allowed.
    ///
    /// First generic parameter is the start index, inclusive. Second parameter
    /// denotes the end index, exclusive. Remaining bound checks are enforced by
    /// the type system. E.g. end >= start and end <= max length.
    ///
    /// Returns `None` if the end index overflowed for a padded number whose
    /// length is less than `END_INDEX`.
    ///
    /// Returned max and min sizes always be further relaxed with
    /// [`PaddedNumber::resize`].
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
        [(); { MAX - END_INDEX } as usize]:,
    {
        if END_INDEX > self.len() {
            return None;
        }

        let (leading_zeros, number) = section_impl(self.leading_zeros, self.number, START_INDEX, END_INDEX);
        Some(PaddedNumber { leading_zeros, number })
    }

    /// Get a section of a padded number, missing digits allowed.
    ///
    /// `START`, as the names suggests, denotes the start index on the padded
    /// number to select from, inclusive. `END` index is on the other hand
    /// exclusive.
    ///
    /// Remaining bounds are statically verified. E.g.:
    /// - `START` < `END`,
    /// - `NEW_MIN` <= `END` - `START`
    ///
    /// Returns `None` if the returned section length would be less than the
    /// specified `NEW_MIN` parameter. As for the maximum length, `END - START`
    /// is provided from the method. [`PaddedNumber::resize`] can then be used
    /// to relax these bounds even further.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![feature(generic_const_exprs)]
    ///
    /// # use padded_number_macros::*;
    /// let section = padded_number!("00123")
    ///     .relaxed_section::<3, 10, 1>()
    ///     .expect("invalid min length");
    ///
    /// assert_eq!(section, bound_padded_number!(1, 7, "23"));
    ///
    /// let section = bound_padded_number!(1, 10, "00").relaxed_section::<5, 7, 1>();
    /// // no digits between index 5 and 7, and at least 1 was required
    /// assert!(section.is_none());
    /// ```
    ///
    /// ```compile_fail
    /// #![feature(generic_const_exprs)]
    ///
    /// # use padded_number_macros::*;
    /// let section = bound_padded_number!(1, 5, "123");
    /// section.checked_section::<2, 3, 10>(); // <-- NEW_MIN '10' > END '2' - START '1'
    /// ```
    pub fn relaxed_section<const START: u8, const END: u8, const NEW_MIN: u8>(
        &self,
    ) -> Option<PaddedNumber<NEW_MIN, { END - START }>>
    where
        [(); { MAX - END } as usize]:,
        [(); { (END - START) - NEW_MIN } as usize]:,
    {
        let remaining_length = self.len().saturating_sub(START);

        if remaining_length < NEW_MIN {
            return None;
        }

        if remaining_length == 0 {
            return Some(PaddedNumber { leading_zeros: 0, number: 0 });
        }

        let (leading_zeros, number) = section_impl(self.leading_zeros, self.number, START, START + remaining_length);

        Some(PaddedNumber { leading_zeros, number })
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
        [(); { MIN - END_INDEX } as usize]:,
        // Bound required to call `Self::section_impl`
        [(); { MAX - END_INDEX } as usize]:,
    {
        // safe to call since end <= min_length
        let (leading_zeros, number) = section_impl(self.leading_zeros, self.number, START_INDEX, END_INDEX);
        PaddedNumber { leading_zeros, number }
    }
}

/// # Panics
/// - If !(start <= end <= remaining_number_length)
fn section_impl(leading_zeros: u8, remaining_number: u64, start: u8, end: u8) -> (u8, u64) {
    match (start.checked_sub(leading_zeros), end.checked_sub(leading_zeros)) {
        (Some(translated_start), Some(translated_end)) => {
            (0, number_subsection(remaining_number, translated_start, translated_end))
        }
        (None, Some(translated_end)) => (
            leading_zeros - start,
            number_subsection(remaining_number, 0, translated_end),
        ),
        (None, None) => {
            let leading_zero_start = leading_zeros - start;
            let leading_zero_end = leading_zeros - end;
            (leading_zero_start - leading_zero_end, 0)
        }
        (Some(_), None) => {
            panic!("encountered start > end")
        }
    }
}

/// # Panics
/// - If start > number_length
/// - If end > number_length
pub(crate) const fn number_subsection(number: u64, start: u8, end: u8) -> u64 {
    let number_length = utils::number_len(number);

    if number_length == 0 {
        return 0;
    }

    let left_shifts = start;
    let right_shifts = number_length - end;

    let mut number = number;
    number = left_shift_repeated(number, number_length, left_shifts);
    number = right_shift_repeated(number, right_shifts);

    return number;

    /// # Panics
    /// - If repetitions > number_length
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

    /// # Panics
    /// - If number_length == 0
    const fn left_shift(number: u64, number_length: u8) -> u64 {
        let decimal = 10u64.pow((number_length - 1) as u32);
        number - (number / decimal) * decimal
    }

    const fn right_shift_repeated(number: u64, repetitions: u8) -> u64 {
        number / 10u64.pow(repetitions as u32)
    }
}
