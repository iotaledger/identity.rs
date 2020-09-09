use identity_derive::Diff;
use identity_diff::Diff;

#[derive(Diff, Debug, Clone, PartialEq)]
pub enum SomeEnum {
    A,
    B,
}

impl Default for SomeEnum {
    fn default() -> Self {
        Self::A
    }
}

#[test]
fn test_enum() {
    let t = SomeEnum::A;

    let t2 = SomeEnum::B;

    let diff = t.diff(&t2);

    println!("{:?}", diff);
}
