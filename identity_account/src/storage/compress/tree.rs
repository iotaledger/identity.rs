use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Eq, Debug, Clone, Serialize, Deserialize)]
pub struct HTree {
    pub count: u32,
    pub value: Option<char>,
    pub left: Option<Box<HTree>>,
    pub right: Option<Box<HTree>>,
}
impl HTree {
    /// Create a new Huffman Tree.
    pub fn new(value: char, count: u32) -> Self {
        HTree {
            count,
            value: Some(value),
            left: None,
            right: None,
        }
    }

    /// Merge two Huffman Trees.
    pub fn merge(tree_left: HTree, tree_right: HTree) -> HTree {
        HTree {
            count: tree_left.count + tree_right.count,
            value: None,
            left: Some(Box::new(tree_left)),
            right: Some(Box::new(tree_right)),
        }
    }
}

impl Ord for HTree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for HTree {
    fn partial_cmp(&self, other: &HTree) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

impl PartialEq for HTree {
    fn eq(&self, other: &HTree) -> bool {
        self.count == other.count
    }
}
