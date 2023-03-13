// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;
use identity_core::diff::Diff;
use identity_core::diff::DiffString;
use identity_core::diff::DiffVec;

use crate::jwk::JwkOperation;
use crate::jwk::JwkType;
use crate::jwk::JwkUse;

pub struct DiffJwk {
  kty: <JwkType as Diff>::Type,
  use_: Option<<JwkUse as Diff>::Type>,
  key_ops: Option<DiffVec<JwkOperation>>,
  alg: Option<DiffString>,
  kid: Option<DiffString>,
  x5u: Option<<Url as Diff>::Type>,
}
