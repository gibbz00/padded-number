use ::phf::PhfHash;

use crate::*;

impl<const A: u8, const B: u8> PhfHash for PaddedNumber<A, B> {
    fn phf_hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let Self { leading_zeros, number } = *self;
        state.write_u8(leading_zeros);
        state.write_u64(number);
    }
}

#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hasher};

    use super::*;

    #[test]
    fn hash_number() {
        let a_hash = padded_number_hash("123");
        let b_hash = padded_number_hash("321");
        assert_ne!(a_hash, b_hash);
    }

    #[test]
    fn hash_zeros() {
        let a_hash = padded_number_hash("0");
        let b_hash = padded_number_hash("000");
        assert_ne!(a_hash, b_hash);
    }

    #[test]
    fn hash_combined() {
        let a_hash = padded_number_hash("00123");
        let b_hash = padded_number_hash("0321");
        assert_ne!(a_hash, b_hash);

        let c_hash = padded_number_hash("00123");
        assert_eq!(a_hash, c_hash)
    }

    fn padded_number_hash(str: &str) -> u64 {
        let padded_number = PaddedNumber::<1, 10>::try_new(str).unwrap();

        // overkill to pull in FNV just for testing
        let mut hasher = DefaultHasher::default();

        padded_number.phf_hash(&mut hasher);

        hasher.finish()
    }
}
