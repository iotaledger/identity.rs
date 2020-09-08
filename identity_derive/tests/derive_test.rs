use identity_derive::Diff;
use identity_diff::Diff;

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct Test {
    a: u32,
}

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct TestTuple(String);

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct TestMixture(Test);

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct TestNest {
    a: Test,
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
