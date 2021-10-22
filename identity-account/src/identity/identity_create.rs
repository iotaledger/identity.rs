// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;
use identity_iota::tangle::NetworkName;

use crate::types::MethodSecret;

/// Configuration used to create a new Identity.
#[derive(Debug)]
pub(crate) struct IdentityCreate {
  pub(crate) key_type: KeyType,
  pub(crate) network: Option<NetworkName>,
  pub(crate) method_secret: Option<MethodSecret>,
}
