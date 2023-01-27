// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use proc_macro2::Delimiter;
use proc_macro2::TokenTree;
use quote::format_ident;
use syn::DeriveInput;
use syn::Field;
use syn::Meta;
use syn::MetaList;
use syn::NestedMeta;
use syn::Path;
use syn::PathSegment;

const PARENS: Delimiter = Delimiter::Parenthesis;

/// checks to see if a field's type is `Option`.  This logic is necessary to find cases where fields contain nested
/// Options and avoid a `Some(None)` case.
pub fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
  let idents_of_path = path.segments.iter().fold(String::new(), |mut acc, v| {
    acc.push_str(&v.ident.to_string());
    acc.push('|');
    acc
  });
  vec!["Option|", "std|option|Option|", "core|option|Option|"]
    .into_iter()
    .find(|s| idents_of_path == *s)
    .and_then(|_| path.segments.last())
}

/// checks to see if the `should_ignore` attribute has been put before a field.
pub fn should_ignore(field: &Field) -> bool {
  let find = field.attrs.iter().find(|field| {
    let attr_seg: Vec<String> = field.path.segments.iter().map(|seg| format!("{}", seg.ident)).collect();

    let diff_attr = attr_seg == ["diff"];
    let arg_iter = field.tokens.clone().into_iter().next();

    let should_ignore = match arg_iter {
      Some(TokenTree::Group(gr)) if gr.delimiter() == PARENS => gr
        .stream()
        .into_iter()
        .map(|tt| format!("{tt}"))
        .any(|x| x == "should_ignore"),
      _ => false,
    };

    diff_attr && should_ignore
  });

  find.is_some()
}

pub fn parse_from_into(input: &DeriveInput) -> bool {
  let find = input.attrs.iter().find(|a| {
    if let Meta::List(MetaList { path, nested, .. }) = a.parse_meta().unwrap() {
      {
        if let Some(ident) = path.get_ident() {
          if "diff" == format!("{}", format_ident!("{}", ident)) {
            let find_nested = nested.iter().find(|m| {
              if let NestedMeta::Meta(Meta::Path(p)) = m {
                if let Some(ident) = p.get_ident() {
                  if "from_into" == format!("{}", format_ident!("{}", ident)) {
                    return true;
                  }
                }
              }
              false
            });

            return find_nested.is_some();
          }
        }
      }
    }
    false
  });

  find.is_some()
}
