// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use identity_core::convert::FromJson;

use crate::error::Result;

pub fn deserialize<T>(data: Vec<u8>) -> Result<T>
where
  T: for<'a> Deserialize<'a>,
{
  T::from_json_slice(&data).map_err(Into::into)
}

pub fn deserialize_opt<T>(data: Vec<u8>) -> Result<Option<T>>
where
  T: for<'a> Deserialize<'a>,
{
  if data.is_empty() {
    Ok(None)
  } else {
    deserialize(data)
  }
}

pub fn deserialize_list<T>(data: Vec<Vec<u8>>) -> Result<Vec<T>>
where
  T: for<'a> Deserialize<'a>,
{
  data.into_iter().map(deserialize).collect()
}
