// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::IotaDID;

#[napi(js_name = DID)]
pub struct JsDID(pub(crate) IotaDID);
