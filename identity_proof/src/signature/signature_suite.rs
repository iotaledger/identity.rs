use anyhow::anyhow;
use identity_core::common::{Object, Timestamp};
use identity_crypto::{KeyGen, PublicKey, SecretKey, Sign, Verify};

use crate::{
    canonicalize::{CanonicalJson, Canonicalize},
    document::LinkedDataDocument,
    error::{Error, Result},
    signature::{LinkedDataSignature, SignatureData, SignatureOptions, SignatureValue},
    utils::{decode_b64, encode_b64},
};

const DEFAULT_PURPOSE: &str = "assertionMethod";

/// A trait for implementations of linked data signature suites.
///
/// Ref: https://w3c-ccg.github.io/ld-proofs/#linked-data-signatures
pub trait SignatureSuite: KeyGen + Sign + Verify {
    /// Returns a unique identifier for this signature suite.
    ///
    /// Note: This SHOULD be a standard value.
    /// Ref: https://w3c-ccg.github.io/ld-cryptosuite-registry/
    fn signature(&self) -> &'static str;

    /// The message digest algorithm for the `SignatureSuite` implementation.
    ///
    /// Ref: https://w3c-ccg.github.io/ld-proofs/#dfn-message-digest-algorithm
    fn digest(&self, message: &[u8]) -> Result<Vec<u8>>;

    /// The service used to handle document normalization.
    fn canonicalizer(&self) -> &dyn Canonicalize {
        &CanonicalJson
    }

    /// Normalizes a converted document with the configured service.
    fn canonicalize(&self, object: &Object) -> Result<Vec<u8>> {
        self.canonicalizer().canonicalize(object)
    }

    /// Encodes the signature as a `String`.
    fn encode_signature(&self, signature: Vec<u8>) -> String {
        encode_b64(&signature)
    }

    /// Decodes a `String`-encoded signature.
    fn decode_signature(&self, signature: &LinkedDataSignature) -> Result<Vec<u8>> {
        decode_b64(signature.proof())
    }

    /// Creates a `SignatureValue` from a raw String.
    fn to_signature_value(&self, signature: String) -> SignatureValue {
        SignatureValue::Proof(signature)
    }

    /// Creates a `LinkedDataSignature` with the given `document`, `options`,
    /// and `secret` key.
    ///
    /// The `LinkedDataSignature` can be serialized and later verified by a
    /// receiving party.
    fn create_proof(
        &self,
        document: &dyn LinkedDataDocument,
        options: SignatureOptions,
        secret: &SecretKey,
    ) -> Result<LinkedDataSignature> {
        let options: SignatureOptions = self.preprocess_options(options);
        let data: Vec<u8> = self.create_verify_hash(document, &options)?;
        let signature: Vec<u8> = <Self as Sign>::sign(self, &data, secret)?;

        self.to_proof(signature, options)
    }

    /// Verifies a `LinkedDataSignature` with the given `document` and `public` key.
    fn verify_proof(
        &self,
        document: &dyn LinkedDataDocument,
        proof: &LinkedDataSignature,
        public: &PublicKey,
    ) -> Result<bool> {
        let options: SignatureOptions = proof.to_options();
        let data: Vec<u8> = self.create_verify_hash(document, &options)?;
        let signature: Vec<u8> = self.decode_signature(&proof)?;

        <Self as Verify>::verify(self, &data, &signature, public).map_err(Into::into)
    }

    /// Creates a `LinkedDataSignature` for the document signature and options
    fn to_proof(&self, signature: Vec<u8>, options: SignatureOptions) -> Result<LinkedDataSignature> {
        Ok(LinkedDataSignature {
            proof_type: self.signature().into(),
            created: options.created.expect("infallible"),
            purpose: options.purpose.unwrap_or_else(|| DEFAULT_PURPOSE.into()),
            domain: options.domain,
            nonce: options.nonce,
            data: SignatureData {
                value: self.to_signature_value(self.encode_signature(signature)),
                properties: options.properties,
            },
        })
    }

    /// Utility function to create a "verify hash" according to the LD proofs
    /// standard.
    ///
    /// Ref: https://w3c-ccg.github.io/ld-proofs/#create-verify-hash-algorithm
    fn create_verify_hash(
        &self,
        document: &dyn LinkedDataDocument,
        options: &dyn LinkedDataDocument,
    ) -> Result<Vec<u8>> {
        let mut document_digest: Vec<u8> = self.to_hash(document, |document| {
            self.preprocess_document(document, self.require_document_fields(), self.exclude_document_fields())
        })?;

        let mut options_digest: Vec<u8> = self.to_hash(options, |options| {
            self.preprocess_document(options, self.require_option_fields(), self.exclude_option_fields())
        })?;

        options_digest.append(&mut document_digest);

        Ok(options_digest)
    }

    /// Pre-process, canonicalize, and hash the given document.
    fn to_hash(
        &self,
        document: &dyn LinkedDataDocument,
        preprocess: impl Fn(Object) -> Result<Object>,
    ) -> Result<Vec<u8>> {
        document
            .to_object()
            .and_then(|object| preprocess(object))
            .and_then(|object| self.canonicalize(&object))
            .and_then(|object| self.digest(&object))
    }

    /// Process documents in preparation for signing. This may include
    /// validating or removing certain fields from the document.
    fn preprocess_document(
        &self,
        mut object: Object,
        required: &'static [&'static str],
        excluded: &'static [&'static str],
    ) -> Result<Object> {
        for key in required {
            if !object.contains_key(*key) {
                return Err(Error::PreProcess(anyhow!("missing `{}`", key)));
            }
        }

        for key in excluded {
            object.remove(*key);
        }

        Ok(object)
    }

    /// Process `SignatureOptions` in preparation for signing.
    fn preprocess_options(&self, mut options: SignatureOptions) -> SignatureOptions {
        self.set_default_timestamp(&mut options);
        options
    }

    /// Sets a timestamp in the `SignatureOptions` if not already set.
    fn set_default_timestamp(&self, options: &mut SignatureOptions) {
        if options.created.is_none() {
            options.created = Some(Timestamp::now());
        }
    }

    /// Fields stripped from the serialized proof document before normalization.
    fn exclude_document_fields(&self) -> &'static [&'static str] {
        &["proof"]
    }

    /// Fields required in the serialized proof document before normalization.
    fn require_document_fields(&self) -> &'static [&'static str] {
        &[]
    }

    /// Fields stripped from the serialized proof options before normalization.
    fn exclude_option_fields(&self) -> &'static [&'static str] {
        &["jws", "proofValue", "signatureValue"]
    }

    /// Fields required in the serialized proof options before normalization.
    fn require_option_fields(&self) -> &'static [&'static str] {
        &["created"]
    }
}
