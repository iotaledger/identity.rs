// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use identity_core::crypto::KeyPair;
use identity_iota_core::did::IotaDIDUrl;
use identity_iota_core::document::IotaDocument;

#[derive(Debug, Clone)]
pub struct ActorIdentity {
  pub(crate) doc: IotaDocument,
  pub(crate) keypairs: HashMap<IotaDIDUrl, KeyPair>,
}

impl From<(IotaDocument, HashMap<IotaDIDUrl, KeyPair>)> for ActorIdentity {
  fn from(tuple: (IotaDocument, HashMap<IotaDIDUrl, KeyPair>)) -> Self {
    Self {
      doc: tuple.0,
      keypairs: tuple.1,
    }
  }
}
