// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_data_integrity::proof::ProofPurpose;
use identity_did::did::CoreDIDUrl;

use crate::key_storage::KeyStorage;

use super::RemoteKey;

#[non_exhaustive]
/// [`Storage`] backed material extracted from a document's verification method.
/// Can be applied by cryptosuites when generating DataIntegrityProofs.
///
/// This struct can be obtained by calling
/// [`CoreDocumentExt::signing_material`](crate::CoreDocumentEx::signing_material()).
pub struct SigningMaterial<K: KeyStorage> {
  pub remote_key: RemoteKey<K>,
  pub verification_method: CoreDIDUrl,
  pub proof_purpose: ProofPurpose,
}
