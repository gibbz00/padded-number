use super::*;

impl<const A: u8, const B: u8> std::fmt::Debug for PaddedNumber<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        let mut string = ['0'].repeat(self.leading_zeros as usize).iter().collect::<String>();

        if self.number != 0 {
            string.push_str(&self.number.to_string());
        }

        f.write_str(&string)
    }
}

impl<const A: u8, const B: u8> std::fmt::Display for PaddedNumber<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::mock_from_str;

    #[test]
    fn debug_print() {
        let expected_dbg_str = "0123";
        let number = mock_from_str::<1, 4>(expected_dbg_str);
        let actual_dbg_str = format!("{number:?}");

        assert_eq!(expected_dbg_str, &actual_dbg_str);
    }

    #[test]
    fn display_print() {
        let expected_display_str = "0012";
        let number = mock_from_str::<1, 4>(expected_display_str);
        let actual_display_str = format!("{number}");

        assert_eq!(expected_display_str, &actual_display_str);
    }

    #[test]
    fn display_empty() {
        let expected_display_str = "";
        let number = mock_from_str::<0, 0>("");
        let actual_display_str = format!("{number}");

        assert_eq!(expected_display_str, &actual_display_str);
    }

    #[test]
    fn display_leading_zeros_only() {
        let expected_display_str = "00";
        let number = mock_from_str::<2, 2>("00");
        let actual_display_str = format!("{number}");

        assert_eq!(expected_display_str, &actual_display_str);
    }
}
