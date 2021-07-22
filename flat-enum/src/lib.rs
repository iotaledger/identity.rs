use proc_macro::TokenStream;

use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields};

// TODO: conditionally add wasm-bindgen when the flat_wasm_bindgen attribute is specified
#[proc_macro_derive(FlatEnum, attributes(flat_wasm_bindgen))]
#[proc_macro_error::proc_macro_error]
pub fn derive_flat_enum(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse_macro_input!(input as DeriveInput);

  // Build the flat error struct and enum
  gen_flat_enum(&ast)
}

fn gen_flat_enum(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let data = &ast.data;

  let flat_struct_name = quote::format_ident!("Flat{}", name.to_string());
  let flat_enum_code_name = quote::format_ident!("Flat{}Code", name.to_string());

  let mut flat_enum_code_variants;
  let mut flat_enum_code_from_cases;

  match data {
    Data::Enum(data_enum) => {
      flat_enum_code_variants = TokenStream2::new();
      flat_enum_code_from_cases = TokenStream2::new();

      // Iterate over enum variants
      for variant in &data_enum.variants {
        let variant_name = &variant.ident;

        // Enum variants can:
        // - have unnamed fields like `Variant(i32, i64)`
        // - have named fields like `Variant {x: i32, y: i32}`
        // - be a Unit like `Variant`
        let fields_in_variant = match &variant.fields {
          Fields::Unnamed(_) => quote::quote_spanned! { variant.span()=> (..) },
          Fields::Unit => quote::quote_spanned! { variant.span()=> },
          Fields::Named(_) => quote::quote_spanned! { variant.span()=> {..} },
        };

        // Generate a flat C-style enum variant
        let variant_doc_string = format!("[{}::{}]", name.to_string(), variant_name.to_string());
        flat_enum_code_variants.extend(quote::quote_spanned! {variant.span()=>
          #[doc = #variant_doc_string]
          #variant_name,
        });

        // Implement From<OriginalEnum> cases
        // TODO: improve description strings, currently requires Debug to be implemented on the
        //       original enum which includes the name in the string.
        //       e.g. Enum::A => "A" - do we want "" instead?
        //       e.g. Enum::B(1,2) => "B(1,2)" - do we want "(1,2)" instead?
        //       e.g. Enum::C{"a","b"} => "C { "a", "b" }" - do we want "{ "a", "b" }" instead?
        flat_enum_code_from_cases.extend(quote::quote_spanned! {variant.span()=>
          #name::#variant_name #fields_in_variant => #flat_struct_name::new(#flat_enum_code_name::#variant_name, format!("{:?}", input)),
        });
      }
    }
    _ => proc_macro_error::abort_call_site!("FlatError is only supported for enums"),
  };

  // Generate Flat* struct and Flat*Code enum
  let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
  let expanded = quote::quote! {
    // Serializable wrapper struct
    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[repr(C)]
    pub struct #flat_struct_name {
      pub code: #flat_enum_code_name,
      pub description: String,
    }

    impl #flat_struct_name {
      pub fn new(code: #flat_enum_code_name, description: String) -> Self {
        Self {
          code,
          description,
        }
      }
    }

    // C-style enum
    #[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    #[repr(C)]
    pub enum #flat_enum_code_name {
      #flat_enum_code_variants
    }

    // From implementation
    impl #impl_generics From<#name #type_generics> for #flat_struct_name #where_clause {
      fn from(input: #name #type_generics) -> Self {
        match &input {
          #flat_enum_code_from_cases
        }
      }
    }

    // Add to_flat_enum() function to the original enum
    impl #impl_generics #name #type_generics #where_clause {
      pub fn to_flat_enum(self) -> #flat_struct_name {
        #flat_struct_name::from(self)
      }
    }
  };

  TokenStream::from(expanded)
}
