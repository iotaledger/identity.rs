// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(non_snake_case)]

use crate::model::DataFields;
use crate::model::EVariant;
use crate::model::InputModel;
use crate::model::SVariant;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::GenericParam;
use syn::WhereClause;

/// derive a Diff type Enum from an incoming `InputModel`.
pub fn derive_diff_enum(input: &InputModel) -> TokenStream {
  // collect appropriate data and generate param declarations.
  let diff: &Ident = input.diff();
  let evariants: &Vec<EVariant> = input.e_variants();

  let serde_attrs = if input.from_into() {
    let name = input.name();
    let stype = quote!(#name).to_string();

    quote! {
        #[serde(from=#stype, into=#stype)]
    }
  } else {
    quote! {}
  };

  let param_decls: &Punctuated<GenericParam, Comma> = input.param_decls();

  let clause = quote! {};

  let param_decls: Vec<TokenStream> = param_decls
    .iter()
    .map(|p| match p {
      GenericParam::Lifetime(life) => quote! { #life },
      GenericParam::Const(cp) => quote! { #cp },
      GenericParam::Type(typ) => {
        let S: &Ident = &typ.ident;

        let bounds: Vec<TokenStream> = typ.bounds.iter().map(|bound| quote! { #bound }).collect();

        quote! {
            #S: identity_diff::Diff
            #(+ #bounds)*
        }
      }
    })
    .collect();

  // create the Diff enum body.
  let body: TokenStream = evariants
    .iter()
    .map(|var| {
      let vname = &var.name;
      let typs: Vec<TokenStream> = var.fields.iter().map(|f| f.typ_as_tokens()).collect();

      match var.variant {
        // for named variant.
        SVariant::Named => {
          let fnames: Vec<&Ident> = var.fields.iter().map(|f| f.name()).collect();

          // generate code.
          quote! {
              #vname {
                  #(
                      #[doc(hidden)] #[serde(skip_serializing_if = "Option::is_none")] #fnames: #typs,
                  )*
              },
          }
        }
        // generate code for tuple variant.
        SVariant::Tuple => quote! {
            #vname( #( #[doc(hidden)] #[serde(skip_serializing_if = "Option::is_none")] #typs, )* ),
        },
        // generate code for unit variant.
        SVariant::Unit => quote! {
            #vname,
        },
      }
    })
    .collect();

  // create the main code and insert into the body.
  quote! {
      #[derive(Clone, PartialEq)]
      #[derive(serde::Deserialize, serde::Serialize)]
      #serde_attrs
      pub enum #diff<#(#param_decls),*>
          #clause
          {
              #body
          }
  }
}

// implement Debug on the Enum from the `InputModel`.
pub fn impl_debug_enum(input: &InputModel) -> TokenStream {
  // collect appropriate data and generate param declarations.
  let diff: &Ident = input.diff();
  let evariants: &Vec<EVariant> = input.e_variants();

  let param_decls: &Punctuated<GenericParam, Comma> = input.param_decls();
  let params: &Punctuated<Ident, Comma> = input.params();

  let param_decls: Vec<TokenStream> = param_decls
    .iter()
    .map(|p| match p {
      GenericParam::Lifetime(life) => quote! { #life },
      GenericParam::Const(cp) => quote! { #cp },
      GenericParam::Type(typ) => {
        let S: &Ident = &typ.ident;

        let bounds: Vec<TokenStream> = typ.bounds.iter().map(|tb| quote! { #tb }).collect();

        quote! {
            #S: identity_diff::Diff
            + std::fmt::Debug
            #(+ #bounds)*
        }
      }
    })
    .collect();

  // create where clause from predicates.
  let clause: &WhereClause = input.clause();
  let predicates: Vec<TokenStream> = clause.predicates.iter().map(|pred| quote! { #pred }).collect();
  let clause = quote! { where #(#predicates),*};

  // get patterns and bodies.
  let (patterns, bodies) = parse_evariants(evariants, diff);

  // create a body.
  let body = quote! {
      match self {
          #(
              #patterns => #bodies,
          )*
      }
  };

  // generate code.
  quote! {
      impl<#(#param_decls),*> std::fmt::Debug for #diff<#params>
          #clause
          {
              fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
              {
                  #body
              }
          }
  }
}

/// derive the `Diff` trait for incoming Enum in `InputModel`.
pub fn impl_diff_enum(input: &InputModel) -> TokenStream {
  // collect appropriate data and generate param declarations.
  let name: &Ident = input.name();
  let diff: &Ident = input.diff();
  let evariants: &Vec<EVariant> = input.e_variants();

  let param_decls = input.param_decls();
  let params = input.params();

  let clause: &WhereClause = input.clause();

  let param_decls: Vec<TokenStream> = param_decls
    .iter()
    .map(|p| match p {
      GenericParam::Lifetime(life) => quote! { #life },
      GenericParam::Const(cp) => quote! { #cp },
      GenericParam::Type(typ) => {
        let S: &Ident = &typ.ident;

        let bounds: Vec<TokenStream> = typ.bounds.iter().map(|tb| quote! { #tb }).collect();

        quote! {
            #S: std::clone::Clone
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

  // create where clause from predicates.
  let preds: Vec<TokenStream> = clause.predicates.iter().map(|cl| quote! { #cl }).collect();
  let clause = quote! { where #(#preds),* };

  // setup vectors for merge, diff, into_diff and from_diff data.
  let mut merge_rpatterns: Vec<TokenStream> = vec![];
  let mut merge_lpatterns: Vec<TokenStream> = vec![];
  let mut merge_bodies: Vec<TokenStream> = vec![];

  let mut diff_rpatterns: Vec<TokenStream> = vec![];
  let mut diff_lpatterns: Vec<TokenStream> = vec![];
  let mut diff_bodies: Vec<TokenStream> = vec![];

  let mut from_body: Vec<TokenStream> = vec![];

  let mut into_body: Vec<TokenStream> = vec![];

  // sort through each enum variant to generate data for each function.
  evariants.iter().for_each(|var| {
    let vname = &var.name;
    let vfields = &var.fields;
    let struct_type = var.variant.clone();

    // get merge data.
    let (mlp, mrp, mb) = parse_merge(vname, vfields, struct_type.clone());
    // get diff data.
    let (dlp, drp, db) = parse_diff(vname, vfields, struct_type.clone());

    // get the from and into data.
    let (fb, ib) = parse_from_into(var, vname, vfields, diff, struct_type);

    merge_lpatterns.extend(mlp);
    merge_rpatterns.extend(mrp);
    merge_bodies.extend(mb);

    diff_lpatterns.extend(dlp);
    diff_rpatterns.extend(drp);
    diff_bodies.extend(db);

    from_body.extend(fb);
    into_body.extend(ib);
  });

  // generate the code.
  quote! {
      impl<#(#param_decls),*> identity_diff::Diff for #name<#params>
          #clause
      {
          type Type = #diff<#params>;

          #[allow(unused)]
          fn merge(&self, diff: Self::Type) -> identity_diff::Result<Self> {
              match(self, &diff) {
                  #(
                      (#merge_lpatterns, #merge_rpatterns) => {
                          #merge_bodies
                      },
                  )*
              }
          }

          fn diff(&self, other: &Self) -> identity_diff::Result<Self::Type> {
              match (self, other) {
                  #(
                      (#diff_lpatterns, #diff_rpatterns) => { #diff_bodies },
                  )*
              }
          }

          #[allow(unused)]
          fn from_diff(diff: Self::Type) -> identity_diff::Result<Self> {
              match diff {
                  #(
                      #from_body
                  )*
              }
          }

          #[allow(unused)]
          fn into_diff(self) -> identity_diff::Result<Self::Type> {
              match self {
                  #(
                      #into_body
                  )*
              }
          }
      }
  }
}

/// function that parses and sorts the variants into twp Vec<TokenStream> types.
fn parse_evariants(evariants: &[EVariant], diff: &Ident) -> (Vec<TokenStream>, Vec<TokenStream>) {
  // setup vectors for patterns and bodies.
  let mut patterns: Vec<TokenStream> = vec![];
  let mut bodies: Vec<TokenStream> = vec![];

  evariants
    .iter()
    .for_each(|var| match (var.variant.clone(), &var.name, &var.fields) {
      // Named variants.
      (SVariant::Named, vname, fields) => {
        let fnames: Vec<&Ident> = fields.iter().map(|f| f.name()).collect();
        let buf: Ident = format_ident!("buf");

        // format fields and create code.
        let fields: Vec<TokenStream> = fields
          .iter()
          .map(|f| {
            let (fname, ftyp) = (f.name(), f.typ());

            let str_name = format!("{fname}");

            if f.should_ignore() {
              quote! {
                  #buf.field(stringify!(#str_name), &#fname);
              }
            } else {
              quote! {
                  if let Some(val) = &#fname {
                      #buf.field(#str_name, &val);
                  } else {
                      #buf.field(#str_name, &None as &Option<#ftyp>);
                  }
              }
            }
          })
          .collect();

        // push into patterns and bodies.
        patterns.push(quote! {
            Self::#vname { #(#fnames),* }
        });
        bodies.push(quote! {{
            let typ_name = String::new() + stringify!(#diff) + "::" + stringify!(#vname);
            let mut #buf = f.debug_struct(&typ_name);
            #( #fields )*

            #buf.finish()
        }});
      }
      // Tuple variants.
      (SVariant::Tuple, vname, vfields) => {
        let field_max = vfields.len();
        let field_names: Vec<Ident> = (0..field_max).map(|ident| format_ident!("field_{}", ident)).collect();

        let buf: Ident = format_ident!("buf");

        let fields: Vec<TokenStream> = vfields
          .iter()
          .enumerate()
          .map(|(idx, fld)| {
            let fname = format_ident!("field_{}", idx);
            let ftyp = fld.typ();

            match field_max {
              1 => quote! {},
              _ if fld.should_ignore() => quote! {
                  #buf.field(&#fname);
              },
              _ => quote! {
                  if let Some(val) = #fname {
                      #buf.field(&val);
                  } else {
                      #buf.field(&None as &Option<#ftyp>);
                  }
              },
            }
          })
          .collect();
        // push into patterns and bodies
        patterns.push(quote! {
            Self::#vname( #(#field_names),* )
        });
        bodies.push(match field_max {
          1 => quote! {{
              let typ_name = String::new() + stringify!(#diff) + "::" + stringify!(#vname);

              if let Some(val) = &field_0 {
                  write!(f, "{}({:?})", typ_name, val)
              } else {
                  let val = &None as &Option<()>;
                  write!(f, "{}({:?})", typ_name, val)
              }
          }},
          _ => quote! {{
              let typ_name = String::new() + stringify!(#diff) + "::" + stringify!(#vname);
              let mut #buf = f.debug_tuple(&typ_name);
              #( #fields )*
              #buf.finish()
          }},
        });
      }
      // unit variant.
      (SVariant::Unit, vname, _vfields) => {
        patterns.push(quote! {
            Self::#vname
        });
        bodies.push(quote! {{
                let typ_name = String::new() + stringify!(#diff) + "::" + stringify!(#vname);
                f.debug_struct(&typ_name).finish()
        }});
      }
    });

  // return patterns and bodies.
  (patterns.to_vec(), bodies.to_vec())
}

// parse data to generate the merge functions.
fn parse_merge(
  vname: &Ident,
  vfields: &[DataFields],
  struct_type: SVariant,
) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
  let mut merge_rpatterns: Vec<TokenStream> = vec![];
  let mut merge_lpatterns: Vec<TokenStream> = vec![];
  let mut merge_bodies: Vec<TokenStream> = vec![];

  match struct_type {
    // named variant.
    SVariant::Named => {
      // get field names.
      let fnames: Vec<&Ident> = vfields.iter().map(|f| f.name()).collect();

      let (left_names, right_names) = populate_field_names(vfields, 0, struct_type);

      // setup merge code.
      let merge_fvalues: Vec<TokenStream> = vfields
        .iter()
        .zip(left_names.iter())
        .zip(right_names.iter())
        .map(|((f, ln), rn)| {
          if f.should_ignore() {
            quote! { #ln.clone() }
          } else {
            quote! {
                if let Some(diff) = #rn {
                    #ln.merge(diff.clone())?
                } else {
                    #ln.clone()
                }
            }
          }
        })
        .collect();

      // push merge code into vectors.
      merge_lpatterns.push(quote! {
          Self::#vname {
              #(#fnames: #left_names),*
          }
      });

      merge_rpatterns.push(quote! {
          Self::Type::#vname {
              #(#fnames: #right_names),*
          }
      });

      merge_bodies.push(quote! {
          Ok(Self::#vname {
              #(#fnames: #merge_fvalues),*
          })
      });

      merge_lpatterns.push(quote! {_});

      merge_rpatterns.push(quote! {
          diff @Self::Type::#vname { .. }
      });

      merge_bodies.push(quote! {
          Self::from_diff(diff.clone())
      });

      (
        merge_lpatterns.to_vec(),
        merge_rpatterns.to_vec(),
        merge_bodies.to_vec(),
      )
    }
    // tuple variants.
    SVariant::Tuple => {
      let (left_names, right_names) = populate_field_names(vfields, vfields.len(), struct_type);

      // setup merge logic.
      let merge_fvalues: Vec<TokenStream> = vfields
        .iter()
        .zip(left_names.iter())
        .zip(right_names.iter())
        .map(|((f, ln), rn)| {
          if f.should_ignore() {
            quote! { #ln.clone() }
          } else {
            quote! {
                if let Some(diff) = #rn {
                    #ln.merge(diff.clone())?
                } else {
                    #ln.clone()
                }
            }
          }
        })
        .collect();

      // push into vectors.
      merge_lpatterns.push(quote! {
          Self::#vname(
              #(#left_names),*
          )
      });

      merge_rpatterns.push(quote! {
          Self::Type::#vname(
              #(#right_names),*
          )
      });

      merge_bodies.push(quote! {
          Ok(Self::#vname(#(#merge_fvalues),*))
      });

      merge_lpatterns.push(quote! { _ });

      merge_rpatterns.push(quote! {
          diff @ Self::Type::#vname(..)
      });

      merge_bodies.push(quote! {
          Self::from_diff(diff.clone())
      });
      (
        merge_lpatterns.to_vec(),
        merge_rpatterns.to_vec(),
        merge_bodies.to_vec(),
      )
    }
    // unit variants.
    SVariant::Unit => {
      // push into vectors.
      merge_lpatterns.push(quote! {
          Self::#vname
      });

      merge_rpatterns.push(quote! {
          Self::Type::#vname
      });

      merge_bodies.push(quote! {
          Ok(Self::#vname)
      });

      merge_lpatterns.push(quote! { _ });

      merge_rpatterns.push(quote! {
          diff @ Self::Type::#vname
      });

      merge_bodies.push(quote! {
          Self::from_diff(diff.clone())
      });

      // return generated code.
      (
        merge_lpatterns.to_vec(),
        merge_rpatterns.to_vec(),
        merge_bodies.to_vec(),
      )
    }
  }
}

/// parses data for the derived diff function.
fn parse_diff(
  vname: &Ident,
  vfields: &[DataFields],

  struct_type: SVariant,
) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
  let mut diff_rpatterns: Vec<TokenStream> = vec![];
  let mut diff_lpatterns: Vec<TokenStream> = vec![];
  let mut diff_bodies: Vec<TokenStream> = vec![];

  match struct_type {
    // named variant.
    SVariant::Named => {
      let fnames: Vec<&Ident> = vfields.iter().map(|f| f.name()).collect();

      let (left_names, right_names) = populate_field_names(vfields, 0, struct_type);

      // setup diff logic.
      let diff_fvalues: Vec<TokenStream> = vfields
        .iter()
        .zip(left_names.iter())
        .zip(right_names.iter())
        .map(|((f, ln), rn)| {
          if f.should_ignore() {
            quote! {
                Option::None
            }
          } else if f.is_option() {
            quote! {
                if #ln != #rn && *#ln != None && *#rn != None {
                    Some(#ln.diff(#rn)?)
                } else {
                    None
                }
            }
          } else {
            quote! {
                if #ln == #rn {
                    None
                } else {
                    Some(#ln.diff(#rn)?)
                }
            }
          }
        })
        .collect();

      // push into vectors.
      diff_lpatterns.push(quote! {
          Self::#vname { #(#fnames: #left_names),* }
      });

      diff_rpatterns.push(quote! {
          Self::#vname { #(#fnames: #right_names),* }
      });

      diff_bodies.push(quote! {
          Ok(Self::Type::#vname {
              #(#fnames: #diff_fvalues),*
          })
      });

      diff_lpatterns.push(quote! { _ });

      diff_rpatterns.push(quote! { other @ Self::#vname {..} });
      diff_bodies.push(quote! {
          other.clone().into_diff()
      });
    }
    // tuple variants.
    SVariant::Tuple => {
      let (left_names, right_names) = populate_field_names(vfields, vfields.len(), struct_type);

      // setup diff logic.
      let diff_fvalues: Vec<TokenStream> = vfields
        .iter()
        .zip(left_names.iter().zip(right_names.iter()))
        .map(|(f, (ln, rn))| {
          if f.should_ignore() {
            quote! {
                Option::None
            }
          } else if f.is_option() {
            quote! {
                if #ln != #rn && *#ln != None && *#rn != None {
                    Some(#ln.diff(#rn)?)
                } else {
                    None
                }
            }
          } else {
            quote! {
                if #ln == #rn {
                    None
                } else {
                    Some(#ln.diff(#rn)?)
                }
            }
          }
        })
        .collect();

      // push into vectors.
      diff_lpatterns.push(quote! {
          Self::#vname( #(#left_names),* )
      });

      diff_rpatterns.push(quote! {
          Self::#vname( #(#right_names),* )
      });

      diff_bodies.push(quote! {
          Ok(Self::Type::#vname( #(#diff_fvalues),* ))
      });

      diff_lpatterns.push(quote! {_});

      diff_rpatterns.push(quote! { other @ Self::#vname(..) });
      diff_bodies.push(quote! {
          other.clone().into_diff()
      });
    }
    // push into vectors for unit variants.
    SVariant::Unit => {
      diff_lpatterns.push(quote! {
          Self::#vname
      });

      diff_rpatterns.push(quote! {
          Self::#vname
      });

      diff_lpatterns.push(quote! { _ });

      diff_rpatterns.push(quote! {
          other @ Self::#vname
      });

      diff_bodies.push(quote! {
          Ok(Self::Type::#vname)
      });

      diff_bodies.push(quote! {
          other.clone().into_diff()
      });
    }
  }
  (diff_lpatterns.to_vec(), diff_rpatterns.to_vec(), diff_bodies.to_vec())
}

// parse data for from_diff and into_diff functions.
fn parse_from_into(
  var: &EVariant,
  vname: &Ident,
  vfields: &[DataFields],
  diff: &Ident,
  struct_type: SVariant,
) -> (Vec<TokenStream>, Vec<TokenStream>) {
  let mut from_body: Vec<TokenStream> = vec![];
  let mut into_body: Vec<TokenStream> = vec![];

  match struct_type {
    // named structs.
    SVariant::Named => {
      let fnames: Vec<&Ident> = vfields.iter().map(|f| f.name()).collect();
      // setup from logic.
      let from_fassign: Vec<TokenStream> = var
        .fields
        .iter()
        .map(|f| {
          let fname = f.name();
          let ftyp = f.typ();

          if f.should_ignore() {
            quote! {#fname: Default::default()}
          } else {
            quote! {
                #fname: <#ftyp>::from_diff(
                    match #fname {
                        Some(v) => v,
                        None => <#ftyp>::default().into_diff()?
                    }
                )?
            }
          }
        })
        .collect();

      // setup into logic.
      let into_fassign: Vec<TokenStream> = var
        .fields
        .iter()
        .map(|f| {
          let fname = f.name();

          if f.should_ignore() {
            quote! { #fname: Option::None }
          } else if f.is_option() {
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

      // push into vectors.
      from_body.push(quote! {
          #diff::#vname { #(#fnames),* } => {
              Ok(Self::#vname { #(#from_fassign),* })
          }
      });

      into_body.push(quote! {
          Self::#vname { #(#fnames),* } => {
              Ok(#diff::#vname { #(#into_fassign),* })
          }
      });
    }
    // tuple variants.
    SVariant::Tuple => {
      let fnames: Vec<Ident> = (0..vfields.len())
        .map(|ident| format_ident!("field_{}", ident))
        .collect();

      // from logic.
      let from_fassign: Vec<TokenStream> = var
        .fields
        .iter()
        .enumerate()
        .map(|(idx, f)| {
          let fname = &fnames[idx];
          let ftyp = f.typ();

          if f.should_ignore() {
            quote! { Default::default() }
          } else {
            quote! {
                <#ftyp>::from_diff(
                    match #fname {
                        Some(v) => v,
                        None => <#ftyp>::default().into_diff()?
                    }
                )?
            }
          }
        })
        .collect();

      // into logic.
      let into_fassign: Vec<TokenStream> = var
        .fields
        .iter()
        .enumerate()
        .map(|(idx, f)| {
          let fname = &fnames[idx];

          if f.should_ignore() {
            quote! { Option::None }
          } else if f.is_option() {
            quote! {
                if #fname.clone().into_diff()? == identity_diff::DiffOption::None {
                    None
                } else {
                    Some(#fname.into_diff()?)
                }
            }
          } else {
            quote! {
                Some(#fname.into_diff()?)
            }
          }
        })
        .collect();

      // push code into vectors.
      from_body.push(quote! {
          #diff::#vname( #(#fnames),* ) => {
              Ok(Self::#vname( #(#from_fassign),* ))
          }
      });

      into_body.push(quote! {
          Self::#vname( #(#fnames),* ) => {
              Ok(#diff::#vname(
                  #(#into_fassign),*
              ))
          }
      });
    }
    // setup code for unit variants.
    SVariant::Unit => {
      from_body.push(quote! {
         #diff::#vname => {
              Ok(Self::#vname)
          },
      });

      into_body.push(quote! {
          Self::#vname => {
              Ok(#diff::#vname)
          },
      });
    }
  }

  (from_body.to_vec(), into_body.to_vec())
}

// create field names based on thee size of an enum.
fn populate_field_names(vfields: &[DataFields], fmax: usize, struct_type: SVariant) -> (Vec<Ident>, Vec<Ident>) {
  match struct_type {
    SVariant::Named => (
      vfields
        .iter()
        .map(|f| f.name())
        .map(|ident| format_ident!("left_{}", ident))
        .collect(),
      vfields
        .iter()
        .map(|f| f.name())
        .map(|ident| format_ident!("right_{}", ident))
        .collect(),
    ),
    SVariant::Tuple => (
      (0..fmax).map(|ident| format_ident!("left_{}", ident)).collect(),
      (0..fmax).map(|ident| format_ident!("right_{}", ident)).collect(),
    ),
    _ => panic!("Wrong Struct Type!"),
  }
}
