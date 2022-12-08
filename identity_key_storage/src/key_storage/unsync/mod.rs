// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_trait::async_trait;

use crate::key_generation_schema::MultikeySchema;

#[async_trait(?Send)]
trait KeyStorage {
    type SigningAlgorithm: std::fmt::Display + FromStr + TryFrom<String> + AsRef<str>;
    
    async fn generate_multikey(schema: &MultikeySchema); 
}