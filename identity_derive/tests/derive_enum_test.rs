use identity_derive::Diff;
use identity_diff::Diff;

#[derive(Diff, Debug, Clone, PartialEq)]
pub enum SomeEnum {
    Test,
}

impl Default for SomeEnum {
    fn default() -> Self {
        Self::Test
    }
}

#[test]
fn test_enum() {
    let t = SomeEnum::Test;

    let t2 = SomeEnum::Test;

    let diff = t.diff(&t2);

    println!("{:?}", diff);
}
