// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::error::Error;

pub(crate) fn cast<'a>(error: &'a (dyn Error + Send + Sync + 'static)) -> &'a (dyn Error + 'static) {
  error
}

pub(crate) trait AsDynError {
  fn as_dyn_err(&self) -> Option<&(dyn Error + 'static)>;
}

impl AsDynError for Option<Box<dyn Error + Send + Sync + 'static>> {
  fn as_dyn_err(&self) -> Option<&(dyn Error + 'static)> {
    self.as_deref().map(cast)
  }
}
