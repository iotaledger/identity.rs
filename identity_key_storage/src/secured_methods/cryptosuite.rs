// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_data_integrity::proof::ProofOptions;

use crate::identifiers::KeyId;
use crate::key_storage::KeyStorage;
use crate::signature::Signature;
use crate::storage_error::SimpleStorageError;

use super::Signable;

#[async_trait(?Send)]
pub trait CryptoSuite<K>
where
  K: KeyStorage,
{
  async fn sign_data_integrity<'signable>(
    &self,
    key_id: &KeyId,
    data: Signable<'signable>,
    proof_options: ProofOptions,
    key_storage: &K,
  ) -> Result<Signature, SimpleStorageError>
  where
    'signable: 'async_trait;
}
