# `padded-number` - For numbers with significant leading zeros

[![ci_status](https://img.shields.io/github/actions/workflow/status/gibbz00/padded-number/ci.yaml?style=for-the-badge)](https://github.com/gibbz00/padded-number/actions/workflows/ci.yaml)
[![codecov](https://img.shields.io/codecov/c/gh/gibbz00/padded-number?token=8uQpdikxPA&style=for-the-badge)](https://codecov.io/gh/gibbz00/padded-number)
[![license](https://img.shields.io/github/license/gibbz00/padded-number.svg?style=for-the-badge)](https://github.com/gibbz00/padded-number/blob/main/LICENSE.md)

Used when "0" and "00" should be considered as distinct values. Comes with
ordering, addition, subtraction, length bounds, const context features included.

Generic parameter `A` denotes the minimum digit count, inclusive.
Generic parameter `B` denotes maximum digit count, inclusive.

`A < B` allows for variable digit length.
`A == B` requires the digit to exactly of length A (or B).
`A > B, where A, B > 0` is technically declarable, but attempts at
constructing a value will result in a runtime error.
`A == 0` results in empty values being allowed as valid numbers. ("")
