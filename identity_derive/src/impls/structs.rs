use crate::model::{DataFields, InputModel, InputStruct, SVariant};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, token::Comma, Data, DataEnum, DataStruct, DeriveInput, Fields, GenericParam, Token, Type,
    Variant, WhereClause,
};

use identity_diff::Diff;

pub fn derive_diff_struct(input: &InputModel) -> TokenStream {
    let svariant = input.s_variant();
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
                    #S: identity_diff::Diff
                    #(+ #bounds)*
                }
            }
        })
        .collect();

    let field_tps: Vec<TokenStream> = fields.iter().map(|field| field.typ_as_tokens()).collect();
    match svariant {
        SVariant::Named => {
            let field_names: Vec<&Ident> = fields.iter().map(|field| field.name()).collect();

            quote! {
                #[derive(Clone, PartialEq, Default)]
                #[derive(serde::Deserialize, serde::Serialize)]
                pub struct #diff<#(#param_decls),*>
                    #clause
                {
                    #( #[doc(hidden)] pub(self) #field_names: #field_tps, )*
                }
            }
        }
        SVariant::Tuple => {
            quote! {
                #[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
                pub struct #diff<#(#param_decls),*> (
                    #( #[doc(hidden)] pub(self) #field_tps, )*
                ) #clause ;
            }
        }
        SVariant::Unit => quote! {
            #[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
            pub struct #diff<#(#param_decls),*> #clause ;
        },
    }
}

pub fn debug_impl(input: &InputModel) -> TokenStream {
    let svariant = input.s_variant();
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
                    #S: identity_diff::Diff + std::fmt::Debug
                    #(+ #bounds)*
                }
            }
        })
        .collect();

    let preds: Vec<TokenStream> = clause.predicates.iter().map(|pred| quote! { #pred }).collect();
    let clause = quote! { where #(#preds),*};

    let field_tps: Vec<TokenStream> = fields.iter().map(|field| field.typ_as_tokens()).collect();

    match svariant {
        SVariant::Named => {
            let mut mac = TokenStream::new();
            let buf: Ident = format_ident!("buf");
            for field in fields.iter() {
                let (fname, ftype) = (field.name(), field.typ());
                mac.extend(if field.should_ignore() {
                    quote! {
                        #buf.field(stringify!(#fname), &self.#fname);
                    }
                } else {
                    quote! {
                        let fname = stringify!(#fname);
                        if let Some(#fname) = &self.#fname {
                            #buf.field(fname, #fname);
                        } else {
                            #buf.field(fname, &None as &Option<#ftype>);
                        }
                    }
                });
            }

            quote! {
                impl<#(#param_decls),*> std::fmt::Debug
                    for #diff<#params>
                    #clause
                    {
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                            const NAME: &str = stringify!(#diff);
                            let mut #buf = f.debug_struct(NAME);
                            #mac
                            #buf.finish()
                        }
                    }
            }
        }
        SVariant::Tuple => {
            let ftyps: Vec<&Type> = fields.iter().map(|field| field.typ()).collect();
            let count = fields.len();

            let mut f_tokens = TokenStream::new();
            let buf = format_ident!("buf");
            for field in fields.iter() {
                let (fpos, ftyp) = (field.position(), field.typ());
                let fname = format_ident!("field");
                f_tokens.extend(match count {
                    1 => quote! {},
                    _ if field.should_ignore() => quote! {
                        #buf.field(&self.#fpos);
                    },
                    _ => quote! {
                        if let Some(#fname) = &self.#fpos {
                            #buf.field(#fname);
                        } else {
                            #buf.field(&None as &Option<#ftyp>);
                        }
                    },
                });
            }
            let mac = match count {
                1 => quote! {
                    const NAME: &str = stringify!(#diff);
                    if let Some(field) = &self.0 {
                        write!(f, "{}({:?})", name, field)
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

            quote! {
                impl<#(#param_decls),*> std::fmt::Debug for #diff<#params>
                    #clause
                    {
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                            #mac
                        }
                    }
            }
        }
        SVariant::Unit => quote! {
            quote! {
            impl<#(#param_decls),*> std::fmt::Debug
                    for #diff<#params>
                    #clause
                {
                    fn fmt(&self, f: &mut std::fmt::Formatter)
                        -> std::fmt::Result
                    {
                        f.debug_struct(stringify!(#diff))
                            .finish()
                    }
                }
            }
        },
    }
}

pub fn diff_impl(input: &InputModel) -> TokenStream {
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
                    + identity_diff::Diff
                    + std::debug::std::fmt::Debug
                    + std::cmp::PartialEq
                    + for<'de> serde::Deserialize<'de>
                    + serde::Serialize
                    #(+ #bounds)*
                }
            }
        })
        .collect();

    let preds: Vec<TokenStream> = clause.predicates.iter().map(|pred| quote! { #pred }).collect();
    let clause = quote! { where #(#preds),*};

    match svariant {
        SVariant::Named => {
            let fnames: Vec<&Ident> = fields.iter().map(|field| field.name()).collect();
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
                                self.#fname.merge(d)
                            } else {
                                self.#fname.clone()
                            },
                        }
                    }
                })
                .collect();

            let fields_diff: Vec<TokenStream> = fields
                .iter()
                .map(|field| {
                    let fname = field.name();
                    if field.should_ignore() {
                        quote! {
                            #fname: std::marker::PhantomData,
                        }
                    } else {
                        quote! {
                            #fname: if self.#fname != other.fname {
                                Some(self.#fname.diff(&other.fname))
                            } else {
                                None
                            },
                        }
                    }
                })
                .collect();

            let fields_from: Vec<TokenStream> = fields
                .iter()
                .map(|field| {
                    let fname = field.name();
                    let ftyp = field.typ();
                    if field.should_ignore() {
                        quote! {#fname: Default::default() }
                    } else {
                        quote! {
                            #fname: <#ftyp>::from_diff(
                                #fname
                            )
                        }
                    }
                })
                .collect();

            let fields_into: Vec<TokenStream> = fields
                .iter()
                .map(|field| {
                    let fname = field.name();
                    if field.should_ignore() {
                        quote! {#fname: std::marker::PhantomData }
                    } else {
                        quote! {
                            #fname: Some(#fname.into_diff())
                        }
                    }
                })
                .collect();
            quote! {
                impl<#(#param_decls),*> identity_diff::Diff
                    for #name<#params>
                    #clause
                {
                    type Type = #diff<#params>;

                    #[allow(unused)]
                    fn merge(&self, diff: Self::Type) -> Self {
                        Self{ #(#field_merge),* }
                    }

                    fn diff(&self, other: &Self) -> Self::Type {
                        #diff { #(#fields_diff),* }
                    }

                    #[allow(unused)]
                    fn from_diff(diff: Self::Type) -> Self {
                        match diff {
                            #diff { #(#fnames), *} => {
                                Self{ #(#fields_from, )* }
                            }
                        }
                    }

                    #[allow(unused)]
                    fn into_diff(self) -> Self::Type {
                        match self {
                            Self { #(#fnames),* .. } => {
                                #diff { #(#fields_into, )* }
                            },
                        }
                    }
                }

            }
        }
        SVariant::Tuple => {
            quote! { unimplemented!()}
        }
        SVariant::Unit => quote! {
            quote! { unimplemented!() }
        },
    }
}
