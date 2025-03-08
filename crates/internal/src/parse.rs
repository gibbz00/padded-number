use crate::*;

pub const fn parse(min: u8, max: u8, str: &str) -> Result<(u8, u64), ParsePaddedNumberError> {
    {
        let str_len = str.len();

        if str_len == 0 && min == 0 {
            return Ok((0, 0));
        }

        if str_len < min as usize {
            return Err(ParsePaddedNumberError::TooShort(min, str_len as u8));
        }

        if str_len > max as usize {
            return Err(ParsePaddedNumberError::TooLong(max, str_len as u8));
        }
    }

    let leading_zeros = konst::iter::eval!(konst::string::chars(str), take_while(|char| *char == '0'), count()) as u8;

    let remaining_number = konst::try_!(konst::result::map_err!(
        u64::from_str_radix(str, 10),
        ParsePaddedNumberError::InvalidNumber
    ));

    Ok((leading_zeros, remaining_number))
}
