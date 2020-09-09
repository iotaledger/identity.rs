use identity_derive::Diff;
use identity_diff::Diff;

#[derive(Diff, Debug, Clone, PartialEq)]
pub enum SomeEnum {
    Test { a: usize },
}

impl Default for SomeEnum {
    fn default() -> Self {
        Self::Test { a: 0 }
    }
}

#[test]
fn test_enum() {
    let t = SomeEnum::Test { a: 0 };

    let t2 = SomeEnum::Test { a: 0 };

    let diff = t.diff(&t2);

    println!("{:?}", diff);
}
