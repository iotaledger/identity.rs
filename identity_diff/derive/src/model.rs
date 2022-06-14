// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::extract_option_segment;
use crate::impls::debug_impl;
use crate::impls::derive_diff_enum;
use crate::impls::derive_diff_struct;
use crate::impls::diff_impl;
use crate::impls::impl_debug_enum;
use crate::impls::impl_diff_enum;
use crate::impls::impl_from_into;
use crate::parse_from_into;
use crate::should_ignore;
use proc_macro2::Ident;
use proc_macro2::Literal;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Data;
use syn::DataEnum;
use syn::DataStruct;
use syn::DeriveInput;
use syn::Fields;
use syn::GenericParam;
use syn::Token;
use syn::Type;
use syn::Variant;
use syn::WhereClause;

/// A model for dealing with the different input from the incoming AST.
#[derive(Clone, Debug)]
pub enum InputModel {
  Enum(InputEnum),
  Struct(InputStruct),
}

/// Sorts attributes regarding incoming Enums.
#[derive(Clone, Debug)]
pub struct InputEnum {
  // name identifier.
  pub name: Ident,
  // diff identifier.
  pub diff: Ident,
  // variants for the Enum.
  pub variants: Vec<EVariant>,
  // generics and traits declarations.
  pub param_decls: Punctuated<GenericParam, Comma>,
  // generics and trait bounds for the Enum.
  pub params: Punctuated<Ident, Comma>,
  // where clause for the Enum.
  pub clause: WhereClause,
  // should this enum be serialized/deserialized as its non-diff counterpart.
  pub from_into: bool,
}

/// Sorts data regarding incoming Structs.
#[derive(Clone, Debug)]
pub struct InputStruct {
  // struct variant.
  pub variant: SVariant,
  // struct name.
  pub name: Ident,
  // struct diff name.
  pub diff: Ident,
  // struct fields.
  pub fields: Vec<DataFields>,
  // generics and traits declarations.
  pub param_decls: Punctuated<GenericParam, Comma>,
  // generics and trait bounds for the Enum.
  pub params: Punctuated<Ident, Comma>,
  // where clause for the Enum.
  pub clause: WhereClause,
  // should this struct be serialized/deserialized as its non-diff counterpart/
  pub from_into: bool,
}

/// Enum variant data.
#[derive(Clone, Debug)]
pub struct EVariant {
  // Struct variant type.
  pub variant: SVariant,
  // variant name.
  pub name: Ident,
  // variant fields.
  pub fields: Vec<DataFields>,
}

/// Struct Variant structure types.
#[derive(Clone, Debug)]
pub enum SVariant {
  Named,
  Tuple,
  Unit,
}

/// sorts data for fields inside of a struct or enum.
#[derive(Clone, Debug)]
pub enum DataFields {
  Named {
    // field name.
    name: Ident,
    // field type.
    typ: Type,
    // should ignore flag.
    should_ignore: bool,
  },
  Unnamed {
    // field position.
    position: Literal,
    // field type.
    typ: Type,
    // should ignore flag.
    should_ignore: bool,
  },
}

impl InputModel {
  // parse the `DeriveInput` into an `InputModel`.
  pub fn parse(input: &DeriveInput) -> Self {
    match &input.data {
      // Check for a struct with fields.
      Data::Struct(DataStruct { fields, .. }) if !fields.is_empty() => Self::parse_struct(input, fields),
      // check for a unit struct.
      Data::Struct(DataStruct { .. }) => Self::parse_unit(input),
      // check for an enum.
      Data::Enum(DataEnum { variants, .. }) => Self::parse_enum(input, variants),
      _ => panic!("Data Type not supported"),
    }
  }

  /// parse structs.
  fn parse_struct(input: &DeriveInput, fields: &Fields) -> Self {
    Self::Struct(InputStruct::parse(input, fields))
  }

  /// parse unit structs.
  fn parse_unit(input: &DeriveInput) -> Self {
    Self::Struct(InputStruct::parse_unit(input))
  }

  /// parse enums.
  fn parse_enum(input: &DeriveInput, variants: &Punctuated<Variant, Comma>) -> Self {
    Self::Enum(InputEnum::parse(input, variants))
  }

