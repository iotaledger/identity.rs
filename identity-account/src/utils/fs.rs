// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::Path;

use crate::error::Result;

pub fn ensure_directory<P>(path: &P) -> Result<()>
where
  P: AsRef<Path> + ?Sized,
{
  if let Some(parent) = path.as_ref().parent() {
    fs::create_dir_all(parent)?;
  }

  Ok(())
}
