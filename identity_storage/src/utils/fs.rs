// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::Path;

pub fn ensure_directory<P>(path: &P) -> Result<(), std::io::Error>
where
  P: AsRef<Path> + ?Sized,
{
  if let Some(parent) = path.as_ref().parent() {
    fs::create_dir_all(parent)?;
  }

  Ok(())
}

#[cfg(test)]
pub(crate) fn random_temporary_path() -> String {
  use rand::distributions::DistString;
  use rand::rngs::OsRng;

  let mut file = std::env::temp_dir();
  file.push("test_strongholds");
  file.push(rand::distributions::Alphanumeric.sample_string(&mut OsRng, 32));
  file.set_extension("stronghold");
  file.to_str().unwrap().to_owned()
}
