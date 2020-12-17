use crate::crypto::digest;
use crate::crypto::digest::Digest as _;
use crate::error::Result;
use crate::lib::*;

/// The Concat KDF (using SHA-256) as defined in Section 5.8.1 of NIST.800-56A
pub fn concat_kdf(alg: &str, len: usize, z: &[u8], apu: &[u8], apv: &[u8]) -> Result<Vec<u8>> {
  let mut digest: digest::SHA2_256 = digest::SHA2_256::new();
  let mut output: Vec<u8> = Vec::new();

  let length: usize = digest::SHA2_256::output_size();
  let rounds: usize = (len + (length - 1)) / length;

  for count in 0..rounds {
    // Iteration Count
    digest.update(&(count + 1).to_be_bytes());

    // Derived Secret
    digest.update(z);

    // AlgorithmId
    digest.update(&(alg.len() as u32).to_be_bytes());
    digest.update(alg.as_bytes());

    // PartyUInfo
    digest.update(&(apu.len() as u32).to_be_bytes());
    digest.update(apu);

    // PartyVInfo
    digest.update(&(apv.len() as u32).to_be_bytes());
    digest.update(apv);

    // Shared Key Length
    digest.update(&((len * 8) as u32).to_be_bytes());

    output.extend_from_slice(&digest.finalize_reset());
  }

  output.truncate(len);

  Ok(output)
}
