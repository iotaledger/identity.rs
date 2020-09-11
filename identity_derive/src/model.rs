use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, token::Comma, Data, DataEnum, DataStruct, DeriveInput, Fields, GenericParam, Token, Type,
    Variant, WhereClause,
};

use crate::{
    impls::{debug_impl, derive_diff_enum, derive_diff_struct, diff_impl, impl_debug_enum, impl_diff_enum},
    should_ignore,
};

#[derive(Clone, Debug)]
pub enum InputModel {
    Enum(InputEnum),
    Struct(InputStruct),
}

#[derive(Clone, Debug)]
pub struct InputEnum {
    pub name: Ident,
    pub diff: Ident,
    pub variants: Vec<EVariant>,
    pub param_decls: Punctuated<GenericParam, Comma>,
    pub params: Punctuated<Ident, Comma>,
    pub clause: WhereClause,
}

#[derive(Clone, Debug)]
pub struct InputStruct {
    pub variant: SVariant,
    pub name: Ident,
    pub diff: Ident,
    pub fields: Vec<DataFields>,
    pub param_decls: Punctuated<GenericParam, Comma>,
    pub params: Punctuated<Ident, Comma>,
    pub clause: WhereClause,
}

#[derive(Clone, Debug)]
pub struct EVariant {
    pub variant: SVariant,
    pub name: Ident,
    pub fields: Vec<DataFields>,
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

    pub fn s_variant(&self) -> &SVariant {
        match self {
            Self::Enum(InputEnum { name, .. }) => panic!("{} isn't a struct", name),
            Self::Struct(InputStruct { variant, .. }) => variant,
        }
    }

    pub fn e_variants(&self) -> &Vec<EVariant> {
        match self {
            Self::Enum(InputEnum { variants, .. }) => variants,
            Self::Struct(InputStruct { name, .. }) => panic!("{} isn't an Enum", name),
        }
    }

    pub fn name(&self) -> &Ident {
        match self {
            Self::Enum(InputEnum { name, .. }) => name,
            Self::Struct(InputStruct { name, .. }) => name,
        }
    }

    pub fn diff(&self) -> &Ident {
        match self {
            Self::Enum(InputEnum { diff, .. }) => diff,
            Self::Struct(InputStruct { diff, .. }) => diff,
        }
    }

    pub fn params(&self) -> &Punctuated<Ident, Comma> {
        match self {
            Self::Enum(InputEnum { params, .. }) => params,
            Self::Struct(InputStruct { params, .. }) => params,
        }
    }

    pub fn param_decls(&self) -> &Punctuated<GenericParam, Comma> {
        match self {
            Self::Enum(InputEnum { param_decls, .. }) => param_decls,
            Self::Struct(InputStruct { param_decls, .. }) => param_decls,
        }
    }

    pub fn fields(&self) -> &Vec<DataFields> {
        match self {
            Self::Enum(InputEnum { name, .. }) => panic!("{} isn't a Struct", name),
            Self::Struct(InputStruct { fields, .. }) => fields,
        }
    }

    pub fn clause(&self) -> &WhereClause {
        match self {
            Self::Enum(InputEnum { clause, .. }) => clause,
            Self::Struct(InputStruct { clause, .. }) => clause,
        }
    }

    pub fn impl_debug(&self) -> TokenStream {
        match self {
            Self::Struct(InputStruct { .. }) => debug_impl(self),
            Self::Enum(InputEnum { .. }) => impl_debug_enum(self),
        }
    }

    pub fn impl_diff(&self) -> TokenStream {
        match self {
            Self::Struct(InputStruct { .. }) => diff_impl(self),
            Self::Enum(InputEnum { .. }) => impl_diff_enum(self),
        }
    }

    pub fn derive_diff(&self) -> TokenStream {
        match self {
            Self::Struct(InputStruct { .. }) => derive_diff_struct(self),
            Self::Enum(InputEnum { .. }) => derive_diff_enum(self),
        }
    }
}

impl InputEnum {
    pub fn new(input: &DeriveInput) -> Self {
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
        }
    }

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
    pub fn new(input: &DeriveInput) -> Self {
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

impl DataFields {
    pub fn name(&self) -> &Ident {
        match self {
            Self::Named { name, .. } => name,
            Self::Unnamed { .. } => panic!("Positional Field has no name"),
        }
    }

    pub fn position(&self) -> &Literal {
        match self {
            Self::Named { .. } => panic!("Named fields has no position"),
            Self::Unnamed { position, .. } => position,
        }
    }

    pub fn typ(&self) -> &Type {
        match self {
            Self::Named { typ, .. } => typ,
            Self::Unnamed { typ, .. } => typ,
        }
    }

    pub fn typ_as_tokens(&self) -> TokenStream {
        let typ = self.typ();

        if self.should_ignore() {
            quote! {Option<#typ>}
        } else {
            quote! { Option<<#typ as identity_diff::Diff>::Type> }
        }
    }

    pub fn should_ignore(&self) -> bool {
        match self {
            Self::Named { should_ignore, .. } => *should_ignore,
            Self::Unnamed { should_ignore, .. } => *should_ignore,
        }
    }
}
