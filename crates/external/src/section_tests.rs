//! NOTE: not under feature flagged subset module to ensure tests are run with
//! `cargo test`

use crate::*;

const NUMBER_TO_SECTION: u64 = 123456;

#[test]
fn number_subsection_right_shifts() {
    assert_number_subsection(0, 1, 1);
    assert_number_subsection(0, 3, 123);
    assert_number_subsection(0, 6, 123456);
}

#[test]
fn number_subsection_combined_shifts() {
    assert_number_subsection(0, 0, 0);
    assert_number_subsection(5, 6, 6);
    assert_number_subsection(2, 4, 34);
    assert_number_subsection(1, 6, 23456);
}

fn assert_number_subsection(start: u8, end: u8, expected: u64) {
    let actual = crate::section::number_subsection(NUMBER_TO_SECTION, start, end);
    assert_eq!(expected, actual);
}

#[test]
fn overflow_returns_none() {
    let padded_number = PaddedNumber::<1, 5>::try_new("123").unwrap();

    assert!(padded_number.checked_section::<1, 5>().is_none());
    assert!(padded_number.checked_section::<3, 5>().is_none());
}

#[test]
fn zeros_section() {
    assert_section::<1, 7, 0, 3>("0001234", "000");
    assert_section::<1, 7, 2, 3>("0001234", "0");
    assert_section::<1, 7, 1, 2>("0001234", "0");
}

#[test]
fn number_section() {
    assert_section::<1, 7, 3, 7>("0001234", "1234");
    assert_section::<1, 7, 4, 5>("0001234", "2");
    assert_section::<1, 7, 6, 7>("0001234", "4");
}

#[test]
fn both_sections() {
    assert_section::<1, 7, 0, 7>("0001234", "0001234");
    assert_section::<1, 7, 2, 5>("0001234", "012");
}

#[test]
fn empty_section() {
    assert_section::<0, 0, 0, 0>("", "");
    assert_section::<1, 3, 3, 3>("000", "");
    assert_section::<1, 3, 0, 0>("000", "");
}

#[test]
fn expected_section() {
    let padded_number = PaddedNumber::<3, 5>::try_new("00123").unwrap();

    let actual_section = padded_number.expected_section::<0, 3>();
    let expected_section = PaddedNumber::try_new("001").unwrap();

    assert_eq!(expected_section, actual_section)
}

fn assert_section<const A: u8, const B: u8, const C: u8, const D: u8>(number_str: &str, expected_section: &str)
where
    [(); { D - C } as usize]:,
    [(); { B - D } as usize]:,
{
    let padded_number = PaddedNumber::<A, B>::try_new(number_str).unwrap();
    let actual_section = padded_number.checked_section::<C, D>().unwrap();
    let expected_section = PaddedNumber::try_new(expected_section).unwrap();
    assert_eq!(expected_section, actual_section)
}
