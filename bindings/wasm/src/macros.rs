// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[macro_export]
macro_rules! log {
  ($($tt:tt)*) => {
    web_sys::console::log_1(&format!($($tt)*).into());
  }
}
