#[macro_export]
macro_rules! gen_bytes {
  ($length:expr) => {{
    let mut __array: [u8; $length] = [0; $length];
    ::crypto::rand::fill(&mut __array).map(|_| __array)
  }};
}
