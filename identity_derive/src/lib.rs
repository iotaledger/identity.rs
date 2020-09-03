use proc_macro::TokenStream;
// use quote::{format_ident, quote};
// use syn::{
//     parenthesized, parse,
//     parse::{Parse, ParseStream},
//     parse_macro_input,
//     punctuated::Punctuated,
//     token::Comma,
//     Attribute, Data, DeriveInput, Error, Field, Fields, Ident,
// };

#[proc_macro_derive(Diff, attributes(diff))]
pub fn derive_diff(_input: TokenStream) -> TokenStream {
    unimplemented!();
}
