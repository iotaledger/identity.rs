// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::slip10::ChainCode;
use iota_stronghold::ProcResult;
use iota_stronghold::ResultMessage;

use crate::error::Error;
use crate::error::PleaseDontMakeYourOwnResult;
use crate::error::Result;

impl<T> PleaseDontMakeYourOwnResult<T> for ResultMessage<T> {
  #[allow(clippy::wrong_self_convention)]
  fn to_result(self) -> Result<T, Error> {
    match self {
      Self::Ok(data) => Ok(data),
      Self::Error(error) => Err(Error::StrongholdResult(error)),
    }
  }
}

#[derive(Clone, Debug)]
pub enum ProcedureResult {
  SLIP10Generate,
  SLIP10Derive(ChainCode),
  BIP39Recover,
  BIP39Generate,
  BIP39MnemonicSentence(String),
  Ed25519PublicKey([u8; 32]),
  Ed25519Sign([u8; 64]),
}

impl PleaseDontMakeYourOwnResult<ProcedureResult> for ProcResult {
  #[allow(clippy::wrong_self_convention)]
  fn to_result(self) -> Result<ProcedureResult> {
    match self {
      ProcResult::SLIP10Generate(inner) => inner.to_result().map(|_| ProcedureResult::SLIP10Generate),
      ProcResult::SLIP10Derive(inner) => inner.to_result().map(ProcedureResult::SLIP10Derive),
      ProcResult::BIP39Recover(inner) => inner.to_result().map(|_| ProcedureResult::BIP39Recover),
      ProcResult::BIP39Generate(inner) => inner.to_result().map(|_| ProcedureResult::BIP39Generate),
      ProcResult::BIP39MnemonicSentence(inner) => inner.to_result().map(ProcedureResult::BIP39MnemonicSentence),
      ProcResult::Ed25519PublicKey(inner) => inner.to_result().map(ProcedureResult::Ed25519PublicKey),
      ProcResult::Ed25519Sign(inner) => inner.to_result().map(ProcedureResult::Ed25519Sign),
      ProcResult::Error(inner) => Err(Error::StrongholdResult(inner)),
    }
  }
}
