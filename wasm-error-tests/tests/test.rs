#![no_std]
extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;
use wasm_error::IntoWasmError;
use core::fmt::Formatter;
use alloc::string::ToString;

#[derive(Debug, wasm_error::derive::WasmError)]
pub enum BasicEnum {
  A,
  B(String),
  C { a: String },
  D { a: String, b: i64 },
}

impl core::fmt::Display for BasicEnum {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_fmt(format_args!("{:?}", self))
  }
}

#[test]
fn test_wasm_error_basic() {
  let a = WasmBasicEnum {
    code: WasmBasicEnumCode::A,
    description: "A".to_owned(),
  };
  assert_eq!(a.code, WasmBasicEnumCode::A);
  assert_eq!(a.description, "A");
}

#[test]
fn test_wasm_error_from() {
  let a = WasmBasicEnum::from(BasicEnum::A);
  assert_eq!(a.code, WasmBasicEnumCode::A);
  assert_eq!(a.description, "A");

  let b = WasmBasicEnum::from(BasicEnum::B("bee".to_owned()));
  assert_eq!(b.code, WasmBasicEnumCode::B);
  assert_eq!(b.description, r#"B("bee")"#);

  let c = WasmBasicEnum::from(BasicEnum::C { a: "c".to_owned() });
  assert_eq!(c.code, WasmBasicEnumCode::C);
  assert_eq!(c.description, r#"C { a: "c" }"#);

  let d = WasmBasicEnum::from(BasicEnum::D {
    a: "d".to_owned(),
    b: 123,
  });
  assert_eq!(d.code, WasmBasicEnumCode::D);
  assert_eq!(d.description, r#"D { a: "d", b: 123 }"#);
}

#[test]
fn test_to_wasm_error() {
  let a = BasicEnum::A.into_wasm_error();
  assert_eq!(a.code, WasmBasicEnumCode::A);
  assert_eq!(a.description, "A");

  let b = BasicEnum::B("bee".to_owned()).into_wasm_error();
  assert_eq!(b.code, WasmBasicEnumCode::B);
  assert_eq!(b.description, r#"B("bee")"#);

  let c = BasicEnum::C { a: "c".to_owned() }.into_wasm_error();
  assert_eq!(c.code, WasmBasicEnumCode::C);
  assert_eq!(c.description, r#"C { a: "c" }"#);

  let d = BasicEnum::D {
    a: "d".to_owned(),
    b: 123,
  }
  .into_wasm_error();
  assert_eq!(d.code, WasmBasicEnumCode::D);
  assert_eq!(d.description, r#"D { a: "d", b: 123 }"#);
}
