use identity_diff::Diff;
use serde::{Deserialize, Serialize};

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct Test {
    a: u32,
}

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct SomeGeneric<T: Copy>(T);

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct TestTuple(String);

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct TestMixture(Test);

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct TestNest {
    a: Test,
}

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct BigTuple(usize, Vec<usize>, bool, String);

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct BigStruct {
    a: Vec<usize>,
    b: bool,
    c: String,
    d: usize,
}

#[derive(Diff, Debug, Clone, PartialEq, Default)]
struct TestUnit;

#[derive(Diff, Debug, Clone, PartialEq)]
struct TestIgnore {
    a: usize,
    #[diff(should_ignore)]
    b: usize,
}

#[derive(Diff, Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
struct JsonStruct {
    a: usize,
    b: String,
    #[diff(should_ignore)]
    c: Option<i32>,
    d: Vec<u32>,
}

#[test]
fn test_traditional_struct() {
    let t = Test { a: 10 };
    let t2 = Test { a: 10 };

    let diff = t.diff(&t2);

    let res = t.merge(diff);

    let expected = Test { a: 10 };

    assert_eq!(expected, res);

    let t3 = Test { a: 2 };

    let diff = t.diff(&t3);

    let res = t.merge(diff);

    let expected = Test { a: 2 };

    assert_eq!(expected, res);
}

#[test]
fn test_tuple_struct() {
    let t = TestTuple(String::from("Some String"));
    let t2 = TestTuple(String::from("Some String"));

    let diff = t.diff(&t2);

    let res = t.merge(diff);

    let expected = TestTuple(String::from("Some String"));

    assert_eq!(expected, res);

    let t3 = TestTuple(String::from("Some new String"));

    let diff = t.diff(&t3);

    let res = t.merge(diff);

    let expected = TestTuple(String::from("Some new String"));

    assert_eq!(expected, res);
}

#[test]
fn test_unit_struct() {
    let t = TestUnit;

    let t2 = TestUnit;

    let diff = t.diff(&t2);

    let res = t.merge(diff);

    let expected = TestUnit;

    assert_eq!(expected, res);
}

#[test]
fn test_generic_struct() {
    let t = SomeGeneric::<usize>(10);

    let t2 = SomeGeneric::<usize>(20);

    let diff = t.diff(&t2);

    let res = t.merge(diff);

    let expected = SomeGeneric::<usize>(20);

    assert_eq!(expected, res)
}

#[test]
fn test_embed() {
    let t = TestMixture(Test { a: 10 });

    let t2 = TestMixture(Test { a: 20 });

    let diff = t.diff(&t2);

    let res = t.merge(diff);

    let expected = TestMixture(Test { a: 20 });

    assert_eq!(expected, res);

    let t = TestNest { a: Test { a: 10 } };

    let t2 = TestNest { a: Test { a: 20 } };

    let diff = t.diff(&t2);

    let res = t.merge(diff);

    let expected = TestNest { a: Test { a: 20 } };

    assert_eq!(expected, res);
}

#[test]
fn test_big_struct() {
    let t = BigStruct {
        a: vec![1, 2, 3],
        b: true,
        c: String::from("Some String"),
        d: 10,
    };
    let t2 = BigStruct {
        a: vec![1, 2, 3],
        b: true,
        c: String::from("Some String"),
        d: 10,
    };

    let diff = t.diff(&t2);

    let res = t.merge(diff);

    let expected = BigStruct {
        a: vec![1, 2, 3],
        b: true,
        c: String::from("Some String"),
        d: 10,
    };

    assert_eq!(expected, res);

    let t3 = BigStruct {
        a: vec![5, 6, 7],
        b: false,
        c: String::from("Some New String"),
        d: 15,
    };

    let diff = t.diff(&t3);

    let res = t.merge(diff);

    let expected = BigStruct {
        a: vec![5, 6, 7],
        b: false,
        c: String::from("Some New String"),
        d: 15,
    };

    assert_eq!(expected, res);
}

#[test]
fn test_big_tuple() {
    let t = BigTuple(10, vec![1, 2, 3], true, String::from("Some String"));
    let t2 = BigTuple(10, vec![1, 2, 3], true, String::from("Some String"));

    let diff = t.diff(&t2);

    let res = t.merge(diff);

    let expected = BigTuple(10, vec![1, 2, 3], true, String::from("Some String"));

    assert_eq!(expected, res);

    let t3 = BigTuple(15, vec![5, 6, 7], false, String::from("Some New String"));

    let diff = t.diff(&t3);

    let res = t.merge(diff);

    let expected = BigTuple(15, vec![5, 6, 7], false, String::from("Some New String"));

    assert_eq!(expected, res);
}

#[test]
fn test_ignore_struct() {
    let t = TestIgnore { a: 10, b: 10 };
    let t2 = TestIgnore { a: 100, b: 2039123 };

    let diff = t.diff(&t2);

    let res = t.merge(diff);

    let expected = TestIgnore { b: 10, ..t2 };

    assert_eq!(expected, res);
}

#[test]
fn test_json_diff() {
    let t = JsonStruct {
        a: 10,
        b: String::from("test"),
        c: Some(30),
        d: vec![20, 30, 1],
    };

    let t2 = JsonStruct {
        a: 30,
        b: String::from("tester"),
        c: Some(50),
        d: vec![20, 100],
    };

    let diff = t.diff(&t2);

    println!("{:?}", diff);

    let json = serde_json::to_string(&diff).unwrap();

    println!("{}", json);

    let diff: DiffJsonStruct = serde_json::from_str(&json).unwrap();

    println!("{:?}", diff);
}
