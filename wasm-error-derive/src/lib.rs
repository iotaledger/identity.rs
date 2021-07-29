use proc_macro::TokenStream;

use syn::{Data, DeriveInput, Fields};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;

#[proc_macro_derive(WasmError)]
#[proc_macro_error::proc_macro_error]
pub fn derive_wasm_error(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse_macro_input!(input as DeriveInput);

  // Build the wasm error struct and enum
  gen_wasm_error(&ast)
}

fn gen_wasm_error(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let data = &ast.data;

  let wasm_struct_name = quote::format_ident!("Wasm{}", name.to_string());
  let wasm_error_code_name = quote::format_ident!("Wasm{}Code", name.to_string());
  let wasm_struct_js_name = format!("{}", name.to_string());
  let wasm_error_code_js_name = format!("{}Code", name.to_string());

  let mut wasm_error_code_variants;
  let mut wasm_error_code_from_cases;

  match data {
    Data::Enum(data_enum) => {
      wasm_error_code_variants = TokenStream2::new();
      wasm_error_code_from_cases = TokenStream2::new();

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

        // Generate a wasm C-style enum variant
        let variant_doc_string = format!("[{}::{}]", name.to_string(), variant_name.to_string());
        wasm_error_code_variants.extend(quote::quote_spanned! {variant.span()=>
          #[doc = #variant_doc_string]
          #variant_name,
        });

        // Implement From<OriginalEnum> cases
        // TODO: improve description strings, currently requires Debug to be implemented on the
        //       original enum which includes the name in the string.
        //       e.g. Enum::A => "A" - do we want "" instead?
        //       e.g. Enum::B(1,2) => "B(1,2)" - do we want "(1,2)" instead?
        //       e.g. Enum::C{"a","b"} => "C { "a", "b" }" - do we want "{ "a", "b" }" instead?
        wasm_error_code_from_cases.extend(quote::quote_spanned! {variant.span()=>
          #name::#variant_name #fields_in_variant => #wasm_struct_name::new(#wasm_error_code_name::#variant_name, input.to_string()),
        });
      }
    }
    _ => proc_macro_error::abort_call_site!("WasmError is only supported for enums"),
  };

  // Generate Wasm* struct and Wasm*Code enum
  let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
  let expanded = quote::quote! {
    use wasm_bindgen::prelude::*;
    // Serializable wrapper struct
    #[wasm_bindgen(js_name=#wasm_struct_js_name)]
    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct #wasm_struct_name {
      code: #wasm_error_code_name,
      description: String,
    }

    #[wasm_bindgen]
    impl #wasm_struct_name {
      #[wasm_bindgen(constructor)]
      pub fn new(code: #wasm_error_code_name, description: String) -> Self {
        Self {
          code,
          description,
        }
      }

      #[wasm_bindgen(getter)]
      pub fn code(&self) -> #wasm_error_code_name {
        self.code
      }

      #[wasm_bindgen(getter)]
      pub fn description(&self) -> String {
        self.description.clone()
      }
    }

    // WasmError trait
    impl wasm_error::WasmError for #wasm_struct_name {}

    // C-style enum
    #[wasm_bindgen(js_name=#wasm_error_code_js_name)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    pub enum #wasm_error_code_name {
      #wasm_error_code_variants
    }

    // From implementation
    impl #impl_generics From<#name #type_generics> for #wasm_struct_name #where_clause {
      fn from(input: #name #type_generics) -> Self {
        match &input {
          #wasm_error_code_from_cases
        }
      }
    }

    // Implement IntoWasmError for the original enum, to use into_wasm_error()
    impl #impl_generics wasm_error::IntoWasmError<#wasm_struct_name> for #name #type_generics #where_clause {
      fn into_wasm_error(self) -> #wasm_struct_name {
        #wasm_struct_name::from(self)
      }
    }

    // Workaround since cannot implement From<T> for JsValue where T: IntoWasmError<dyn WasmError>
    // nor T: IntoWasmError<#wasm_struct_name>
    impl #impl_generics From<#name #type_generics> for wasm_bindgen::JsValue #where_clause
    {
      fn from(input: #name #type_generics) -> Self {
        use wasm_error::IntoWasmError;
        // TODO: check that unwrap always works?
        wasm_bindgen::JsValue::from_serde(&input.into_wasm_error()).unwrap()
      }
    }
  };

  TokenStream::from(expanded)
}
