/// Supported sizes for RSA key generation.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum RsaBits {
  B2048 = 2048,
  B3072 = 3072,
  B4096 = 4096,
}

impl RsaBits {
  pub const fn bits(self) -> usize {
    match self {
      Self::B2048 => 2048,
      Self::B3072 => 3072,
      Self::B4096 => 4096,
    }
  }
}
