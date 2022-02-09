// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::ChainState;

#[napi(js_name = ChainState)]
pub struct JsChainState(pub(crate) ChainState);

impl From<ChainState> for JsChainState {
  fn from(chain_state: ChainState) -> Self {
    JsChainState(chain_state)
  }
}
