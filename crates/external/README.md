# `padded-number` - For numbers containing significant leading zeros

[![ci_status](https://img.shields.io/github/actions/workflow/status/gibbz00/padded-number/ci.yaml?style=for-the-badge)](https://github.com/gibbz00/padded-number/actions/workflows/ci.yaml)
[![codecov](https://img.shields.io/codecov/c/gh/gibbz00/padded-number?token=8uQpdikxPA&style=for-the-badge)](https://codecov.io/gh/gibbz00/padded-number)
[![license](https://img.shields.io/github/license/gibbz00/padded-number.svg?style=for-the-badge)](https://github.com/gibbz00/padded-number/blob/main/LICENSE.md)
[![crates_io](https://img.shields.io/crates/v/padded-number.svg?style=for-the-badge)](https://crates.io/crates/padded-number)
[![docs_rs](https://img.shields.io/docsrs/padded-number/latest.svg?style=for-the-badge)](https://docs.rs/padded-number)

Used when "0" and "00" should be considered as distinct values. Encapsulated
in a `PaddedNumber` type with, length bounds, ordering, arithmetic, and const
context features included.

```rust
use padded_number::padded_number;

// macros creates a valid `PaddedNumber` at compile time
assert_eq!(padded_number!("001"), padded_number!("001"));
assert_ne!(padded_number!("0"), padded_number!("00"));
```

## Length bounds

```rust
use std::str::FromStr;
use padded_number::{PaddedNumber, bound_padded_number};

let from_macro = bound_padded_number!(2, 3, "123");
let from_str = PaddedNumber::<2, 3>::from_str("123").unwrap();
assert_eq!(from_macro, from_str);

// try_new is const fn compared to `FromStr`
assert!(PaddedNumber::<2, 3>::try_new("0").is_err());
assert!(PaddedNumber::<2, 3>::try_new("0000").is_err());
```

## Ordering

```rust
use padded_number::padded_number;

let a = padded_number!("0");
let b = padded_number!("00");
assert!(a < b);

let u = padded_number!("10");
let v = padded_number!("001");
assert!(u < v);
```

## Addition and subtraction with u64 as right-hand-side

Zeros being their own step is required to make padded number arithmetic consistent.

```rust
use padded_number::padded_number;

assert_eq!(
  padded_number!("9") + 1,
  padded_number!("00")
);

assert_eq!(
  padded_number!("000") - 1,
  padded_number!("99")
);
```

## Feature flags

All are disabled by default.

- `macros` - Enables the `padded_number!` and `bound_padded_number!` macros.
- `serde` - Enabled serde support for `PaddedNumber`. Serialization is done to and from a plain string.
