use std::collections::HashSet;

pub trait Alphabet {
    fn contains(&self, c: char) -> bool;
}

pub struct AlphabetSet {
    alpha: HashSet<char>,
}

impl AlphabetSet {
    pub fn new() -> Self {
        Self { alpha: HashSet::new() }
    }

    pub fn insert(&mut self, vec: Vec<char>) {
        self.alpha.extend(vec.into_iter())
    }
}

impl Alphabet for AlphabetSet {
    fn contains(&self, c: char) -> bool {
        self.alpha.contains(&c)
    }
}
