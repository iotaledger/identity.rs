macro_rules! assert_matches {
  ($($tt:tt)*) => {
    assert!(matches!($($tt)*))
  };
}

macro_rules! timestamp {
  ($expr:expr) => {{
    use ::std::convert::TryFrom;
    ::identity_vc::prelude::Timestamp::try_from($expr).unwrap()
  }};
}
