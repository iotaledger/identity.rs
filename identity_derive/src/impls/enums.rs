#![allow(non_snake_case)]

use crate::model::{DataFields, EVariant, InputModel, SVariant};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, token::Comma, GenericParam, Type, WhereClause};

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
                                #[doc(hidden)] #[serde(skip_serializing_if = "Option::is_none")] #fnames: #typs,
                            )*
                        },
                    }
                }
                SVariant::Tuple => quote! {
                    #vname( #( #[doc(hidden)] #[serde(skip_serializing_if = "Option::is_none")] #typs, )* ),
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

    let (patterns, bodies) = parse_evariants(evariants, diff);

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

    evariants.iter().for_each(|var| {
        let vname = &var.name;
        let vfields = &var.fields;
        let struct_type = var.variant.clone();

        let (mlp, mrp, mb) = parse_merge(vname, vfields, struct_type.clone());
        let (dlp, drp, db) = parse_diff(vname, vfields, struct_type.clone());

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

fn parse_evariants(evariants: &[EVariant], diff: &Ident) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut patterns: Vec<TokenStream> = vec![];
    let mut bodies: Vec<TokenStream> = vec![];

    evariants
        .iter()
        .for_each(|var| match (var.variant.clone(), &var.name, &var.fields) {
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
                                #buf.field(stringify!(#fname), &#fname);
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

                        if let Some(field) = &field_0 {
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
        });

    (patterns.to_vec(), bodies.to_vec())
}

fn parse_merge(
    vname: &Ident,
    vfields: &[DataFields],
    struct_type: SVariant,
) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
    let mut merge_rpatterns: Vec<TokenStream> = vec![];
    let mut merge_lpatterns: Vec<TokenStream> = vec![];
    let mut merge_bodies: Vec<TokenStream> = vec![];

    match struct_type {
        SVariant::Named => {
            let fnames: Vec<&Ident> = vfields.iter().map(|f| f.name()).collect();

            let (left_names, right_names) = populate_field_names(vfields, 0, struct_type);

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

            (
                merge_lpatterns.to_vec(),
                merge_rpatterns.to_vec(),
                merge_bodies.to_vec(),
            )
        }
        SVariant::Tuple => {
            let ftyps: Vec<&Type> = vfields.iter().map(|f| f.typ()).collect();

            let fmax = ftyps.len();

            let (left_names, right_names) = populate_field_names(vfields, fmax, struct_type);

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
            (
                merge_lpatterns.to_vec(),
                merge_rpatterns.to_vec(),
                merge_bodies.to_vec(),
            )
        }
        SVariant::Unit => {
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

            (
                merge_lpatterns.to_vec(),
                merge_rpatterns.to_vec(),
                merge_bodies.to_vec(),
            )
        }
    }
}

fn parse_diff(
    vname: &Ident,
    vfields: &[DataFields],

    struct_type: SVariant,
) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
    let mut diff_rpatterns: Vec<TokenStream> = vec![];
    let mut diff_lpatterns: Vec<TokenStream> = vec![];
    let mut diff_bodies: Vec<TokenStream> = vec![];

    match struct_type {
        SVariant::Named => {
            let fnames: Vec<&Ident> = vfields.iter().map(|f| f.name()).collect();

            let (left_names, right_names) = populate_field_names(vfields, 0, struct_type);

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
                                Some(#ln.diff(#rn))
                            } else {
                                None
                            }
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
        SVariant::Tuple => {
            let ftyps: Vec<&Type> = vfields.iter().map(|f| f.typ()).collect();

            let fmax = ftyps.len();

            let (left_names, right_names) = populate_field_names(vfields, fmax, struct_type);

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
                                Some(#ln.diff(#rn))
                            } else {
                                None
                            }
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

            diff_lpatterns.push(quote! {
                Self::#vname( #(#left_names),* )
            });

            diff_rpatterns.push(quote! {
                Self::#vname( #(#right_names),* )
            });

            diff_bodies.push(quote! {
                Self::Type::#vname( #(#diff_fvalues),* )
            });

            diff_lpatterns.push(quote! {_});

            diff_rpatterns.push(quote! { other @ Self::#vname(..) });
            diff_bodies.push(quote! {
                other.clone().into_diff()
            });
        }
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
                Self::Type::#vname
            });

            diff_bodies.push(quote! {
                other.clone().into_diff()
            });
        }
    }
    (diff_lpatterns.to_vec(), diff_rpatterns.to_vec(), diff_bodies.to_vec())
}

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
        SVariant::Named => {
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
                        quote! { #fname: Option::None }
                    } else if f.is_option() {
                        quote! {
                            #fname: if let identity_diff::option::DiffOption::Some(_) = #fname.clone().into_diff() {
                                Some(#fname.into_diff())
                            } else {
                                None
                            }
                        }
                    } else {
                        quote! {
                            #fname: Some(#fname.into_diff())
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
        }
        SVariant::Tuple => {
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
                .enumerate()
                .map(|(idx, f)| {
                    let fname = &fnames[idx];

                    if f.should_ignore() {
                        quote! { Option::None }
                    } else if f.is_option() {
                        quote! {
                            if #fname.clone().into_diff() == identity_diff::option::DiffOption::None {
                                None
                            } else {
                                Some(#fname.into_diff())
                            }
                        }
                    } else {
                        quote! {
                            Some(#fname.into_diff())
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
        }
        SVariant::Unit => {
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
        }
    }

    (from_body.to_vec(), into_body.to_vec())
}

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
