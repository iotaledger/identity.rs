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

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct BigTuple(usize, Vec<usize>, bool, String);

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct BigStruct {
    a: Vec<usize>,
    b: bool,
    c: String,
    d: usize,
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
