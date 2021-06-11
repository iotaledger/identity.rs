// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(non_snake_case)]

use crate::model::InputModel;
use crate::model::SVariant;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::GenericParam;

/// Derive the difference struct code from the `InputModel`
pub fn derive_diff_struct(input: &InputModel) -> TokenStream {
  // setup the relevant fields.
  let svariant = input.s_variant();
  let diff = input.diff();
  let fields = input.fields();
  let param_decls = input.param_decls();
  let clause = input.clause();
  let serde_attrs = if input.from_into() {
    let name = input.name();
    let stype = quote!(#name).to_string();

    quote! {
        #[serde(from=#stype, into=#stype)]
    }
  } else {
    quote! {}
  };

  // set the param declarations.
  let param_decls: Vec<TokenStream> = param_decls
    .iter()
    .map(|tp_decl| match tp_decl {
      GenericParam::Lifetime(life) => quote! { #life  },
      GenericParam::Const(consts) => quote! { #consts},
      GenericParam::Type(typ) => {
        let S: &Ident = &typ.ident;

        let bounds: Vec<TokenStream> = typ
          .bounds
          .iter()
          .map(|bound| {
            quote! {
                #bound
            }
          })
          .collect();

        quote! {
            #S: identity_diff::Diff
            #(+ #bounds)*
        }
      }
    })
    .collect();

  // get the field types as tokens.
  let field_tps: Vec<TokenStream> = fields.iter().map(|field| field.typ_as_tokens()).collect();

  match svariant {
    // for name structs.
    SVariant::Named => {
      // get te field names
      let field_names: Vec<&Ident> = fields.iter().map(|field| field.name()).collect();

      // generate the Diff struct.
      quote! {
          #[derive(Clone, PartialEq, Default)]
          #[derive(serde::Deserialize, serde::Serialize)]
          #serde_attrs
          pub struct #diff<#(#param_decls),*>
              #clause
          {
              #( #[doc(hidden)] #[serde(skip_serializing_if = "Option::is_none")] pub(self) #field_names: #field_tps, )*
          }
      }
    }
    // for Tuple variant Structs.
    SVariant::Tuple => {
      // generate the Diff struct.
      quote! {
          #[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize, Default)]
          #serde_attrs
          pub struct #diff<#(#param_decls),*> (
              #( #[doc(hidden)] #[serde(skip_serializing_if = "Option::is_none")] pub(self) #field_tps, )*
          ) #clause ;
      }
    }
    // for unit variant Structs.
    SVariant::Unit => quote! {
        // generate the Diff struct.
        #[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize, Default)]
        #serde_attrs
        pub struct #diff<#(#param_decls),*> #clause ;
    },
  }
}

/// Implement the Debug trait on a derived struct.
pub fn debug_impl(input: &InputModel) -> TokenStream {
  // collect relevant fields.
  let svariant = input.s_variant();
  let diff = input.diff();
  let fields = input.fields();
  let param_decls = input.param_decls();
  let params = input.params();
  let clause = input.clause();

  // setup param declarations.
  let param_decls: Vec<TokenStream> = param_decls
    .iter()
    .map(|tp_decl| match tp_decl {
      GenericParam::Lifetime(life) => quote! { #life  },
      GenericParam::Const(consts) => quote! { #consts},
      GenericParam::Type(typ) => {
        let S: &Ident = &typ.ident;

        let bounds: Vec<TokenStream> = typ
          .bounds
          .iter()
          .map(|bound| {
            quote! {
                #bound
            }
          })
          .collect();

        quote! {
            #S: identity_diff::Diff + std::fmt::Debug
            #(+ #bounds)*
        }
      }
    })
    .collect();

  // setup the where clause predicates and the where clause code.
  let preds: Vec<TokenStream> = clause.predicates.iter().map(|pred| quote! { #pred }).collect();
  let clause = quote! { where #(#preds),*};

  match svariant {
    // name struct
    SVariant::Named => {
      // create a buffer and generate the logic.
      let mut mac = TokenStream::new();
      let buf: Ident = format_ident!("buf");
      for field in fields.iter() {
        let (fname, ftype) = (field.name(), field.typ());

        let str_name = format!("{}", fname);

        mac.extend(if field.should_ignore() {
          quote! {
              #buf.field(#str_name, &self.#fname);
          }
        } else {
          quote! {
              if let Some(val) = &self.#fname {
                  #buf.field(#str_name, val);
              } else {
                  #buf.field(#str_name, &None as &Option<#ftype>);
              }
          }
        });
      }

      // generate code.
      quote! {
          impl<#(#param_decls),*> std::fmt::Debug
              for #diff<#params>
              #clause
              {
                  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                      const NAME: &str = stringify!(diff);
                      let mut #buf = f.debug_struct(NAME);
                      #mac
                      #buf.finish()
                  }
              }
      }
    }
    // Tuple Struct.
    SVariant::Tuple => {
      let count = fields.len();

      let mut f_tokens = TokenStream::new();
      let buf = format_ident!("buf");
      for field in fields.iter() {
        let (fpos, ftyp) = (field.position(), field.typ());

        f_tokens.extend(match count {
          1 => quote! {},
          _ if field.should_ignore() => quote! {
              #buf.field(&self.#fpos);
          },
          _ => quote! {
              if let Some(val) = &self.#fpos {
                  #buf.field(val);
              } else {
                  #buf.field(&None as &Option<#ftyp>);
              }
          },
        });
      }
      let mac = match count {
        1 => quote! {
            const NAME: &str = stringify!(#diff);
            if let Some(val) = &self.0 {
                write!(f, "{}({:?})", NAME, val)
            } else {
                let field = &None as &Option<()>;
                write!(f, "{}({:?})", NAME, field)
            }
        },
        _ => quote! {
            const NAME: &str = stringify!(#diff);
            let mut #buf = f.debug_tuple(NAME);
            #f_tokens
            #buf.finish()
        },
      };

      // generate code.
      quote! {
          impl<#(#param_decls),*> std::fmt::Debug for #diff<#params>
              #clause
              {
                  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                      #mac
                  }
              }
      }
    }
    // generate code for unit struct.
    SVariant::Unit => quote! {
            impl<#(#param_decls),*> std::fmt::Debug for #diff<#params>
                #clause
            {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
                {
                    f.debug_struct(stringify!(#diff)).finish()
                }
            }
    },
  }
}

pub fn impl_from_into(input: &InputModel) -> TokenStream {
  if input.from_into() {
    let diff = input.diff();
    let param_decls = input.param_decls();
    let params = input.params();
    let clause = input.clause();
    let name = input.name();
    let param_decls: Vec<TokenStream> = param_decls
      .iter()
      .map(|tp_decl| match tp_decl {
        GenericParam::Lifetime(life) => quote! { #life  },
        GenericParam::Const(consts) => quote! { #consts},
        GenericParam::Type(typ) => {
          let S: &Ident = &typ.ident;

          let bounds: Vec<TokenStream> = typ
            .bounds
            .iter()
            .map(|bound| {
              quote! {
                  #bound
              }
            })
            .collect();

          quote! {
              #S: std::clone::Clone
              + std::default::Default
              + identity_diff::Diff
              + for<'de> serde::Deserialize<'de>
              + serde::Serialize
              #(+ #bounds)*
          }
        }
      })
      .collect();

    let preds: Vec<TokenStream> = clause.predicates.iter().map(|pred| quote! { #pred }).collect();
    let clause = quote! { where #(#preds),*};

    quote! {
        impl <#(#param_decls),*> std::convert::From<#name<#params>> for #diff<#params> #clause
        {
            fn from(name: #name<#params>) -> Self {
                name.into_diff().expect("Unable to convert to diff")
            }
        }

        impl <#(#param_decls),*> std::convert::From<#diff<#params>> for #name<#params> #clause
        {
            fn from(diff: #diff<#params>) -> Self {
                Self::from_diff(diff).expect("Unable to convert from diff")
            }
        }
    }
  } else {
    quote! {}
  }
}

/// implement Diff for the struct.
pub fn diff_impl(input: &InputModel) -> TokenStream {
  // collect relevant fields and generate param declarations.
  let svariant = input.s_variant();
  let name = input.name();
  let diff = input.diff();
  let fields = input.fields();
  let param_decls = input.param_decls();
  let params = input.params();
  let clause = input.clause();
  let param_decls: Vec<TokenStream> = param_decls
    .iter()
    .map(|tp_decl| match tp_decl {
      GenericParam::Lifetime(life) => quote! { #life  },
      GenericParam::Const(consts) => quote! { #consts},
      GenericParam::Type(typ) => {
        let S: &Ident = &typ.ident;

        let bounds: Vec<TokenStream> = typ
          .bounds
          .iter()
          .map(|bound| {
            quote! {
                #bound
            }
          })
          .collect();

        quote! {
            #S: std::clone::Clone
            + std::default::Default
            + identity_diff::Diff
            + std::fmt::Debug
            + std::cmp::PartialEq
            + for<'de> serde::Deserialize<'de>
            + serde::Serialize
            #(+ #bounds)*
        }
      }
    })
    .collect();

  // get predicates and generate where clause.
  let preds: Vec<TokenStream> = clause.predicates.iter().map(|pred| quote! { #pred }).collect();
  let clause = quote! { where #(#preds),* };

  match svariant {
    // named struct.
    SVariant::Named => {
      // get field names.
      let fnames: Vec<&Ident> = fields.iter().map(|field| field.name()).collect();
      // get merge field logic.
      let field_merge: Vec<TokenStream> = fields
        .iter()
        .map(|field| {
          let fname = field.name();
          if field.should_ignore() {
            quote! {
                #fname: self.#fname.clone(),
            }
          } else {
            quote! {
                #fname: if let Some(d) = diff.#fname {
                    self.#fname.merge(d)?
                } else {
                    self.#fname.clone()
                },
            }
          }
        })
        .collect();

      // get diff field logic.
      let fields_diff: Vec<TokenStream> = fields
        .iter()
        .map(|field| {
          let fname = field.name();
          if field.should_ignore() {
            quote! {
                #fname: Option::None
            }
          } else if field.is_option() {
            quote! {
                #fname: if self.#fname == other.#fname || other.#fname == None {
                    None
                } else {
                    Some(self.#fname.diff(&other.#fname)?)
                }
            }
          } else {
            quote! {
                #fname: if self.#fname == other.#fname {
                    None
                } else {
                    Some(self.#fname.diff(&other.#fname)?)
                }
            }
          }
        })
        .collect();

      // get from_diff field logic.
      let fields_from: Vec<TokenStream> = fields
        .iter()
        .map(|field| {
          let fname = field.name();
          let ftyp = field.typ();
          if field.should_ignore() {
            quote! { #fname: Default::default() }
          } else {
            quote! {
                #fname: <#ftyp>::from_diff(
                    match #fname {
                        Some(v) => v,
                        None => <#ftyp>::default().into_diff()?
                    },
                )?
            }
          }
        })
        .collect();

      // get into_diff field logic.
      let fields_into: Vec<TokenStream> = fields
        .iter()
        .map(|field| {
          let fname = field.name();
          if field.should_ignore() {
            quote! { #fname: Option::None }
          } else if field.is_option() {
            quote! {
                #fname: if let identity_diff::DiffOption::Some(_) = #fname.clone().into_diff()? {
                    Some(#fname.into_diff()?)
                } else {
                    None
                }
            }
          } else {
            quote! {
                #fname: Some(#fname.into_diff()?)
            }
          }
        })
        .collect();

      // generate body and code.
      quote! {
          impl<#(#param_decls),*> identity_diff::Diff
              for #name<#params>
              #clause
          {
              type Type = #diff<#params>;

              #[allow(unused)]
              fn merge(&self, diff: Self::Type) -> identity_diff::Result<Self> {
                  Ok(Self{ #(#field_merge)* })
              }

              fn diff(&self, other: &Self) -> identity_diff::Result<Self::Type> {
                  Ok(#diff { #(#fields_diff),* })
              }

              #[allow(unused)]
              fn from_diff(diff: Self::Type) -> identity_diff::Result<Self> {
                  match diff {
                      #diff { #(#fnames),* } => {
                          Ok(Self{ #(#fields_from),* })
                      }
                  }
              }

              #[allow(unused)]
              fn into_diff(self) -> identity_diff::Result<Self::Type> {
                  match self {
                      Self { #(#fnames),* } => {
                          Ok(#diff { #(#fields_into),* })
                      }
                  }
              }
          }

      }
    }
    // Tuple struct.
    SVariant::Tuple => {
      // get types and create markers for the positioned fields.
      let field_markers: Vec<Ident> = (0..fields.len()).map(|t| format_ident!("field_{}", t)).collect();

      // get merge field logic.
      let field_merge: Vec<TokenStream> = fields
        .iter()
        .map(|field| {
          let pos = field.position();
          if field.should_ignore() {
            quote! {
                self.#pos.clone(),
            }
          } else {
            quote! {
                if let Some(v) = diff.#pos {
                    self.#pos.merge(v)?
                } else {
                    self.#pos.clone()
                },
            }
          }
        })
        .collect();

      // get diff field logic.
      let fields_diff: Vec<TokenStream> = fields
        .iter()
        .map(|field| {
          let pos = field.position();
          if field.should_ignore() {
            quote! {
                Option::None
            }
          } else if field.is_option() {
            quote! {
                if self.#pos != other.#pos && other.#pos != None {
                    Some(self.#pos.diff(&other.#pos)?)
                } else {
                    None
                },
            }
          } else {
            quote! {
                if self.#pos != other.#pos {
                    Some(self.#pos.diff(&other.#pos)?)
                } else {
                    None
                },
            }
          }
        })
        .collect();

      // get from_diff field logic.
      let fields_from: Vec<TokenStream> = fields
        .iter()
        .enumerate()
        .map(|(idx, field)| {
          let marker = &field_markers[idx];
          let typ = field.typ();

          if field.should_ignore() {
            quote! { Default::default() }
          } else {
            quote! {
                #marker.map(|v| <#typ>::from_diff(v).expect("Unable to convert from diff")).unwrap_or_default()
            }
          }
        })
        .collect();

      // get into_diff field logic.
      let fields_into: Vec<TokenStream> = fields
        .iter()
        .enumerate()
        .map(|(idx, field)| {
          let marker = &field_markers[idx];
          if field.should_ignore() {
            quote! { Option::None }
          } else if field.is_option() {
            quote! {
                if #marker.clone().into_diff()? == identity_diff::DiffOption::None {
                    None
                } else {
                    Some(#marker.into_diff()?)
                }
            }
          } else {
            quote! {
                Some(#marker.into_diff()?)
            }
          }
        })
        .collect();

      // generate code for the `Diff` Trait.
      quote! {
          impl<#(#param_decls),*> identity_diff::Diff
              for #name<#params>
              #clause
          {
              type Type = #diff<#params>;

              #[allow(unused)]
              fn merge(&self, diff: Self::Type) -> identity_diff::Result<Self> {
                  Ok(Self( #(#field_merge)* ))
              }
              #[allow(unused)]
              fn diff(&self, other: &Self) -> identity_diff::Result<Self::Type> {
                  Ok(#diff( #(#fields_diff)* ))
              }

              #[allow(unused)]
              fn from_diff(diff: Self::Type) -> identity_diff::Result<Self> {
                  match diff {
                      #diff ( #(#field_markers),*) => {
                          Ok(Self( #(#fields_from),* ))
                      }
                  }
              }

              #[allow(unused)]
              fn into_diff(self) -> identity_diff::Result<Self::Type> {
                  match self {
                      Self ( #(#field_markers,)* ..) => {
                          Ok(#diff ( #(#fields_into),* ))
                      },
                  }
              }
          }
      }
    }
    // generated code for unit structs.
    SVariant::Unit => quote! {
        impl<#(#param_decls),*> identity_diff::Diff
                for #name<#params>
                #clause
            {
                type Type = #diff<#params>;

                #[allow(unused)]
                fn merge(&self, diff: Self::Type) -> identity_diff::Result<Self> {
                    Ok(Self)
                }

                fn diff(&self, other: &Self) -> identity_diff::Result<Self::Type> {
                    Ok(#diff)
                }

                #[allow(unused)]
                fn from_diff(diff: Self::Type) -> identity_diff::Result<Self> {
                    Ok(Self)
                }

                #[allow(unused)]
                fn into_diff(self) -> identity_diff::Result<Self::Type> {
                    Ok(#diff)
                }
            }
    },
  }
}
