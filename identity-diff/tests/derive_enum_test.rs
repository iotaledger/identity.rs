// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![cfg(feature = "derive")]
#![allow(unused_variables)]
#![allow(deprecated)]

use identity_diff::Diff;
use serde::Deserialize;
use serde::Serialize;

#[derive(Diff, Debug, Clone, PartialEq)]
pub enum StructEnum {
  A { x: usize },
  B { y: usize },
}

#[derive(Diff, Debug, Clone, PartialEq)]
pub enum UnitEnum {
  A,
  B,
  C,
}

#[derive(Diff, Debug, Clone, PartialEq)]
pub enum TupleEnum {
  A(usize),
  B(String),
  C(usize, usize),
}

#[derive(Diff, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MixedEnum {
  A,
  B(usize),
  C { y: String },
}

#[derive(Diff, Debug, Clone, PartialEq)]
pub enum NestedEnum {
  Nest(InnerEnum),
}

#[derive(Diff, Debug, Clone, PartialEq)]
pub enum InnerEnum {
  Inner { y: InnerStruct },
}

impl Default for InnerEnum {
  fn default() -> Self {
    Self::Inner {
      y: InnerStruct::default(),
    }
  }
}
#[derive(Diff, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum TestOpt {
  Inner(Option<usize>),
  InnerS { a: Option<String> },
}

impl Default for TestOpt {
  fn default() -> Self {
    TestOpt::Inner(None)
  }
}

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct InnerStruct {
  y: usize,
}

#[derive(Diff, Debug, Clone, PartialEq)]
pub enum EnumWithGeneric<T, S>
where
  T: Clone + Default,
  S: Clone + Default,
{
  A(T),
  B(S),
}

