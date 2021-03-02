// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::error::Result;

pub(crate) fn database_file<P, F>(path: &P, file: &F) -> PathBuf
where
  P: AsRef<Path> + ?Sized,
  F: AsRef<Path> + ?Sized,
{
  let path: &Path = path.as_ref();

  if maybe_file(path) {
    return path.to_path_buf();
  }

  path.join(file)
}

pub(crate) fn ensure_directory<P>(path: &P) -> Result<()>
where
  P: AsRef<Path> + ?Sized,
{
  if let Some(parent) = path.as_ref().parent() {
    fs::create_dir_all(parent)?;
  }

  Ok(())
}

pub(crate) fn maybe_file<P>(path: &P) -> bool
where
  P: AsRef<Path> + ?Sized,
{
  path.as_ref().is_file() || path.as_ref().extension().is_some()
}
