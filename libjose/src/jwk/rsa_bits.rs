/// Supported sizes for RSA key generation.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum RsaBits {
  B2048 = 2048,
  B3072 = 3072,
  B4096 = 4096,
  B8192 = 8192,
}
