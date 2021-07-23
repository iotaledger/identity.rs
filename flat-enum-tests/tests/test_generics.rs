#![no_std]
extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;
use flat_enum::derive::FlatEnum;
use flat_enum::IntoFlatEnum;
use core::fmt::Formatter;
use crate::alloc::string::ToString;

#[derive(Debug, FlatEnum)]
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

#[derive(Debug, FlatEnum)]
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
fn test_flat_enum_generic_basic() {
  let a = FlatGenericEnum {
    code: FlatGenericEnumCode::A,
    description: "A".to_owned(),
  };
  assert_eq!(a.code, FlatGenericEnumCode::A);
  assert_eq!(a.description, "A".to_owned());
}

#[test]
fn test_flat_enum_two_generics_basic() {
  let a = FlatTwoGenericsEnum {
    code: FlatTwoGenericsEnumCode::A,
    description: "A".to_owned(),
  };
  assert_eq!(a.code, FlatTwoGenericsEnumCode::A);
  assert_eq!(a.description, "A".to_owned());
}

#[test]
fn test_flat_enum_two_generics() {
  let g = FlatTwoGenericsEnum::from(TwoGenericsEnum::G::<u8, i64> { a: 42, b: 123 });
  assert_eq!(g.code, FlatTwoGenericsEnumCode::G);
  assert_eq!(g.description, "G { a: 42, b: 123 }".to_owned());
}

#[test]
fn test_two_generics_to_flat_enum() {
  let g = TwoGenericsEnum::G::<u8, i64> { a: 42, b: 123 }.into_flat_enum();
  assert_eq!(g.code, FlatTwoGenericsEnumCode::G);
  assert_eq!(g.description, "G { a: 42, b: 123 }".to_owned());
}
