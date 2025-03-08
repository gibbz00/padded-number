#![allow(missing_docs)]

use padded_number::{PaddedNumber, bound_padded_number};

const FROM_MACRO: PaddedNumber<1, 10> = bound_padded_number!(1, 10, "001");

const FROM_MANUAL: PaddedNumber<1, 10> = const {
    let Ok(num) = PaddedNumber::try_new("001") else {
        panic!("oh no! not a valid padded number")
    };

    num
};

#[test]
fn const_declarations() {
    assert_eq!(FROM_MACRO, FROM_MANUAL);
}
