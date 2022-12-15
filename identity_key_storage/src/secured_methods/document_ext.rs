// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0


#[async_trait(?Send)]
pub trait CoreDocumentExt: private::Sealed {

    async fn create_multikey_method<K, I>(&mut self, fragment: &str, schema: &MultikeySchema, storage: &Storage<K,I>) -> Result<()>;


    async fn purge_method<K,I>(&mut self, fragment: &str, storage: &Storage<K,I>) -> Result<()>; 
}

mod private {
    pub trait Sealed {}

    impl<D,T,U,V>  Sealed for CoreDocument<D = CoreDID, T = Object, U = Object, V = Object> 
    where 
        D: DID + KeyComparable
        {}
}

#[async_trait(?Send)]
impl<D: DID + KeyComparable, T, U, V> CoreDocumentExt for CoreDocument<D,T,U,V> {
    
}