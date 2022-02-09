// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::Signature;

#[napi(js_name = Signature)]
pub struct JsSignature(pub(crate) Signature);

impl From<Signature> for JsSignature {
  fn from(signature: Signature) -> Self {
    JsSignature(signature)
  }
}
