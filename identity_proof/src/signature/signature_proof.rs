use identity_core::common::{Object, SerdeInto};
use identity_crypto::{self as crypto, Error, Proof, PublicKey, SecretKey};
use std::marker::PhantomData;

use crate::{
    document::LinkedDataDocument,
    error::Result,
    signature::{LinkedDataSignature, SignatureOptions, SignatureSuite},
};

/// A wrapper around a generic `SignatureSuite` and associated `SignatureOptions`
/// that can be used as a `Proof` implementation.
pub struct SignatureProof<'a, T> {
    suite: T,
    options: Option<SignatureOptions>,
    marker: PhantomData<&'a ()>,
}

impl<'a, T> SignatureProof<'a, T> {
    /// Create a new `SignatureProof`.
    pub const fn new(suite: T) -> Self {
        Self {
            suite,
            options: None,
            marker: PhantomData,
        }
    }

    /// Create a new `SignatureProof` with the provided `SignatureOptions`.
    pub const fn with_options(suite: T, options: SignatureOptions) -> Self {
        Self {
            suite,
            options: Some(options),
            marker: PhantomData,
        }
    }

    pub fn create_proof(
        &self,
        document: &(dyn LinkedDataDocument + 'a),
        secret: &SecretKey,
    ) -> Result<LinkedDataSignature>
    where
        T: SignatureSuite,
    {
        let options: SignatureOptions = self.options.clone().unwrap_or_default();

        self.suite.create_proof(document, options, secret)
    }

    pub fn verify_proof(
        &self,
        document: &(dyn LinkedDataDocument + 'a),
        proof: &LinkedDataSignature,
        public: &PublicKey,
    ) -> Result<bool>
    where
        T: SignatureSuite,
    {
        self.suite.verify_proof(document, proof, public)
    }
}

impl<'a, T> Proof for SignatureProof<'a, T>
where
    T: SignatureSuite,
{
    type Document = dyn LinkedDataDocument + 'a;
    type Output = Object;

    fn create(&self, document: &Self::Document, secret: &SecretKey) -> crypto::Result<Self::Output> {
        self.create_proof(document, secret)
            .and_then(|proof| Ok(proof.serde_into()?))
            .map_err(|error| Error::CreateProof(error.into()))
    }

    fn verify(&self, document: &Self::Document, proof: &Self::Output, public: &PublicKey) -> crypto::Result<bool> {
        proof
            .serde_into()
            .map_err(|error| Error::CreateProof(error.into()))
            .and_then(|proof| {
                self.verify_proof(document, &proof, public)
                    .map_err(|error| Error::CreateProof(error.into()))
            })
    }
}
