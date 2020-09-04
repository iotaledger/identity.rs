#![allow(unused)]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parenthesized, parse,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Attribute, Data, DeriveInput, Error, Field, Fields, Ident,
};

mod model;

#[proc_macro_derive(Diff, attributes(diff))]
pub fn derive_diff(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = interal(input);

    TokenStream::from(output)
}

fn interal(input: DeriveInput) -> TokenStream {
    todo!()
}
