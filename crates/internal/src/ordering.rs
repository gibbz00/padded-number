use std::cmp::Ordering;

use crate::*;

impl<const A: u8, const B: u8> Ord for PaddedNumber<A, B> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.len().cmp(&other.len()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.number.cmp(&other.number),
        }
    }
}

impl<const A: u8, const B: u8> PartialOrd for PaddedNumber<A, B> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::mock_from_str;

    #[test]
    fn ordering() {
        // Longer numbers of greater length are always greater than those with fewer
        assert_ordering("", "0");
        assert_ordering("0", "00");
        assert_ordering("10", "001");
        assert_ordering("9", "00");

        assert_ordering("46720", "46730");

        // matching length compares remaining number
        assert_ordering("0012", "0120");

        fn assert_ordering(number_str_0: &str, number_str_1: &str) {
            let number_0 = mock_from_str::<0, 10>(number_str_0);
            let number_1 = mock_from_str::<0, 10>(number_str_1);
            assert!(number_0 < number_1);
            assert!(number_1 > number_0);
        }
    }
}
