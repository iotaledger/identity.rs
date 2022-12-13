

#[async_trait]
trait CoreDocumentExt {

    async fn create_multikey_method<K, I>(&mut self, fragment: &str, schema: &MultikeySchema, storage: &Storage<K,I>) -> Result<()>;

    async fn sign<K,I,C, D>(&self, data: D, fragment: &str, storage: &Storage<K,I>, cryptosuite: C) -> Result<D>; 

    async fn purge_method<K,I>(&mut self, fragment: &str, storage: &Storage<K,I>) -> Result<()>; 
}

mod private {
    pub trait Sealed {}

    impl<D,T,U,V>  Sealed for CoreDocument<D = CoreDID, T = Object, U = Object, V = Object> 
    where 
        D: DID + KeyComparable
        {}
}