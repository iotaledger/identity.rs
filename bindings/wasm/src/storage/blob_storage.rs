// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_storage::BlobStorage;
use identity_storage::StorageResult;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "BlobStorage")]
  pub type WasmBlobStorage;
}

#[async_trait::async_trait(?Send)]
impl BlobStorage for WasmBlobStorage {
  async fn store(&self, key: &str, blob: Option<Vec<u8>>) -> StorageResult<()> {
    todo!()
  }

  async fn load(&self, key: &str) -> StorageResult<Option<Vec<u8>>> {
    todo!()
  }

  async fn flush(&self) -> StorageResult<()> {
    todo!()
  }
}

#[wasm_bindgen(typescript_custom_section)]
const STORAGE: &'static str = r#"
interface BlobStorage {
  // TODO
}"#;
