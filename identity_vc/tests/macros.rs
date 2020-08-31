#[macro_export]
macro_rules! assert_matches {
  ($($tt:tt)*) => {
    assert!(matches!($($tt)*))
  };
}

#[macro_export]
macro_rules! timestamp {
  ($expr:expr) => {{
    use ::std::convert::TryFrom;
    ::identity_core::common::Timestamp::try_from($expr).unwrap()
  }};
}