  /// get struct variant.
  pub fn s_variant(&self) -> &SVariant {
    match self {
      Self::Enum(InputEnum { name, .. }) => panic!("{} isn't a struct", name),
      Self::Struct(InputStruct { variant, .. }) => variant,
    }
  }

  /// get enum variant.
  pub fn e_variants(&self) -> &Vec<EVariant> {
    match self {
      Self::Enum(InputEnum { variants, .. }) => variants,
      Self::Struct(InputStruct { name, .. }) => panic!("{} isn't an Enum", name),
    }
  }

  /// get name of struct or enum.
  pub fn name(&self) -> &Ident {
    match self {
      Self::Enum(InputEnum { name, .. }) => name,
      Self::Struct(InputStruct { name, .. }) => name,
    }
  }

  /// get diff name for enum or struct.
  pub fn diff(&self) -> &Ident {
    match self {
      Self::Enum(InputEnum { diff, .. }) => diff,
      Self::Struct(InputStruct { diff, .. }) => diff,
    }
  }

  /// get the params for the Enum or Struct.
  pub fn params(&self) -> &Punctuated<Ident, Comma> {
    match self {
      Self::Enum(InputEnum { params, .. }) => params,
      Self::Struct(InputStruct { params, .. }) => params,
    }
  }

  /// get the param declarations for the Enum or Struct.
  pub fn param_decls(&self) -> &Punctuated<GenericParam, Comma> {
    match self {
      Self::Enum(InputEnum { param_decls, .. }) => param_decls,
      Self::Struct(InputStruct { param_decls, .. }) => param_decls,
    }
  }

  /// get the fields for the Enum or Struct.
  pub fn fields(&self) -> &Vec<DataFields> {
    match self {
      Self::Enum(InputEnum { name, .. }) => panic!("{} isn't a Struct", name),
      Self::Struct(InputStruct { fields, .. }) => fields,
    }
  }

  /// Get the where clause.
  pub fn clause(&self) -> &WhereClause {
    match self {
      Self::Enum(InputEnum { clause, .. }) => clause,
      Self::Struct(InputStruct { clause, .. }) => clause,
    }
  }

  /// Implement the `Debug` trait on Enums and Structs.
  pub fn impl_debug(&self) -> TokenStream {
    match self {
      Self::Struct(InputStruct { .. }) => debug_impl(self),
      Self::Enum(InputEnum { .. }) => impl_debug_enum(self),
    }
  }

  pub fn impl_from_into(&self) -> TokenStream {
    match self {
      Self::Struct(InputStruct { .. }) => impl_from_into(self),
      Self::Enum(InputEnum { .. }) => impl_from_into(self),
    }
  }

  /// Implement the `Diff` trait on Enums and Structs.
  pub fn impl_diff(&self) -> TokenStream {
    match self {
      Self::Struct(InputStruct { .. }) => diff_impl(self),
      Self::Enum(InputEnum { .. }) => impl_diff_enum(self),
    }
  }

  /// Build the Diff Type for the Enum or Struct.
  pub fn derive_diff(&self) -> TokenStream {
    match self {
      Self::Struct(InputStruct { .. }) => derive_diff_struct(self),
      Self::Enum(InputEnum { .. }) => derive_diff_enum(self),
    }
  }

  #[allow(clippy::wrong_self_convention)]
  pub fn from_into(&self) -> bool {
    match self {
      Self::Struct(InputStruct { from_into, .. }) => *from_into,
      Self::Enum(InputEnum { from_into, .. }) => *from_into,
    }
  }
}

impl InputEnum {
  /// create a new `InputEnum`.
  pub fn new(input: &DeriveInput) -> Self {
    let from_into = parse_from_into(input);
    Self {
      name: input.ident.clone(),
      diff: format_ident!("Diff{}", &input.ident),
      variants: Vec::new(),
      param_decls: input.generics.params.clone(),
      params: input
        .generics
        .type_params()
        .map(|type_param| type_param.ident.clone())
        .collect(),
      clause: input.generics.where_clause.clone().unwrap_or_else(|| WhereClause {
        where_token: Token![where](Span::call_site()),
        predicates: Punctuated::new(),
      }),
      from_into,
    }
  }

