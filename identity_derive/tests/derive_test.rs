use identity_derive::Diff;
use identity_diff::Diff;

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct Test {
    a: u32,
}

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct TestTuple(String);

#[test]
fn test_traditional_struct() {
    let t = Test { a: 10 };
    let t2 = Test { a: 10 };

    let diff1 = t.diff(&t2);

    let res1 = t.merge(diff1);

    let expected = Test { a: 10 };

    assert_eq!(expected, res1);

    let t3 = Test { a: 2 };

    let diff2 = t.diff(&t3);

    let res2 = t.merge(diff2);

    let expected = Test { a: 2 };

    assert_eq!(expected, res2);
}

#[test]
fn test_tuple_struct() {
    let t = TestTuple(String::from("Some String"));
    let t2 = TestTuple(String::from("Some String"));

    let diff1 = t.diff(&t2);

    let res1 = t.merge(diff1);

    let expected = TestTuple(String::from("Some String"));

    assert_eq!(expected, res1);

    let t3 = TestTuple(String::from("Some new String"));

    let diff2 = t.diff(&t3);

    let res2 = t.merge(diff2);

    let expected = TestTuple(String::from("Some new String"));

    assert_eq!(expected, res2);
}
