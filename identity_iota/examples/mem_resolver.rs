use async_trait::async_trait;
use std::collections::BTreeMap;

use identity_core::{
    self as core,
    common::{AsJson as _, Object, OneOrMany, Timestamp, Url},
    did::{DIDDocument, DIDDocumentBuilder, KeyData, KeyType, PublicKey, PublicKeyBuilder, ServiceBuilder, DID},
    resolver::{
        dereference, resolve, Dereference, DocumentMetadata, InputMetadata, MetaDocument, Resolution, ResolverMethod,
    },
};
use identity_crypto::{Ed25519, KeyGen as _, KeyPair};
use identity_iota::error::Result;
use multihash::{Blake2b256, MultihashGeneric};

#[smol_potat::main]
async fn main() -> Result<()> {
    let mut resolver: MemResolver = MemResolver::new();

    let keypair: KeyPair = Ed25519::generate(&Ed25519, Default::default())?;
    let public: String = bs58::encode(keypair.public()).into_string();

    let ident: MultihashGeneric<_> = Blake2b256::digest(public.as_bytes());
    let ident: String = bs58::encode(ident.digest()).into_string();

    let did: DID = format!("did:iota:com:{}", ident).parse()?;
    let key: DID = format!("did:iota:com:{}#key-1", ident).parse()?;
    let srv: DID = format!("did:iota:com:{}#srv-1", ident).parse()?;

    let pkey: PublicKey = PublicKeyBuilder::default()
        .id(key.clone())
        .controller(did.clone())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .key_data(KeyData::PublicKeyBase58(public))
        .build()
        .unwrap();

    let doc: DIDDocument = DIDDocumentBuilder::default()
        .context(OneOrMany::One(DID::BASE_CONTEXT.into()))
        .id(did.clone())
        .public_keys(vec![pkey.clone()])
        .auth(vec![key.clone().into()])
        .assert(vec![key.clone().into()])
        .verification(vec![key.clone().into()])
        .delegation(vec![key.clone().into()])
        .invocation(vec![key.clone().into()])
        .agreement(vec![key.clone().into()])
        .services(vec![ServiceBuilder::default()
            .id(srv.clone())
            .service_type("MyService")
            .endpoint(Url::parse("https://example.com")?)
            .build()
            .unwrap()])
        .build()
        .unwrap();

    println!("> Doc");
    println!("{}", doc.to_json_pretty()?);

    resolver.add(doc)?;

    let did: String = did.to_string();
    let key: String = key.to_string();
    let srv: String = format!("{}?service=srv-1", did);

    let data: Resolution = resolve(&did, InputMetadata::new(), &resolver).await?;
    println!("> Resolution");
    println!("{}", data.to_json_pretty()?);

    let data: Resolution = resolve("not-a-did", InputMetadata::new(), &resolver).await?;
    println!("> Resolution (invalid-did)");
    println!("{}", data.to_json_pretty()?);

    let data: Resolution = resolve("did:iota:1234", InputMetadata::new(), &resolver).await?;
    println!("> Resolution (not-found)");
    println!("{}", data.to_json_pretty()?);

    let data: Dereference = dereference(&key, InputMetadata::new(), &resolver).await?;
    println!("> Dereference (key)");
    println!("{}", data.to_json_pretty()?);

    let data: Dereference = dereference(&srv, InputMetadata::new(), &resolver).await?;
    println!("> Dereference (service)");
    println!("{}", data.to_json_pretty()?);

    Ok(())
}

pub struct MemResolver {
    store: Vec<MetaDocument>,
    index: BTreeMap<DID, usize>,
}

impl MemResolver {
    pub fn new() -> Self {
        Self {
            store: Vec::new(),
            index: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, document: DIDDocument) -> Result<()> {
        if let Some(index) = self.index(&document.id) {
            let current: &mut MetaDocument = self.store.get_mut(index).expect("infallible");

            current.data = document;
            current.meta.updated = Some(Timestamp::now());
        } else {
            let did: DID = self.clean(&document.id);
            let idx: usize = self.store.len();

            self.store.push(MetaDocument {
                data: document,
                meta: DocumentMetadata {
                    created: Some(Timestamp::now()),
                    updated: Some(Timestamp::now()),
                    properties: Object::new(),
                },
            });

            self.index.insert(did, idx);
        }

        Ok(())
    }

    pub fn get(&self, did: &DID) -> Option<&MetaDocument> {
        self.index(did).and_then(|index| self.store.get(index))
    }

    pub fn index(&self, did: &DID) -> Option<usize> {
        self.index.get(&self.clean(did)).copied()
    }

    fn clean(&self, did: &DID) -> DID {
        DID {
            method_name: did.method_name.clone(),
            id_segments: did.id_segments.clone(),
            path_segments: None,
            query: None,
            fragment: None,
        }
    }
}

#[async_trait]
impl ResolverMethod for MemResolver {
    fn is_supported(&self, _: &DID) -> bool {
        true
    }

    async fn read(&self, did: &DID, _: InputMetadata) -> core::Result<Option<MetaDocument>> {
        Ok(self.get(did).cloned())
    }
}
