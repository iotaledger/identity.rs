use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Type};

struct HandlerGenerator {
  /// The identifier of the struct we're generating code for
  ident: Ident,
  /// The identifiers of the struct's fields
  field_idents: Vec<Ident>,
  /// The types of the struct's fields
  field_types: Vec<Type>,
}

impl HandlerGenerator {
  fn parse_fields(derive_input: DeriveInput) -> Self {
    if let syn::Data::Struct(data_struct) = derive_input.data {
      let mut field_idents = Vec::with_capacity(data_struct.fields.len());
      let mut field_types = Vec::with_capacity(data_struct.fields.len());

      for field in data_struct.fields {
        field_idents.push(field.ident.expect("macro cannot be applied to tuple structs"));
        field_types.push(field.ty);
      }

      Self {
        ident: derive_input.ident,
        field_idents,
        field_types,
      }
    } else {
      panic!("macro can only be applied to structs")
    }
  }

  fn generate_request_wrapper(&self) -> proc_macro2::TokenStream {
    let variants = &self.field_idents;
    let types = &self.field_types;

    quote! {
      #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, identity_actor::RequestPermissions)]
      #[doc(hidden)]
      #[automatically_derived]
      pub enum __RequestWrapper {
        #(#variants(<#types as identity_actor::IdentityRequestHandler>::Request),)*
      }
    }
  }

  fn generate_response_wrapper(&self) -> proc_macro2::TokenStream {
    let variants = &self.field_idents;
    let types = &self.field_types;

    quote! {
      #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, identity_actor::RequestPermissions)]
      #[doc(hidden)]
      #[automatically_derived]
      pub enum __ResponseWrapper {
        #( #variants(<#types as identity_actor::IdentityRequestHandler>::Response),)*
      }
    }
  }

  fn impl_handler_trait(&self) -> proc_macro2::TokenStream {
    let ident = &self.ident;
    let variants = &self.field_idents;

    quote! {
      impl identity_actor::IdentityRequestHandler for #ident {
        type Request = __RequestWrapper;
        type Response = __ResponseWrapper;

        fn handle(&mut self, request: Self::Request) -> Self::Response {
          match request {
            #(
              Self::Request::#variants(request) => Self::Response::#variants(self.#variants.handle(request)),
            )*
          }
        }
      }
    }
  }

  fn generate_from_impls(&self) -> proc_macro2::TokenStream {
    let variants = &self.field_idents;
    let types = &self.field_types;

    quote! {
      #(
        impl From<<#types as identity_actor::IdentityRequestHandler>::Request> for __RequestWrapper {
          fn from(req: <#types as identity_actor::IdentityRequestHandler>::Request) -> Self {
            Self::#variants(req)
          }
        }
      )*
    }
  }

  fn generate(&self) -> proc_macro2::TokenStream {
    let request_wrapper_enum = self.generate_request_wrapper();
    let response_wrapper_enum = self.generate_response_wrapper();
    let from_impls = self.generate_from_impls();
    let handler_trait = self.impl_handler_trait();

    let ident = &self.ident;

    quote! {
      #[allow(non_camel_case_types)]
      mod __generated {
        use super::*;
        #request_wrapper_enum

        #response_wrapper_enum

        #from_impls

        #handler_trait
      }

      pub type CustomIdentityCommunicator = identity_actor::IdentityCommunicator<
        __generated::__RequestWrapper,
        __generated::__ResponseWrapper,
        __generated::__RequestWrapperPermission,
        #ident
      >;

    }
  }
}

#[proc_macro_derive(IdentityHandler)]
pub fn derive(input: TokenStream) -> TokenStream {
  let derive: DeriveInput = parse_macro_input!(input);

  let gen = HandlerGenerator::parse_fields(derive);

  gen.generate().into()
}
