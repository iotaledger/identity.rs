#[macro_export]
macro_rules! to_bytes {
  ($slice:expr, $length:expr, $name:expr) => {{
    use ::core::convert::TryInto as _;
    let __array: $crate::error::Result<[u8; $length]> = $slice
      .try_into()
      .map_err(|_| $crate::error::Error::InvalidArray(concat!("Invalid ", $name, " Length")));

    __array
  }};
}

#[macro_export]
macro_rules! gen_bytes {
  ($length:expr) => {{
    let mut __array: [u8; $length] = [0; $length];
    ::crypto::rand::fill(&mut __array).map(|_| __array)
  }};
}
