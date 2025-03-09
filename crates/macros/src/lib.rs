//! # `padded-number-macros` - Macros for compile time `padded-number` constructs

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    LitInt, LitStr,
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Comma,
};

/// Construct a `PaddedNumber` at compile time
///
/// May be seend as shorthand for writing `PaddedNumber::<1, {
/// u8::MAX}>::try_new("001").unwrap()`, but compile time error reporting.
///
/// Errors if provided string is not within the provided length bounds, or it it
/// constrains anything but ASCII digits.
///
/// Works in const context:
///
/// ```no_compile
/// const PADDED_NUMBER: PaddedNumber = padded_number!("001");
/// ```
#[proc_macro]
pub fn padded_number(token_stream: TokenStream) -> TokenStream {
    let number_literal = parse_macro_input!(token_stream as LitStr);

    let args = Args { min: 1, max: u8::MAX, number_literal };

    padded_number_impl(args).into()
}

/// Construct a bound `PaddedNumber` at compile time, similar to
/// `padded_number!`
///
/// First and second parameter denote min and max length bounds respectively,
/// both inclusive.
///
/// Works in const context:
///
/// ```no_compile
/// const PADDED_NUMBER: PaddedNumber<1, 3> = bound_padded_number!(1, 3, "001");
/// ```
#[proc_macro]
pub fn bound_padded_number(token_stream: TokenStream) -> TokenStream {
    let args = parse_macro_input!(token_stream as Args);
    padded_number_impl(args).into()
}

struct Args {
    min: u8,
    max: u8,
    number_literal: LitStr,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let min = input.parse::<LitInt>()?.base10_parse()?;
        let _comma = input.parse::<Comma>()?;
        let max = input.parse::<LitInt>()?.base10_parse()?;
        let _comma = input.parse::<Comma>()?;
        let number_string = input.parse::<LitStr>()?;

        Ok(Args { min, max, number_literal: number_string })
    }
}

fn padded_number_impl(args: Args) -> TokenStream2 {
    let Args { min, max, number_literal } = args;
    let number_str = number_literal.value();

    match padded_number_internal::parse(min, max, &number_str) {
        Ok((leading_zeros, remaining_number)) => {
            quote! {
                // SAFETY: invariants verified by proc macro
                unsafe {
                    padded_number::PaddedNumber::<#min, #max>::new_unchecked(
                        #leading_zeros,
                        #remaining_number
                    )
                }
            }
        }
        Err(error) => syn::Error::new(number_literal.span(), error.to_string()).into_compile_error(),
    }
}
