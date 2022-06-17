// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![doc = include_str!("./../README.md")]

use crate::model::InputModel;
use crate::utils::extract_option_segment;
use crate::utils::parse_from_into;
use crate::utils::should_ignore;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

mod impls;
mod model;
mod utils;

/// Entry point for the `Diff` derive proc macro.  `Diff` implements the `Diff` trait from the `identity_diff` crate on
/// any Enum or Struct type.  Contains and optional attribute `should_ignore` which will ignore an appended field.
#[proc_macro_derive(Diff, attributes(diff))]
pub fn derive_diff(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  internal(input)
}

/// Function for dealing with the internal logic of the macro.
fn internal(input: DeriveInput) -> TokenStream {
  let model: InputModel = InputModel::parse(&input);
  // debug implementation derivation.
  let debug = model.impl_debug();
  // diff type derivation.
  let diff = model.impl_diff();
  // diff trait implementation derivation.
  let diff_typ = model.derive_diff();

  let from_into = model.impl_from_into();

  let output = quote! {
      #diff_typ
      #debug
      #diff
      #from_into
  };

  // for debugging.
  // println!("{}", from_into);
  // println!("{}", diff_typ);
  // println!("{}", debug);
  // println!("{}", diff);

  // A hack.
  TokenStream::from(output)
}
