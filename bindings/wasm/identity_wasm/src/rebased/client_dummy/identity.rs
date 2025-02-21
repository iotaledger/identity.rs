// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This file has been moved here from identity_iota_core/src/client_dummy.
// The file will be removed after the TS-Client-SDK is integrated.
// The file provides a POC for the wasm-bindgen glue code needed to
// implement the TS-Client-SDK integration.

use serde;
use serde::Deserialize;
use serde::Serialize;

use super::Multicontroller;
use identity_iota::iota::IotaDocument;
use identity_iota::iota_interaction::types::id::UID;

#[derive(Debug, Deserialize, Serialize)]
pub struct OnChainIdentity {
  pub id: UID,
  pub did_doc: Multicontroller<Vec<u8>>,
}

impl OnChainIdentity {}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProposalAction {
  UpdateDocument(IotaDocument),
  Deactivate,
}
