#![no_std]
extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;
use wasm_error::derive::WasmError;
use wasm_error::IntoWasmError;
use core::fmt::Formatter;
use crate::alloc::string::ToString;

#[derive(Debug, WasmError)]
pub enum GenericEnum<T>
where
  T: core::fmt::Debug,
{
  A,
  B(T),
  C { a: T },
  D { a: T, b: T },
}

impl<T> core::fmt::Display for GenericEnum<T> where
  T: core::fmt::Debug,{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_fmt(format_args!("{:?}", self))
  }
}

#[derive(Debug, WasmError)]
pub enum TwoGenericsEnum<T, V>
where
  T: core::fmt::Debug,
  V: core::fmt::Debug,
{
  A,
  B(T),
  C(V),
  D(T, V),
  E { a: T },
  F { a: V },
  G { a: T, b: V },
}

impl<T, V> core::fmt::Display for TwoGenericsEnum<T, V> where
  T: core::fmt::Debug,
  V: core::fmt::Debug,{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_fmt(format_args!("{:?}", self))
  }
}

#[test]
fn test_wasm_error_generic_basic() {
  let a = WasmGenericEnum {
    code: WasmGenericEnumCode::A,
    description: "A".to_owned(),
  };
  assert_eq!(a.code, WasmGenericEnumCode::A);
  assert_eq!(a.description, "A".to_owned());
}

#[test]
fn test_wasm_error_two_generics_basic() {
  let a = WasmTwoGenericsEnum {
    code: WasmTwoGenericsEnumCode::A,
    description: "A".to_owned(),
  };
  assert_eq!(a.code, WasmTwoGenericsEnumCode::A);
  assert_eq!(a.description, "A".to_owned());
}

#[test]
fn test_wasm_error_two_generics() {
  let g = WasmTwoGenericsEnum::from(TwoGenericsEnum::G::<u8, i64> { a: 42, b: 123 });
  assert_eq!(g.code, WasmTwoGenericsEnumCode::G);
  assert_eq!(g.description, "G { a: 42, b: 123 }".to_owned());
}

#[test]
fn test_two_generics_to_wasm_error() {
  let g = TwoGenericsEnum::G::<u8, i64> { a: 42, b: 123 }.into_wasm_error();
  assert_eq!(g.code, WasmTwoGenericsEnumCode::G);
  assert_eq!(g.description, "G { a: 42, b: 123 }".to_owned());
}
