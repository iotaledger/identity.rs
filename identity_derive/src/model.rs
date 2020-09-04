#![allow(unused)]

use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, token::Comma, Data, DataEnum, DataStruct, DeriveInput, Fields, GenericParam, Token, Type,
    Variant, WhereClause,
};

#[derive(Clone, Debug)]
pub enum InputModel {
    Enum(InputEnum),
    Struct(InputStruct),
}

#[derive(Clone, Debug)]
pub struct InputEnum {
    name: Ident,
    diff: Ident,
    variants: Vec<EVariant>,
    param_decl: Punctuated<GenericParam, Comma>,
    params: Punctuated<Ident, Comma>,
    clause: WhereClause,
}

#[derive(Clone, Debug)]
pub struct InputStruct {
    variant: SVariant,
    name: Ident,
    diff: Ident,
    fields: Vec<DataFields>,
    param_decl: Punctuated<GenericParam, Comma>,
    params: Punctuated<Ident, Comma>,
    clause: WhereClause,
}

#[derive(Clone, Debug)]
pub struct EVariant {
    variant: SVariant,
    name: Ident,
    fields: Vec<DataFields>,
}

#[derive(Clone, Debug)]
pub enum SVariant {
    Named,
    Tuple,
    Unit,
}

#[derive(Clone, Debug)]
pub enum DataFields {
    Named {
        name: Ident,
        typ: Type,
        should_ignore: bool,
    },
    Unnamed {
        position: Literal,
        typ: Type,
        should_ignore: bool,
    },
}

impl InputModel {
    pub fn parse(input: &DeriveInput) -> Self {
        match &input.data {
            Data::Struct(DataStruct { fields, .. }) if !fields.is_empty() => Self::parse_struct(input, fields),
            Data::Struct(DataStruct { .. }) => Self::parse_unit(input),
            Data::Enum(DataEnum { variants, .. }) => Self::parse_enum(input, variants),
            _ => panic!("Data Type not supported"),
        }
    }

    fn parse_struct(input: &DeriveInput, fields: &Fields) -> Self {
        Self::Struct(InputStruct::parse(input, fields))
    }

    fn parse_unit(input: &DeriveInput) -> Self {
        Self::Struct(InputStruct::parse_unit(input))
    }

    fn parse_enum(input: &DeriveInput, variants: &Punctuated<Variant, Comma>) -> Self {
        Self::Enum(InputEnum::parse(input, variants))
    }
}

impl InputEnum {
    pub fn new(input: &DeriveInput) -> Self {
        Self {
            name: input.ident.clone(),
            diff: format_ident!("Diff{}", &input.ident),
            variants: Vec::new(),
            param_decl: input.generics.params.clone(),
            params: input
                .generics
                .type_params()
                .map(|type_param| type_param.ident.clone())
                .collect(),
            clause: input.generics.where_clause.clone().unwrap_or_else(|| WhereClause {
                where_token: Token![where](Span::call_site()),
                predicates: Punctuated::new(),
            }),
        }
    }

    fn parse(input: &DeriveInput, variants: &Punctuated<Variant, Comma>) -> Self {
        let mut model = Self::new(input);
        variants.iter().for_each(|vars| {
            let mut variant = EVariant::new(&vars.ident);

            vars.fields.iter().enumerate().for_each(|(idx, vs)| {
                if let Some(ident) = vs.ident.as_ref() {
                    variant.variant = SVariant::Named;
                    variant.fields.push(DataFields::Named {
                        name: ident.clone(),
                        typ: vs.ty.clone(),
                        should_ignore: false,
                    });
                } else {
                    variant.variant = SVariant::Tuple;
                    variant.fields.push(DataFields::Unnamed {
                        position: Literal::usize_unsuffixed(idx),
                        typ: vs.ty.clone(),
                        should_ignore: false,
                    });
                }
            });
            model.variants.push(variant);
        });

        model
    }
}

impl InputStruct {
    pub fn new(input: &DeriveInput) -> Self {
        Self {
            variant: SVariant::Unit,
            name: input.ident.clone(),
            diff: format_ident!("Diff{}", &input.ident),
            fields: Vec::new(),
            param_decl: input.generics.params.clone(),
            params: input.generics.type_params().map(|tp| tp.ident.clone()).collect(),
            clause: input.generics.where_clause.clone().unwrap_or_else(|| WhereClause {
                where_token: Token![where](Span::call_site()),
                predicates: Punctuated::new(),
            }),
        }
    }

    fn parse(input: &DeriveInput, fields: &Fields) -> Self {
        let mut model = Self::new(input);
        fields.iter().enumerate().for_each(|(idx, fs)| {
            if let Some(ident) = fs.ident.as_ref() {
                model.variant = SVariant::Named;
                model.fields.push(DataFields::Named {
                    name: ident.clone(),
                    typ: fs.ty.clone(),
                    should_ignore: false,
                });
            } else {
                model.variant = SVariant::Tuple;
                model.fields.push(DataFields::Unnamed {
                    position: Literal::usize_unsuffixed(idx),
                    typ: fs.ty.clone(),
                    should_ignore: false,
                });
            }
        });

        model
    }

    fn parse_unit(input: &DeriveInput) -> Self {
        let mut model = Self::new(input);
        model.variant = SVariant::Unit;

        model
    }
}

impl EVariant {
    pub fn new(name: &Ident) -> Self {
        Self {
            variant: SVariant::Unit,
            name: name.clone(),
            fields: Vec::new(),
        }
    }
}
