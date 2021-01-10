#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum JweFormat {
  Compact,
  General,
  Flatten,
}

impl Default for JweFormat {
  fn default() -> Self {
    Self::Compact
  }
}
