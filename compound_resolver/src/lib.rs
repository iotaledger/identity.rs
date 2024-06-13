use itertools::Itertools;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::Attribute;
use syn::Data;
use syn::DeriveInput;
use syn::Expr;
use syn::Field;
use syn::Ident;
use syn::Token;
use syn::Type;

#[proc_macro_derive(CompoundResolver, attributes(resolver))]
pub fn derive_macro_compound_resolver(input: TokenStream) -> TokenStream {
  let DeriveInput {
    ident: struct_ident,
    data,
    generics,
    ..
  } = syn::parse_macro_input!(input);

  let Data::Struct(data) = data else {
    panic!("Derive macro \"CompoundResolver\" only works on structs");
  };

  data
    .fields
    .into_iter()
    // parse all the fields that are annoted with #[resolver(..)]
    .filter_map(ResolverField::from_field)
    // Group together all resolvers with the same signature (input_ty, target_ty).
    .flat_map(|ResolverField { ident, impls }| {
      impls
        .into_iter()
        .map(move |ResolverImpl { input, target, pred }| ((input, target), (ident.clone(), pred)))
    })
    .into_group_map()
    .into_iter()
    // generates code that forward the implementation of Resolver<T, I> to field_name, if there's multiple fields
    // implementing that trait, use `pred` to decide which one to call.
    .map(|((input_ty, target_ty), impls)| {
      let len = impls.len();
      let impl_block = gen_impl_block_multiple_resolvers(impls.into_iter(), len);
      quote! {
        impl ::identity_iota::resolver::Resolver<#input_ty> for #struct_ident #generics {
          type Target = #target_ty;
          async fn resolve(&self, input: &#input_ty) -> std::result::Result<Self::Target, ::identity_iota::resolver::Error> {
            #impl_block
          }
        }
      }
    })
    .collect::<proc_macro2::TokenStream>()
    .into()
}

fn gen_impl_block_single_resolver(field_name: Ident) -> proc_macro2::TokenStream {
  quote! {
    self.#field_name.resolve(input).await
  }
}

fn gen_impl_block_single_resolver_with_pred(field_name: Ident, pred: Expr) -> proc_macro2::TokenStream {
  let invocation_block = gen_impl_block_single_resolver(field_name);
  quote! {
    if #pred { return #invocation_block }
  }
}

fn gen_impl_block_multiple_resolvers(
  impls: impl Iterator<Item = (Ident, Option<Expr>)>,
  len: usize,
) -> proc_macro2::TokenStream {
  impls
    .enumerate()
    .map(|(i, (field_name, pred))| {
      if let Some(pred) = pred {
        gen_impl_block_single_resolver_with_pred(field_name, pred)
      } else if i == len - 1 {
        gen_impl_block_single_resolver(field_name)
      } else {
        panic!("Multiple resolvers with the same signature. Expected predicate");
      }
    })
    .collect()
}

/// A field annotated with `#[resolver(Input -> Target, ..)]`
struct ResolverField {
  ident: Ident,
  impls: Vec<ResolverImpl>,
}

impl ResolverField {
  pub fn from_field(field: Field) -> Option<Self> {
    let Field { attrs, ident, .. } = field;
    let Some(ident) = ident else {
      panic!("Derive macro \"CompoundResolver\" only works on struct with named fields");
    };

    let impls = attrs
      .into_iter()
      .flat_map(|attr| parse_resolver_attribute(attr).into_iter())
      .collect::<Vec<_>>();

    if !impls.is_empty() {
      Some(ResolverField { ident, impls })
    } else {
      None
    }
  }
}

fn parse_resolver_attribute(attr: Attribute) -> Vec<ResolverImpl> {
  if attr.path().is_ident("resolver") {
    attr
      .parse_args_with(Punctuated::<ResolverImpl, Token![,]>::parse_terminated)
      .expect("invalid resolver annotation")
      .into_iter()
      .collect()
  } else {
    vec![]
  }
}

struct ResolverImpl {
  pub input: Type,
  pub target: Type,
  pub pred: Option<Expr>,
}

impl Parse for ResolverImpl {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let input_ty = input.parse::<Type>()?;
    let _ = input.parse::<Token![->]>()?;
    let target_ty = input.parse::<Type>()?;
    let pred = if input.peek(Token![if]) {
      let _ = input.parse::<Token![if]>()?;
      Some(input.parse::<Expr>()?)
    } else {
      None
    };

    Ok({
      ResolverImpl {
        input: input_ty,
        target: target_ty,
        pred,
      }
    })
  }
}

#[test]
fn test_parse_resolver_attribute() {
  syn::parse_str::<ResolverImpl>("DidKey -> CoreDoc").unwrap();
  syn::parse_str::<ResolverImpl>("DidKey -> Vec<u8>").unwrap();
  syn::parse_str::<ResolverImpl>("Vec<u8> -> &str").unwrap();
  syn::parse_str::<ResolverImpl>("DidIota -> IotaDoc if input.method_id() == \"iota\"").unwrap();
}
