use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, Attribute, Data, DeriveInput, Field, Ident, Token};

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
    // create an iterator over (field_name, Resolver::I, Resolver::T)
    .flat_map(|ResolverField { ident, impls }| {
      impls
        .into_iter()
        .map(move |(input_ty, target_ty)| (ident.clone(), input_ty, target_ty))
    })
    // generates code that forward the implementation of Resolver<T, I> to field_name.
    .map(|(field_name, input_ty, target_ty)| {
      quote! {
        impl ::identity_new_resolver::Resolver<#target_ty, #input_ty> for #struct_ident #generics {
          async fn resolve(&self, input: &#input_ty) -> std::result::Result<#target_ty, ::identity_new_resolver::Error> {
            self.#field_name.resolve(input).await
          }
        }
      }
    })
    .collect::<proc_macro2::TokenStream>()
    .into()
}

/// A field annotated with `#[resolver(Input -> Target, ..)]`
struct ResolverField {
  ident: Ident,
  impls: Vec<(Ident, Ident)>,
}

impl ResolverField {
  pub fn from_field(field: Field) -> Option<Self> {
    let Field { attrs, ident, .. } = field;
    let Some(ident) = ident else {
      panic!("Derive macro \"CompoundResolver\" only works on struct with named fields");
    };

    let impls: Vec<(Ident, Ident)> = attrs
      .into_iter()
      .flat_map(|attr| parse_resolver_attribute(attr).into_iter())
      .collect();

    if !impls.is_empty() {
      Some(ResolverField { ident, impls })
    } else {
      None
    }
  }
}

fn parse_resolver_attribute(attr: Attribute) -> Vec<(Ident, Ident)> {
  if attr.path().is_ident("resolver") {
    attr
      .parse_args_with(Punctuated::<ResolverTy, Token![,]>::parse_terminated)
      .expect("invalid resolver annotation")
      .into_iter()
      .map(Into::into)
      .collect()
  } else {
    vec![]
  }
}

struct ResolverTy {
  pub input: Ident,
  pub target: Ident,
}

impl From<ResolverTy> for (Ident, Ident) {
  fn from(value: ResolverTy) -> Self {
    (value.input, value.target)
  }
}

impl Parse for ResolverTy {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut tys = Punctuated::<Ident, Token![->]>::parse_separated_nonempty(input)?
      .into_iter()
      .take(2);

    Ok({
      ResolverTy {
        input: tys.next().unwrap(),
        target: tys.next().unwrap(),
      }
    })
  }
}