  /// parse the enum.
  fn parse(input: &DeriveInput, variants: &Punctuated<Variant, Comma>) -> Self {
    let mut model = Self::new(input);
    variants.iter().for_each(|vars| {
      let mut variant = EVariant::new(&vars.ident);

      vars.fields.iter().enumerate().for_each(|(idx, fs)| {
        if let Some(ident) = fs.ident.as_ref() {
          variant.variant = SVariant::Named;
          variant.fields.push(DataFields::Named {
            name: ident.clone(),
            typ: fs.ty.clone(),
            should_ignore: should_ignore(fs),
          });
        } else {
          variant.variant = SVariant::Tuple;
          variant.fields.push(DataFields::Unnamed {
            position: Literal::usize_unsuffixed(idx),
            typ: fs.ty.clone(),
            should_ignore: should_ignore(fs),
          });
        }
      });
      model.variants.push(variant);
    });

    model
  }
}

impl InputStruct {
  /// create a new `InputStruct`.
  pub fn new(input: &DeriveInput) -> Self {
    let from_into = parse_from_into(input);

    Self {
      variant: SVariant::Unit,
      name: input.ident.clone(),
      diff: format_ident!("Diff{}", &input.ident),
      fields: Vec::new(),
      param_decls: input.generics.params.clone(),
      params: input.generics.type_params().map(|tp| tp.ident.clone()).collect(),
      clause: input.generics.where_clause.clone().unwrap_or_else(|| WhereClause {
        where_token: Token![where](Span::call_site()),
        predicates: Punctuated::new(),
      }),
      from_into,
    }
  }

  /// parse the ast into for the `InputStruct`.
  fn parse(input: &DeriveInput, fields: &Fields) -> Self {
    let mut model = Self::new(input);
    fields.iter().enumerate().for_each(|(idx, fs)| {
      if let Some(ident) = fs.ident.as_ref() {
        model.variant = SVariant::Named;
        model.fields.push(DataFields::Named {
          name: ident.clone(),
          typ: fs.ty.clone(),
          should_ignore: should_ignore(fs),
        });
      } else {
        model.variant = SVariant::Tuple;
        model.fields.push(DataFields::Unnamed {
          position: Literal::usize_unsuffixed(idx),
          typ: fs.ty.clone(),
          should_ignore: should_ignore(fs),
        });
      }
    });

    model
  }

  /// parse data for a unit struct.
  fn parse_unit(input: &DeriveInput) -> Self {
    let mut model = Self::new(input);
    model.variant = SVariant::Unit;

    model
  }
}

impl EVariant {
  /// create a new enum variant type.
  pub fn new(name: &Ident) -> Self {
    Self {
      variant: SVariant::Unit,
      name: name.clone(),
      fields: Vec::new(),
    }
  }
}

impl DataFields {
  /// get the field name.
  pub fn name(&self) -> &Ident {
    match self {
      Self::Named { name, .. } => name,
      Self::Unnamed { .. } => panic!("Positional Field has no name"),
    }
  }

  /// get the field position.
  pub fn position(&self) -> &Literal {
    match self {
      Self::Named { .. } => panic!("Named fields has no position"),
      Self::Unnamed { position, .. } => position,
    }
  }

  /// get the field type.
  pub fn typ(&self) -> &Type {
    match self {
      Self::Named { typ, .. } => typ,
      Self::Unnamed { typ, .. } => typ,
    }
  }

  /// get the type of the field wrapped in an `Option<T>` where T = the field type for an ignored field and T is an
  /// `identity_diff::Diff::Type` for a non-ignored field.
  pub fn typ_as_tokens(&self) -> TokenStream {
    let typ = self.typ();

    if self.should_ignore() {
      quote! {Option<#typ>}
    } else {
      quote! { Option<<#typ as identity_diff::Diff>::Type> }
    }
  }

  /// check if the field is an Option to avoid nested Options.
  pub fn is_option(&self) -> bool {
    let typ = self.typ();

    let opt = match typ {
      syn::Type::Path(typepath) if typepath.qself.is_none() => Some(typepath.path.clone()),
      _ => None,
    };

    if let Some(o) = opt {
      extract_option_segment(&o).is_some()
    } else {
      false
    }
  }

  /// check to see if the should ignore flag is set for the field.
  pub fn should_ignore(&self) -> bool {
    match self {
      Self::Named { should_ignore, .. } => *should_ignore,
      Self::Unnamed { should_ignore, .. } => *should_ignore,
    }
  }
}
