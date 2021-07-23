#![no_std]
extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;
use flat_enum::IntoFlatEnum;
use core::fmt::Formatter;
use alloc::string::ToString;

#[derive(Debug, flat_enum::derive::FlatEnum)]
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
fn test_flat_enum_basic() {
  let a = FlatBasicEnum {
    code: FlatBasicEnumCode::A,
    description: "A".to_owned(),
  };
  assert_eq!(a.code, FlatBasicEnumCode::A);
  assert_eq!(a.description, "A");
}

#[test]
fn test_flat_enum_from() {
  let a = FlatBasicEnum::from(BasicEnum::A);
  assert_eq!(a.code, FlatBasicEnumCode::A);
  assert_eq!(a.description, "A");

  let b = FlatBasicEnum::from(BasicEnum::B("bee".to_owned()));
  assert_eq!(b.code, FlatBasicEnumCode::B);
  assert_eq!(b.description, r#"B("bee")"#);

  let c = FlatBasicEnum::from(BasicEnum::C { a: "c".to_owned() });
  assert_eq!(c.code, FlatBasicEnumCode::C);
  assert_eq!(c.description, r#"C { a: "c" }"#);

  let d = FlatBasicEnum::from(BasicEnum::D {
    a: "d".to_owned(),
    b: 123,
  });
  assert_eq!(d.code, FlatBasicEnumCode::D);
  assert_eq!(d.description, r#"D { a: "d", b: 123 }"#);
}

#[test]
fn test_to_flat_enum() {
  let a = BasicEnum::A.into_flat_enum();
  assert_eq!(a.code, FlatBasicEnumCode::A);
  assert_eq!(a.description, "A");

  let b = BasicEnum::B("bee".to_owned()).into_flat_enum();
  assert_eq!(b.code, FlatBasicEnumCode::B);
  assert_eq!(b.description, r#"B("bee")"#);

  let c = BasicEnum::C { a: "c".to_owned() }.into_flat_enum();
  assert_eq!(c.code, FlatBasicEnumCode::C);
  assert_eq!(c.description, r#"C { a: "c" }"#);

  let d = BasicEnum::D {
    a: "d".to_owned(),
    b: 123,
  }
  .into_flat_enum();
  assert_eq!(d.code, FlatBasicEnumCode::D);
  assert_eq!(d.description, r#"D { a: "d", b: 123 }"#);
}
