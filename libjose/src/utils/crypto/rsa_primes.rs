// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use num_bigint_dig::traits::ModInverse;
use zeroize::Zeroize;

use crate::error::Error;
use crate::error::Result;
use crate::utils::RsaUint;

pub struct RsaComputed {
  // D mod (P-1)
  pub(crate) dp: RsaUint,
  // D mod (Q-1)
  pub(crate) dq: RsaUint,
  // Q^-1 mod P
  pub(crate) qi: RsaUint,
}

impl RsaComputed {
  pub fn new(d: &RsaUint, p: &RsaUint, q: &RsaUint) -> Result<Self> {
    let one: RsaUint = RsaUint::new(vec![1]);

    let dp: RsaUint = d % (p - &one);
    let dq: RsaUint = d % (q - &one);

    let qi: RsaUint = q
      .clone()
      .mod_inverse(p)
      .ok_or(Error::InvalidRsaPrime)?
      .to_biguint()
      .ok_or(Error::InvalidRsaPrime)?;

    Ok(Self { dp, dq, qi })
  }
}

impl Zeroize for RsaComputed {
  fn zeroize(&mut self) {
    self.dp.zeroize();
    self.dq.zeroize();
    self.qi.zeroize();
  }
}

impl Drop for RsaComputed {
  fn drop(&mut self) {
    self.zeroize();
  }
}
