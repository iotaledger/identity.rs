extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;
//
// #[proc_macro_derive(FlatError)]
// pub fn flat_error(input: TokenStream) -> TokenStream {
//   // See https://doc.servo.org/syn/derive/struct.DeriveInput.html
//   let ast: DeriveInput = syn::parse_macro_input!(input as DeriveInput);
//
//   // Build the struct
//   let gen = gen_flat_error_struct(&ast);
//
//   // Return the generated impl
//   gen.parse().unwrap()
// }
//
// fn gen_flat_error_struct(ast: &syn::DeriveInput) -> quote::Tokens {
//   // get enum name
//   let name = &ast.ident;
//   let data = &ast.data;
//   quote! {
//       // TODO
//     }
// }
