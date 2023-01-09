// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_data_integrity::proof::ProofPurpose;
use identity_did::did::CoreDIDUrl;

use super::RemoteKey;

#[non_exhaustive]
/// [`Storage`] backed material extracted from a document's verification method.
/// Can be applied by cryptosuites when generating DataIntegrityProofs.
///
/// This struct can be obtained by calling
/// [`CoreDocumentExt::signing_material`](crate::CoreDocumentEx::signing_material()).
pub struct SigningMaterial<F> {
  pub remote_key: RemoteKey<F>,
  pub verification_method: CoreDIDUrl,
  pub proof_purpose: Option<ProofPurpose>,
}
