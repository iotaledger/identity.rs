#![no_std]
extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use flat_enum::FlatEnum;

#[derive(Debug, FlatEnum)]
pub enum Enum {
  A,
  B(String),
  C { a: String },
  D { a: String, b: i64 },
}

#[test]
fn test_flat_enum_basic() {
  let a = FlatEnum {
    code: FlatEnumCode::A,
    description: "A".to_owned(),
  };
  assert_eq!(a.code, FlatEnumCode::A);
  assert_eq!(a.description, "A");
}

#[test]
fn test_flat_enum_from() {
  let a = FlatEnum::from(Enum::A);
  assert_eq!(a.code, FlatEnumCode::A);
  assert_eq!(a.description, "A");

  let b = FlatEnum::from(Enum::B("bee".to_owned()));
  assert_eq!(b.code, FlatEnumCode::B);
  assert_eq!(b.description, r#"B("bee")"#);

  let c = FlatEnum::from(Enum::C { a: "c".to_owned() });
  assert_eq!(c.code, FlatEnumCode::C);
  assert_eq!(c.description, r#"C { a: "c" }"#);

  let d = FlatEnum::from(Enum::D {
    a: "d".to_owned(),
    b: 123,
  });
  assert_eq!(d.code, FlatEnumCode::D);
  assert_eq!(d.description, r#"D { a: "d", b: 123 }"#);
}

#[test]
fn test_to_flat_enum() {
  let a = Enum::A.to_flat_enum();
  assert_eq!(a.code, FlatEnumCode::A);
  assert_eq!(a.description, "A");

  let b = Enum::B("bee".to_owned()).to_flat_enum();
  assert_eq!(b.code, FlatEnumCode::B);
  assert_eq!(b.description, r#"B("bee")"#);

  let c = Enum::C { a: "c".to_owned() }.to_flat_enum();
  assert_eq!(c.code, FlatEnumCode::C);
  assert_eq!(c.description, r#"C { a: "c" }"#);

  let d = Enum::D {
    a: "d".to_owned(),
    b: 123,
  }
  .to_flat_enum();
  assert_eq!(d.code, FlatEnumCode::D);
  assert_eq!(d.description, r#"D { a: "d", b: 123 }"#);
}
