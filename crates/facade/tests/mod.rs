#![allow(missing_docs)]

use padded_number::{PaddedNumber, bound_padded_number, padded_number};

#[test]
fn bound_const() {
    const BOUNDED_FROM_MACRO: PaddedNumber<1, 10> = bound_padded_number!(1, 10, "001");

    assert_eq!(PaddedNumber::try_new("001").unwrap(), BOUNDED_FROM_MACRO);
}

#[test]
fn unbound_const() {
    const FROM_MACRO: PaddedNumber = padded_number!("001");

    assert_eq!(PaddedNumber::try_new("001").unwrap(), FROM_MACRO);
}
