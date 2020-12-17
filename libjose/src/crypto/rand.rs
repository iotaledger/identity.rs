pub use ::rand::CryptoRng;
pub use ::rand::Rng;
pub use ::rand::RngCore;

pub use ::rand::rngs::OsRng;

use crate::error::Result;
use crate::lib::*;

pub fn random_bytes<R>(size: usize, mut rng: R) -> Result<Vec<u8>>
where
  R: RngCore + CryptoRng,
{
  let mut bytes: Vec<u8> = vec![0; size];

  rng.try_fill_bytes(&mut bytes).expect("Handle RNG Error");

  Ok(bytes)
}
