// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom as _;
use core::mem;
use crypto::hashes::sha::SHA256;
use crypto::hashes::sha::SHA256_LEN;

use crate::error::Error;
use crate::error::Result;
use crate::lib::*;

const U32_SIZE: usize = mem::size_of::<u32>();

/// The Concat KDF (using SHA-256) as defined in Section 5.8.1 of NIST.800-56A
pub fn concat_kdf(alg: &str, len: usize, z: &[u8], apu: &[u8], apv: &[u8]) -> Result<Vec<u8>> {
  let target: usize = (len + (SHA256_LEN - 1)) / SHA256_LEN;
  let rounds: u32 = u32::try_from(target).map_err(|_| Error::KeyError("Iteration Overflow"))?;

  let mut digest: [u8; SHA256_LEN] = [0; SHA256_LEN];
  let mut buffer: Vec<u8> = Vec::new();
  let mut output: Vec<u8> = Vec::new();

  // Iteration Count
  buffer.extend_from_slice(&[0; U32_SIZE]);

  // Derived Secret
  buffer.extend_from_slice(z);

  // AlgorithmId
  buffer.extend_from_slice(&(alg.len() as u32).to_be_bytes());
  buffer.extend_from_slice(alg.as_bytes());

  // PartyUInfo
  buffer.extend_from_slice(&(apu.len() as u32).to_be_bytes());
  buffer.extend_from_slice(apu);

  // PartyVInfo
  buffer.extend_from_slice(&(apv.len() as u32).to_be_bytes());
  buffer.extend_from_slice(apv);

  // Shared Key Length
  buffer.extend_from_slice(&((len * 8) as u32).to_be_bytes());

  for count in 0..rounds {
    // Update the iteration count
    buffer[..U32_SIZE].copy_from_slice(&(count as u32 + 1).to_be_bytes());

    SHA256(&buffer, &mut digest);

    output.extend_from_slice(&digest);
  }

  output.truncate(len);

  Ok(output)
}
