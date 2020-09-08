use identity_derive::Diff;
use identity_diff::Diff;
use serde::{Deserialize, Serialize};

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct Test {
    a: u32,
}

#[test]
fn test_struct() {
    let t = Test { a: 10 };

    let t2 = Test { a: 2 };

    let diff = t.diff(&t2);

    println!("{:?}", diff);
}
