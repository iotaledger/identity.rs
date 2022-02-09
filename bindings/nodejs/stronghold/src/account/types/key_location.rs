// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::KeyLocation;

#[napi(js_name = KeyLocation)]
pub struct JsKeyLocation(pub(crate) KeyLocation);
