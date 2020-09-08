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

    let param_decls = input.param_decls();
    let params = input.params();

    let clause = quote! {};

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
                    #(+ #bounds)*
                }
            }
        })
        .collect();

    let body: TokenStream = evariants
        .iter()
        .map(|var| {
            let name = &var.name;
            let typs: Vec<TokenStream> = var.fields.iter().map(|f| f.typ_as_tokens()).collect();

            match var.variant {
                SVariant::Named => {
                    let names: Vec<&Ident> = var.fields.iter().map(|f| f.name()).collect();

                    quote! {
                        #name {
                            #(
                                #[doc(hidden)] #names: #typs,
                            )*
                        },
                    }
                }
                SVariant::Tuple => quote! {
                    #name( #( #[doc(hidden)] #typs, )* ),
                },
                SVariant::Unit => quote! {
                    #name,
                },
            }
        })
        .collect();

    quote! {
        #[derive(Clone, PartialEq, Default)]
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

    let param_decls = input.param_decls();
    let params = input.params();

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
                        let n = f.name();
                        let typ = f.typ();

                        if f.should_ignore() {
                            quote! {
                                #buf.field(stringify!(#n), &#n)
                            }
                        } else {
                            quote! {
                                let nam = stringify!(#n);
                                if let Some(#vname) = &#vname {
                                    #buf.field(na, &#vname);
                                } else {
                                    #buf.field(na, &None as &Option<#typ>);
                                }
                            }
                        }
                    })
                    .collect();
                patterns.push(quote! {
                    Self::#vname { #(#fnames),* }
                });
                bodies.push(quote! {{
                    let typ_name = String::new() + stringify!(#diff) + "::" + stringify(#vname);
                    let mut #buf = f.debug_struct(&typ_name);
                    #( #fields )*

                    #buf.finish()
                }});
            }
            (SVariant::Tuple, vname, vfields) => {
                let field_typs: Vec<&Type> = vfields.iter().map(|f| f.typ()).collect();

                let field_max = field_typs.len();
                let field_names: Vec<Ident> = (0..field_max).map(|ident| format_ident!("{}_field", ident)).collect();

                let buf: Ident = format_ident!("buf");

                let fields: Vec<TokenStream> = vfields
                    .iter()
                    .enumerate()
                    .map(|(idx, fld)| {
                        let fname = format_ident!("{}_field", idx);
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
                    1 => quote! {
                        let typ_name = String::new() + stringify!(#diff) + "::" + stringify(#vname);

                        if let Some(field) = &0_field {
                            write!(f, "{}({:?})", typ_name, field)
                        } else {
                            let field = &None as &Option<()>;

                            write!(f, "{}({:?})", typ_name, field);
                        }
                    },
                    _ => quote! {
                        let typ_name = String::new() + stringify!(#diff) + "::" + stringify(#vname);
                        let mut #buf = f.debug_tuple(&typ_name);
                        #( #fields)*
                        #buf.finish()
                    },
                });
            }
            (SVariant::Unit, vname, _vfields) => {
                patterns.push(quote! {
                    Self::#vname
                });
                bodies.push(quote! {
                        let typ_name = String::new() + stringify!(#diff) + "::" + stringify(#vname);
                        f.debug_struct(&typ_name).finish();
                });
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
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

    let preds: Vec<TokenStream> = clause.predicates.iter().map(|cl| quote! { #cl }).collect();
    let clause = quote! { where #(#preds),* };

    let mut diff_patterns: Vec<TokenStream> = vec![];
    let mut self_patterns: Vec<TokenStream> = vec![];
    let mut bodies: Vec<TokenStream> = vec![];

    for var in evariants.iter() {
        match (var.variant.clone(), &var.name, &var.fields) {
            (SVariant::Named, vname, vfields) => {
                let fnames: Vec<&Ident> = vfields.iter().map(|f| f.name()).collect();

                let self_names: Vec<Ident> = vfields
                    .iter()
                    .map(|f| f.name())
                    .map(|ident| format_ident!("self_{}", ident))
                    .collect();

                let diff_names: Vec<Ident> = vfields
                    .iter()
                    .map(|f| f.name())
                    .map(|ident| format_ident!("diff_{}", ident))
                    .collect();

                let fvalues: Vec<TokenStream> = vfields
                    .iter()
                    .zip(self_names)
                    .zip(diff_names)
                    .map(|((f, sn), dn)| {
                        if f.should_ignore() {
                            quote! { #sn.clone() }
                        } else {
                            quote! {
                                if let Some(diff) = #dn {
                                    #sn.merge(diff.clone())
                                } else {
                                    #sn.clone()
                                }
                            }
                        }
                    })
                    .collect();

                self_patterns.push(quote! {
                    Self::#vname {
                        #(#fnames: #self_names),*
                    }
                });

                diff_patterns.push(quote! {
                    Self::Type::#vname {
                        #(#fnames: #diff_names),*
                    }
                });

                bodies.push(quote! {
                    Self::from_diff(diff.clone())
                });
            }
            (SVariant::Tuple, vname, vfields) => {
                let ftyps: Vec<&Type> = vfields.iter().map(|f| f.typ()).collect();

                let fmax = ftyps.len();

                let self_names: Vec<Ident> = (0..fmax).map(|ident| format_ident!("self_{}", ident)).collect();

                let diff_names: Vec<Ident> = (0..fmax).map(|ident| format_ident!("diff_{}", ident)).collect();

                let fvalues: Vec<TokenStream> = vfields
                    .iter()
                    .zip(self_names.iter())
                    .zip(diff_names.iter())
                    .map(|((f, sn), dn)| {
                        if f.should_ignore() {
                            quote! { #sn.clone() }
                        } else {
                            quote! {
                                if let Some(diff) = #dn {
                                    #sn.merge(diff.clone())
                                } else {
                                    #sn.clone()
                                }
                            }
                        }
                    })
                    .collect();

                self_patterns.push(quote! {
                    Self::#vname(
                        #(#self_names),*
                    )
                });

                self_patterns.push(quote! { _ });

                diff_patterns.push(quote! {
                    Self::Type::#vname(
                        #(#self_names),*
                    )
                });

                diff_patterns.push(quote! {
                    diff @ Self::Type::#vname(..)
                });

                bodies.push(quote! {
                    Self::#vname(#(#fvalues),*)
                });

                bodies.push(quote! {
                    Self::from_diff(diff.clone())
                });
            }
            (SVariant::Unit, vname, vfields) => {
                self_patterns.push(quote! {
                    Self::#vname
                });

                self_patterns.push(quote! { _ });

                diff_patterns.push(quote! {
                    Self::Type::#vname
                });

                diff_patterns.push(quote! {
                    diff @ Self::Type::#vname
                });

                bodies.push(quote! {
                    Self::#vname
                });

                bodies.push(quote! {
                    Self::from_diff(diff.clone())
                });
            }
        }
    }

    quote! {
        impl<#(param_decls),*> identity_diff::Diff for #name<#params>
            #clause
        {
            type Type = #diff<#params>;

            #[allow(unused)]
            fn merge(&self, diff: Self::Type) -> Self {
                match(self, &diff) {
                    #(
                        (#self_patterns, #diff_patterns) => {
                            #bodies
                        },
                    )*
                }
            }

            fn diff(&self, other: &Self) -> Self::Type {
            }

            #[allow(unused)]
            fn from_diff(diff: Self::Type) -> Self {
            }

            #[allow(unused)]
            fn into_diff(self) -> Self::Type {
            }
        }
    }
}