#[derive(Diff, Debug, Clone, PartialEq)]
pub enum IgnoreEnum {
  A {
    #[diff(should_ignore)]
    x: usize,
    y: usize,
  },
  B(#[diff(should_ignore)] String, usize),
}

#[derive(Diff, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[diff(from_into)]
#[serde(untagged)]
pub enum IntoFrom {
  Test(TestOpt),
  SomeField(String),
}

#[test]
fn test_struct_enum() {
  let t = StructEnum::A { x: 100 };

  let t2 = StructEnum::B { y: 200 };

  let diff = t.diff(&t2).unwrap();

  let res = t.merge(diff).unwrap();

  assert_eq!(t2, res);

  let diff = t2.into_diff().unwrap();

  assert_eq!(
    DiffStructEnum::B {
      y: Some(200_usize.into_diff().unwrap())
    },
    diff
  );

  let res = StructEnum::from_diff(diff).unwrap();

  assert_eq!(StructEnum::B { y: 200 }, res);
}

#[test]
fn test_unit_enum() {
  let t = UnitEnum::A;
  let t2 = UnitEnum::B;

  let diff = t.diff(&t2).unwrap();

  let res = t.merge(diff).unwrap();

  assert_eq!(t2, res);

  let diff = t2.into_diff().unwrap();

  assert_eq!(DiffUnitEnum::B, diff);

  let res = UnitEnum::from_diff(diff).unwrap();

  assert_eq!(UnitEnum::B, res);
}

#[test]
fn test_tuple_enum() {
  let t = TupleEnum::A(10);
  let t2 = TupleEnum::C(20, 30);

  let diff = t.diff(&t2).unwrap();

  let res = t.merge(diff).unwrap();

  assert_eq!(t2, res);

  let diff = t2.into_diff().unwrap();

  assert_eq!(
    DiffTupleEnum::C(Some(20_usize.into_diff().unwrap()), Some(30_usize.into_diff().unwrap())),
    diff
  );

  let res = TupleEnum::from_diff(diff).unwrap();

  assert_eq!(TupleEnum::C(20, 30), res);
}

#[test]
fn test_mixed_enum() {
  let t = MixedEnum::B(10);
  let t2 = MixedEnum::C {
    y: String::from("test"),
  };

  let diff = t.diff(&t2).unwrap();

  let res = t.merge(diff).unwrap();

  assert_eq!(t2, res);

  let diff = t2.into_diff().unwrap();

  assert_eq!(
    DiffMixedEnum::C {
      y: Some(String::from("test").into_diff().unwrap())
    },
    diff
  );

  let res = MixedEnum::from_diff(diff).unwrap();

  assert_eq!(
    MixedEnum::C {
      y: String::from("test"),
    },
    res
  );
}

#[test]
fn test_nested_enum() {
  let t = NestedEnum::Nest(InnerEnum::default());
  let t2 = NestedEnum::Nest(InnerEnum::Inner {
    y: InnerStruct { y: 10 },
  });

  let diff = t.diff(&t2).unwrap();

  let res = t.merge(diff).unwrap();

  assert_eq!(t2, res);

  let diff = t2.into_diff().unwrap();

  assert_eq!(
    DiffNestedEnum::Nest(Some(DiffInnerEnum::Inner {
      y: Some(DiffInnerStruct {
        y: Some(10_usize.into_diff().unwrap())
      })
    })),
    diff
  );

  let res = NestedEnum::from_diff(diff).unwrap();

  assert_eq!(
    NestedEnum::Nest(InnerEnum::Inner {
      y: InnerStruct { y: 10 },
    }),
    res
  );
}

#[test]
fn test_enum_with_generics() {
  let t: EnumWithGeneric<String, usize> = EnumWithGeneric::A(String::from("test"));
  let t2: EnumWithGeneric<String, usize> = EnumWithGeneric::B(10);

  let diff = t.diff(&t2).unwrap();

  let res = t.merge(diff).unwrap();

  assert_eq!(t2, res);

  let diff = t2.into_diff().unwrap();

  assert_eq!(DiffEnumWithGeneric::B(Some(10_usize.into_diff().unwrap())), diff);

  let res = EnumWithGeneric::from_diff(diff).unwrap();

  assert_eq!(EnumWithGeneric::B(10), res);
}

#[test]
fn test_ignore_enum() {
  let t = IgnoreEnum::A { x: 10, y: 10 };
  let t2 = IgnoreEnum::B(String::from("test"), 30);

  let diff = t.diff(&t2).unwrap();

  let res = t.merge(diff).unwrap();

  let expected = IgnoreEnum::B(String::new(), 30);

  assert_eq!(expected, res)
}

#[test]
fn test_serde_enum() {
  let t = MixedEnum::B(10);
  let t2 = MixedEnum::C {
    y: String::from("test"),
  };

  let diff = t.diff(&t2).unwrap();

  let json = serde_json::to_string(&diff).unwrap();

  let diff = serde_json::from_str(&json).unwrap();

  let res = t.merge(diff).unwrap();

  assert_eq!(t2, res);

  let diff = t2.into_diff().unwrap();

  assert_eq!(
    DiffMixedEnum::C {
      y: Some(String::from("test").into_diff().unwrap())
    },
    diff
  );

  let res = MixedEnum::from_diff(diff).unwrap();

  assert_eq!(
    MixedEnum::C {
      y: String::from("test"),
    },
    res
  );
}

#[test]
fn test_enum_opt() {
  let t = TestOpt::Inner(None);
  let t2 = TestOpt::Inner(None);

  let diff1 = t.diff(&t2).unwrap();

  let diff2 = t2.into_diff().unwrap();

  assert_eq!(diff1, diff2);

  let t = TestOpt::InnerS { a: None };
  let diff = t.into_diff().unwrap();

  let json = serde_json::to_string(&diff).unwrap();

  let expected = r#"{"InnerS":{}}"#;

  assert_eq!(expected, json);

  let t = TestOpt::InnerS { a: None };
  let t2 = TestOpt::InnerS { a: None };

  let diff = t.diff(&t2).unwrap();

  let json = serde_json::to_string(&diff).unwrap();

  assert_eq!(expected, json);

  let t = TestOpt::Inner(None);
  let t2 = TestOpt::InnerS { a: None };

  let diff = t.diff(&t2).unwrap();

  let json = serde_json::to_string(&diff).unwrap();

  assert_eq!(expected, json);
}

#[test]
fn test_from_into() {
  let t = IntoFrom::SomeField(String::from("Test"));

  let t2 = IntoFrom::Test(TestOpt::Inner(Some(10)));

  let diff = t.diff(&t2).unwrap();

  let json = serde_json::to_string(&diff).unwrap();

  let expected = r#"{"Inner":10}"#;

  assert_eq!(expected, json);

  let diff: DiffIntoFrom = serde_json::from_str(&json).unwrap();

  let merge = t2.merge(diff).unwrap();

  let expected = IntoFrom::Test(TestOpt::Inner(Some(10)));

  assert_eq!(expected, merge);
}
