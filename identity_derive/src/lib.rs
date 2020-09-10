use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::{model::InputModel, utils::should_ignore};

mod impls;
mod model;
mod utils;

#[proc_macro_derive(Diff, attributes(diff))]
pub fn derive_diff(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    internal(input)
}

fn internal(input: DeriveInput) -> TokenStream {
    let model: InputModel = InputModel::parse(&input);
    let debug = model.impl_debug();
    let diff = model.impl_diff();
    let diff_typ = model.derive_diff();

    let output = quote! {
        #diff_typ
        #debug
        #diff
    };
    // for debugging.
    // println!("{}", diff_typ);
    // println!("{}", debug);
    // println!("{}", diff);
    TokenStream::from(output)
}
