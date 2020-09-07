use identity_derive::Diff;
use identity_diff::Diff;
use serde::{Deserialize, Serialize};

#[derive(Diff, Debug, Clone, PartialEq, Default)]
pub struct Test(u32);

#[test]
fn test_struct() {
    let t = Test(10);

    let t2 = Test(2);

    let diff = test.diff(&test_2);

    println!("{:?}", diff);
}
