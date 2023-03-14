// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type StorageResult<T> = Result<T, StorageError>;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub async trait JwkStorageDocumentExt {
    // TODO: Also make it possible to set the value of `kid`. This will require changes to the `JwkStorage`. 
    async fn generate_method<K,I>(&mut self, storage: &Storage<K,I>, key_type: KeyType, alg: JwsAlgorithm, fragment: Option<&str>) -> StorageResult<()>
    where 
        K: JwkStorage,
        I: KeyIdStorage
    ;

    async fn remove_method<K,I>(&mut self, storage: &Storage<K,I>, fragment: &str) -> StorageResult<()>
    where 
        K: JwkStorage,
        I: KeyIdStorage
    ;

    async fn sign_bytes<K,I>(&self, storage: &Storage<K,I>, fragment: &str, data: Vec<u8>) -> StorageResult<String> 
    where
        K: JwkStorage,
        I: KeyIdStorage
    ;


    async fn create_presentation_jwt<K,I>(&self, fragment: &str) -> StorageResult<String>
    where 
        K: JwkStorage,
        I: KeyIdStorage
    {
        todo!()
    }

    async fn create_credential_jwt<K,I,T>(&self, credential: &Credential<T>, fragment: &str) -> StorageResult<String>
    where
        K: JwkStorage,
        I: KeyIdStorage,
        T: Serialize, 
    {
        todo!()
    }
}
