#![allow(non_snake_case)]

use crate::model::{DataFields, EVariant, InputModel, InputStruct, SVariant};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, token::Comma, Data, DataEnum, DataStruct, DeriveInput, Fields, GenericParam, Token, Type,
    Variant, WhereClause,
};

use identity_diff::Diff;

pub fn derive_diff_enum(input: &InputModel) -> TokenStream {
    let diff: &Ident = input.diff();
    let evariants: &Vec<EVariant> = input.e_variants();

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

    let body: TokenStream = evariants
        .iter()
        .map(|var| {
            let vname = &var.name;
            let typs: Vec<TokenStream> = var.fields.iter().map(|f| f.typ_as_tokens()).collect();

            match var.variant {
                SVariant::Named => {
                    let fnames: Vec<&Ident> = var.fields.iter().map(|f| f.name()).collect();

                    quote! {
                        #vname {
                            #(
                                #[doc(hidden)] #fnames: #typs,
                            )*
                        },
                    }
                }
                SVariant::Tuple => quote! {
                    #vname( #( #[doc(hidden)] #typs, )* ),
                },
                SVariant::Unit => quote! {
                    #vname,
                },
            }
        })
        .collect();

    quote! {
        #[derive(Clone, PartialEq)]
        #[derive(serde::Deserialize, serde::Serialize)]
        pub enum #diff<#(#param_decls),*>
            #clause
            {
                #body
            }
    }
}

