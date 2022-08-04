// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_did::{did::DID, document::Document};

pub trait Resolve {
  type D: DID;
  type DOC: Document<D = Self::D>;
}
