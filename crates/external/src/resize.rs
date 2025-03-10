use crate::*;

/// Resize a `PaddedNumber` into a smaller one or one of the same size.
///
/// Type system ensures that the new minimum length >= previous minimum length,
/// and that the new maximum length <= previous maximum length.
///
/// ```rust
/// #![feature(generic_const_exprs)]
///
/// # use padded_number_macros::*;
/// # use crate::padded_number::ResizePaddedNumber;
/// let a = bound_padded_number!(2, 3, "123");
/// let b = bound_padded_number!(1, 5, "123").resize();
/// assert_eq!(a, b)
/// ```
///
/// It being marked at trait makes it possible to define functions which allow
/// the passing of any padded number which fits into the desired size.
///
/// ```rust
/// # #![feature(generic_const_exprs)]
/// # use padded_number_macros::*;
/// # use crate::padded_number::ResizePaddedNumber;
/// let a = bound_padded_number!(1, 5, "123");
/// foo_dyn(&a);
/// foo_impl(a);
///
/// let b = bound_padded_number!(2, 4, "123");
/// foo_dyn(&b);
/// foo_impl(b);
///
/// fn foo_dyn(padded_number: &dyn ResizePaddedNumber<2, 3>) {
///     let _padded_number = padded_number.resize();
/// }
///
/// fn foo_impl(padded_number: impl ResizePaddedNumber<2, 3>) {
///     let _padded_number = padded_number.resize();
/// }
/// ```
#[allow(private_bounds)]
pub trait ResizePaddedNumber<const A_1: u8, const B_1: u8>: private::SealedResize {
    /// Resize a padded number
    ///
    /// Check out the trait-level documentation for more
    fn resize(&self) -> PaddedNumber<A_1, B_1>;
}

impl<const A_0: u8, const B_0: u8, const A_1: u8, const B_1: u8> ResizePaddedNumber<A_1, B_1> for PaddedNumber<A_0, B_0>
where
    [(); (A_1 - A_0) as usize]:,
    [(); (B_0 - B_1) as usize]:,
{
    fn resize(&self) -> PaddedNumber<A_1, B_1> {
        let PaddedNumber { leading_zeros, number } = *self;
        PaddedNumber { leading_zeros, number }
    }
}

mod private {
    use super::*;

    pub(super) trait SealedResize {}

    impl<const A: u8, const B: u8> SealedResize for PaddedNumber<A, B> {}
}