pub fn impl_debug_enum(input: &InputModel) -> TokenStream {
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

    let clause: &WhereClause = input.clause();

    let predicates: Vec<TokenStream> = clause.predicates.iter().map(|pred| quote! { #pred }).collect();

    let clause = quote! { where #(#predicates),*};

    let mut patterns: Vec<TokenStream> = vec![];
    let mut bodies: Vec<TokenStream> = vec![];

    for var in evariants.iter() {
        match (var.variant.clone(), &var.name, &var.fields) {
            (SVariant::Named, vname, fields) => {
                let fnames: Vec<&Ident> = fields.iter().map(|f| f.name()).collect();
                let buf: Ident = format_ident!("buf");

                let fields: Vec<TokenStream> = fields
                    .iter()
                    .map(|f| {
                        let fname = f.name();
                        let typ = f.typ();

                        if f.should_ignore() {
                            quote! {
                                #buf.field(stringify!(#fname), &#fname)
                            }
                        } else {
                            quote! {
                                let fname: &'static str = stringify!(#fname);
                                if let Some(#fname) = &#fname {
                                    #buf.field(fname, &#fname);
                                } else {
                                    #buf.field(fname, &None as &Option<#typ>);
                                }
                            }
                        }
                    })
                    .collect();
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
            (SVariant::Tuple, vname, vfields) => {
                let field_typs: Vec<&Type> = vfields.iter().map(|f| f.typ()).collect();

                let field_max = field_typs.len();
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
                                if let Some(field) = #fname {
                                    #buf.field(&field);
                                } else {
                                    #buf.field(&None as &Option<#ftyp>);
                                }
                            },
                        }
                    })
                    .collect();
                patterns.push(quote! {
                    Self::#vname( #(#field_names),* )
                });
                bodies.push(match field_max {
                    1 => quote! {{
                        let typ_name = String::new() + stringify!(#diff) + "::" + stringify!(#vname);

                        if let Some(field) = &0_field {
                            write!(f, "{}({:?})", typ_name, field)
                        } else {
                            let field = &None as &Option<()>;
                            write!(f, "{}({:?})", typ_name, field)
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
            (SVariant::Unit, vname, _vfields) => {
                patterns.push(quote! {
                    Self::#vname
                });
                bodies.push(quote! {{
                        let typ_name = String::new() + stringify!(#diff) + "::" + stringify!(#vname);
                        f.debug_struct(&typ_name).finish()
                }});
            }
        }
    }

    let body = quote! {
        match self {
            #(
                #patterns => #bodies,
            )*
        }
    };

    quote! {
        impl<#(#param_decls),*> std::fmt::Debug for #diff<#params>
            #clause
            {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
                {
                    #body
                }
            }
    }
}

pub fn impl_diff_enum(input: &InputModel) -> TokenStream {
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

    let preds: Vec<TokenStream> = clause.predicates.iter().map(|cl| quote! { #cl }).collect();
    let clause = quote! { where #(#preds),* };

    let mut merge_rpatterns: Vec<TokenStream> = vec![];
    let mut merge_lpatterns: Vec<TokenStream> = vec![];
    let mut merge_bodies: Vec<TokenStream> = vec![];

    let mut diff_rpatterns: Vec<TokenStream> = vec![];
    let mut diff_lpatterns: Vec<TokenStream> = vec![];
    let mut diff_bodies: Vec<TokenStream> = vec![];

    let mut from_body: Vec<TokenStream> = vec![];

    let mut into_body: Vec<TokenStream> = vec![];

    for var in evariants.iter() {
        match (var.variant.clone(), &var.name, &var.fields) {
            (SVariant::Named, vname, vfields) => {
                let fnames: Vec<&Ident> = vfields.iter().map(|f| f.name()).collect();
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
                                        None => <#ftyp>::default().into_diff()
                                    }
                                )
                            }
                        }
                    })
                    .collect();

                let into_fassign: Vec<TokenStream> = var
                    .fields
                    .iter()
                    .map(|f| {
                        let fname = f.name();

                        if f.should_ignore() {
                            quote! { #fname: std::marker::PhantomData }
                        } else {
                            quote! {
                                #fname: Some(#fname.into_diff())
                            }
                        }
                    })
                    .collect();

                let left_names: Vec<Ident> = vfields
                    .iter()
                    .map(|f| f.name())
                    .map(|ident| format_ident!("left_{}", ident))
                    .collect();

                let right_names: Vec<Ident> = vfields
                    .iter()
                    .map(|f| f.name())
                    .map(|ident| format_ident!("right_{}", ident))
                    .collect();

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
                                    #ln.merge(diff.clone())
                                } else {
                                    #ln.clone()
                                }
                            }
                        }
                    })
                    .collect();

                let diff_fvalues: Vec<TokenStream> = vfields
                    .iter()
                    .zip(left_names.iter())
                    .zip(right_names.iter())
                    .map(|((f, ln), rn)| {
                        if f.should_ignore() {
                            quote! {
                                std::marker::PhantomData
                            }
                        } else {
                            quote! {
                                if #ln == #rn {
                                    None
                                } else {
                                    Some(#ln.diff(#rn))
                                }
                            }
                        }
                    })
                    .collect();

                from_body.push(quote! {
                    #diff::#vname { #(#fnames),* } => {
                        Self::#vname { #(#from_fassign),* }
                    }
                });

                into_body.push(quote! {
                    Self::#vname { #(#fnames),* } => {
                        #diff::#vname { #(#into_fassign),* }
                    }
                });

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
                    Self::#vname {
                        #(#fnames: #merge_fvalues),*
                    }
                });

                merge_lpatterns.push(quote! {_});

                merge_rpatterns.push(quote! {
                    diff @Self::Type::#vname {.. }
                });

                merge_bodies.push(quote! {
                    Self::from_diff(diff.clone())
                });

                diff_lpatterns.push(quote! {
                    Self::#vname { #(#fnames: #left_names),* }
                });

                diff_rpatterns.push(quote! {
                    Self::#vname { #(#fnames: #right_names),* }
                });

                diff_bodies.push(quote! {
                    Self::Type::#vname {
                        #(#fnames: #diff_fvalues),*
                    }
                });

                diff_lpatterns.push(quote! { _ });

                diff_rpatterns.push(quote! { other @ Self::#vname {..} });
                diff_bodies.push(quote! {
                    other.clone().into_diff()
                });
            }
            (SVariant::Tuple, vname, vfields) => {
                let ftyps: Vec<&Type> = vfields.iter().map(|f| f.typ()).collect();

                let fmax = ftyps.len();

                let fnames: Vec<Ident> = (0..fmax).map(|ident| format_ident!("field_{}", ident)).collect();

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
                                    #fname
                                )
                            }
                        }
                    })
                    .collect();

                let into_fassign: Vec<TokenStream> = var
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(idx, f)| {
                        let fname = &fnames[idx];

                        if f.should_ignore() {
                            quote! { std::marker::PhantomData }
                        } else {
                            quote! {
                                Some(#fname.into_diff())
                            }
                        }
                    })
                    .collect();

                let left_names: Vec<Ident> = (0..fmax).map(|ident| format_ident!("left_{}", ident)).collect();

                let right_names: Vec<Ident> = (0..fmax).map(|ident| format_ident!("right_{}", ident)).collect();

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
                                    #ln.merge(diff.clone())
                                } else {
                                    #ln.clone()
                                }
                            }
                        }
                    })
                    .collect();

                let diff_fvalues: Vec<TokenStream> = vfields
                    .iter()
                    .zip(left_names.iter().zip(right_names.iter()))
                    .map(|(f, (ln, rn))| {
                        if f.should_ignore() {
                            quote! {
                                std::marker::PhantomData
                            }
                        } else {
                            quote! {
                                if #ln == #rn {
                                    None
                                } else {
                                    Some(#ln.diff(#rn))
                                }
                            }
                        }
                    })
                    .collect();

                from_body.push(quote! {
                    #diff::#vname( #(#fnames),* ) => {
                        Self::#vname( #(#from_fassign),* )
                    }
                });

                into_body.push(quote! {
                    Self::#vname( #(#fnames),* ) => {
                        #diff::#vname(
                            #(#into_fassign),*
                        )
                    }
                });

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
                    Self::#vname(#(#merge_fvalues),*)
                });

                merge_lpatterns.push(quote! { _ });

                merge_rpatterns.push(quote! {
                    diff @ Self::Type::#vname(..)
                });

                merge_bodies.push(quote! {
                    Self::from_diff(diff.clone())
                });

                diff_lpatterns.push(quote! {
                    Self::#vname( #(#left_names),* )
                });

                diff_rpatterns.push(quote! {
                    Self::#vname( #(#right_names),* )
                });

                diff_bodies.push(quote! {
                    Ok(Self::Delta::#vname( #(#diff_fvalues),* ))
                });

                diff_lpatterns.push(quote! {
                    quote! {_}
                });

                diff_rpatterns.push(quote! { other @ Self::#vname(..) });
                diff_bodies.push(quote! {
                    other.clone.into_diff()
                });
            }
            (SVariant::Unit, vname, vfields) => {
                from_body.push(quote! {
                    #diff::#vname => {
                        Self::#vname
                    },
                });

                into_body.push(quote! {
                    Self::#vname => {
                        #diff::#vname
                    },
                });

                merge_lpatterns.push(quote! {
                    Self::#vname
                });

                merge_rpatterns.push(quote! {
                    Self::Type::#vname
                });

                merge_bodies.push(quote! {
                    Self::#vname
                });

                merge_lpatterns.push(quote! { _ });

                merge_rpatterns.push(quote! {
                    diff @ Self::Type::#vname
                });

                merge_bodies.push(quote! {
                    Self::from_diff(diff.clone())
                });

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
                    Self::Type::#vname
                });

                diff_bodies.push(quote! {
                    other.clone().into_diff()
                });
            }
        }
    }

    quote! {
        impl<#(#param_decls),*> identity_diff::Diff for #name<#params>
            #clause
        {
            type Type = #diff<#params>;

            #[allow(unused)]
            fn merge(&self, diff: Self::Type) -> Self {
                match(self, &diff) {
                    #(
                        (#merge_lpatterns, #merge_rpatterns) => {
                            #merge_bodies
                        },
                    )*
                }
            }

            fn diff(&self, other: &Self) -> Self::Type {
                match (self, other) {
                    #(
                        (#diff_lpatterns, #diff_rpatterns) => { #diff_bodies },
                    )*
                }
            }

            #[allow(unused)]
            fn from_diff(diff: Self::Type) -> Self {
                match diff {
                    #(
                        #from_body
                    )*
                }
            }

            #[allow(unused)]
            fn into_diff(self) -> Self::Type {
                match self {
                    #(
                        #into_body
                    )*
                }
            }
        }
    }
}
