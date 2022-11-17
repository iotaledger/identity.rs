// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use crypto::hashes::sha::Sha256;
use crypto::hashes::Digest;

use crate::error::Error;
use crate::error::Result;
use crate::lib::*;

/// The Concat KDF (using SHA-256) as defined in Section 5.8.1 of NIST.800-56A
pub fn concat_kdf(alg: &str, len: usize, z: &[u8], apu: &[u8], apv: &[u8]) -> Result<Vec<u8>> {
  let mut digest: Sha256 = Sha256::new();
  let mut output: Vec<u8> = Vec::new();

  let target: usize = (len + (Sha256::output_size() - 1)) / Sha256::output_size();
  let rounds: u32 = u32::try_from(target).map_err(|_| Error::KeyError("Iteration Overflow"))?;

  for count in 0..rounds {
    // Iteration Count
    digest.update((count + 1).to_be_bytes());

    // Derived Secret
    digest.update(z);

    // AlgorithmId
    digest.update((alg.len() as u32).to_be_bytes());
    digest.update(alg.as_bytes());

    // PartyUInfo
    digest.update((apu.len() as u32).to_be_bytes());
    digest.update(apu);

    // PartyVInfo
    digest.update((apv.len() as u32).to_be_bytes());
    digest.update(apv);

    // Shared Key Length
    digest.update(((len * 8) as u32).to_be_bytes());

    output.extend_from_slice(&digest.finalize_reset());
  }

  output.truncate(len);

  Ok(output)
}
