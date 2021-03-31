// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod encrypted;
mod plaintext;
mod signed;
mod traits;

pub use self::{
  encrypted::{Algorithm as EncryptionAlgorithm, Envelope as Encrypted},
  plaintext::Envelope as Plaintext,
  signed::{Algorithm as SignatureAlgorithm, Envelope as Signed},
  traits::*,
};
