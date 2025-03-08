// TEMP: until first publish
#![allow(missing_docs)]

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

    match padded_number_internal::parse::parse(min, max, &number_str) {
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
