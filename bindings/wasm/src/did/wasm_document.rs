// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity::core::decode_b58;
use identity::crypto::merkle_key::MerkleDigestTag;
use identity::crypto::merkle_key::MerkleKey;
use identity::crypto::merkle_key::Sha256;
use identity::crypto::merkle_tree::Proof;
use identity::crypto::PrivateKey;
use identity::crypto::PublicKey;
use identity::did::verifiable::VerifiableProperties;
use identity::iota::Error;
use identity::iota::IotaDocument;
use identity::iota::IotaVerificationMethod;
use identity::iota::MessageId;
use identity::iota::NetworkName;
use wasm_bindgen::prelude::*;

use crate::common::WasmTimestamp;
use crate::credential::WasmCredential;
use crate::credential::WasmPresentation;
use crate::crypto::KeyPair;
use crate::crypto::WasmSignatureOptions;
use crate::did::WasmDID;
use crate::did::WasmDIDUrl;
use crate::did::WasmDiffMessage;
use crate::did::WasmDocumentMetadata;
use crate::did::WasmMethodScope;
use crate::did::WasmVerificationMethod;
use crate::did::WasmVerifierOptions;
use crate::error::Result;
use crate::error::WasmResult;
use crate::service::Service;

// =============================================================================
// =============================================================================

#[wasm_bindgen(js_name = Document, inspectable)]
#[derive(Clone, Debug)]
pub struct WasmDocument(pub(crate) IotaDocument);

#[wasm_bindgen(js_class = Document)]
impl WasmDocument {
  /// Creates a new DID Document from the given `KeyPair`, network, and verification method
  /// fragment name.
  ///
  /// The DID Document will be pre-populated with a single verification method
  /// derived from the provided `KeyPair` embedded as a capability invocation
  /// verification relationship. This method will have the DID URL fragment
  /// `#sign-0` by default and can be easily retrieved with `Document::defaultSigningMethod`.
  ///
  /// NOTE: the generated document is unsigned, see `Document::signSelf`.
  ///
  /// Arguments:
  ///
  /// * keypair: the initial verification method is derived from the public key with this keypair.
  /// * network: Tangle network to use for the DID, default `Network::mainnet`.
  /// * fragment: name of the initial verification method, default "sign-0".
  #[wasm_bindgen(constructor)]
  pub fn new(keypair: &KeyPair, network: Option<String>, fragment: Option<String>) -> Result<WasmDocument> {
    let network_name = network.map(NetworkName::try_from).transpose().wasm_result()?;
    IotaDocument::new_with_options(&keypair.0, network_name, fragment.as_deref())
      .map(Self)
      .wasm_result()
  }

  /// Creates a new DID Document from the given `VerificationMethod`.
  ///
  /// NOTE: the generated document is unsigned, see `Document::signSelf`.
  #[wasm_bindgen(js_name = fromVerificationMethod)]
  pub fn from_verification_method(method: &WasmVerificationMethod) -> Result<WasmDocument> {
    IotaDocument::from_verification_method(method.0.clone())
      .map(Self)
      .wasm_result()
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns the DID Document `id`.
  #[wasm_bindgen(getter)]
  pub fn id(&self) -> WasmDID {
    WasmDID(self.0.id().clone())
  }

  // ===========================================================================
  // Services
  // ===========================================================================

  /// Add a new `Service` to the document.
  #[wasm_bindgen(js_name = insertService)]
  pub fn insert_service(&mut self, service: &Service) -> Result<bool> {
    Ok(self.0.insert_service(service.0.clone()))
  }

  /// Remove a `Service` identified by the given `DIDUrl` from the document.
  #[wasm_bindgen(js_name = removeService)]
  pub fn remove_service(&mut self, did: WasmDIDUrl) -> Result<()> {
    self.0.remove_service(did.0).wasm_result()
  }

  // ===========================================================================
  // Verification Methods
  // ===========================================================================

  /// Adds a new Verification Method to the DID Document.
  #[wasm_bindgen(js_name = insertMethod)]
  pub fn insert_method(&mut self, method: &WasmVerificationMethod, scope: WasmMethodScope) -> Result<()> {
    self.0.insert_method(method.0.clone(), scope.0).wasm_result()?;
    Ok(())
  }

  /// Removes all references to the specified Verification Method.
  #[wasm_bindgen(js_name = removeMethod)]
  pub fn remove_method(&mut self, did: WasmDIDUrl) -> Result<()> {
    self.0.remove_method(did.0).wasm_result()
  }

  /// Returns the first `VerificationMethod` with a capability invocation relationship
  /// capable of signing this DID document.
  ///
  /// Throws an error if no signing method is present.
  #[wasm_bindgen(js_name = defaultSigningMethod)]
  pub fn default_signing_method(&self) -> Result<WasmVerificationMethod> {
    self
      .0
      .default_signing_method()
      .map(Clone::clone)
      .map(WasmVerificationMethod::from)
      .wasm_result()
  }

  /// Returns the first `VerificationMethod` with an `id` property
  /// matching the provided `query`.
  ///
  /// Throws an error if the method is not found.
  #[wasm_bindgen(js_name = resolveMethod)]
  pub fn resolve_method(&mut self, query: &str) -> Result<WasmVerificationMethod> {
    Ok(WasmVerificationMethod(
      self.0.try_resolve_method(query).wasm_result()?.clone(),
    ))
  }

  #[wasm_bindgen(js_name = revokeMerkleKey)]
  pub fn revoke_merkle_key(&mut self, query: &str, index: usize) -> Result<bool> {
    let method: &mut IotaVerificationMethod = self
      .0
      .try_resolve_method_mut(query)
      .and_then(IotaVerificationMethod::try_from_mut)
      .wasm_result()?;

    method.revoke_merkle_key(index).wasm_result()
  }

  // ===========================================================================
  // Signatures
  // ===========================================================================

  /// Signs the DID document with the verification method specified by `method_query`.
  /// The `method_query` may be the full `DIDUrl` of the method or just its fragment,
  /// e.g. "#sign-0".
  ///
  /// NOTE: does not validate whether the private key of the given `key_pair` corresponds to the
  /// verification method. See `Document::verifySelfSigned`.
  #[wasm_bindgen(js_name = signSelf)]
  pub fn sign_self(&mut self, key_pair: &KeyPair, method_query: String) -> Result<()> {
    self.0.sign_self(key_pair.0.private(), &method_query).wasm_result()
  }

  #[wasm_bindgen(js_name = signCredential)]
  pub fn sign_credential(
    &self,
    data: &JsValue,
    args: &JsValue,
    options: WasmSignatureOptions,
  ) -> Result<WasmCredential> {
    let json: JsValue = self.sign_data(data, args, options)?;
    let data: WasmCredential = WasmCredential::from_json(&json)?;

    Ok(data)
  }

  #[wasm_bindgen(js_name = signPresentation)]
  pub fn sign_presentation(
    &self,
    data: &JsValue,
    args: &JsValue,
    options: WasmSignatureOptions,
  ) -> Result<WasmPresentation> {
    let json: JsValue = self.sign_data(data, args, options)?;
    let data: WasmPresentation = WasmPresentation::from_json(&json)?;

    Ok(data)
  }

  /// Creates a signature for the given `data` with the specified DID Document
  /// Verification Method.
  ///
  /// An additional `proof` property is required if using a Merkle Key
  /// Collection verification Method.
  #[wasm_bindgen(js_name = signData)]
  pub fn sign_data(&self, data: &JsValue, args: &JsValue, options: WasmSignatureOptions) -> Result<JsValue> {
    // TODO: clean this up and annotate types if possible.
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Args {
      MerkleKey {
        method: String,
        public: String,
        private: String,
        proof: String,
      },
      Default {
        method: String,
        private: String,
      },
    }

    let mut data: VerifiableProperties = data.into_serde().wasm_result()?;
    let args: Args = args.into_serde().wasm_result()?;

    match args {
      Args::MerkleKey {
        method,
        public,
        private,
        proof,
      } => {
        let merkle_key: Vec<u8> = self
          .0
          .try_resolve_method(&*method)
          .and_then(|method| method.key_data().try_decode().map_err(Error::InvalidDoc))
          .wasm_result()?;

        let public: PublicKey = decode_b58(&public).map(Into::into).wasm_result()?;
        let private: PrivateKey = decode_b58(&private).map(Into::into).wasm_result()?;

        let digest: MerkleDigestTag = MerkleKey::extract_tags(&merkle_key).wasm_result()?.1;
        let proof: Vec<u8> = decode_b58(&proof).wasm_result()?;

        match digest {
          MerkleDigestTag::SHA256 => match Proof::<Sha256>::decode(&proof) {
            Some(proof) => self
              .0
              .signer(&private)
              .method(&method)
              .options(options.0)
              .merkle_key((&public, &proof))
              .sign(&mut data)
              .wasm_result()?,
            None => return Err("Invalid Public Key Proof".into()),
          },
          _ => return Err("Invalid Merkle Key Digest".into()),
        }
      }
      Args::Default { method, private } => {
        let private: PrivateKey = decode_b58(&private).wasm_result().map(Into::into)?;

        self
          .0
          .signer(&private)
          .method(&method)
          .options(options.0)
          .sign(&mut data)
          .wasm_result()?;
      }
    }

    JsValue::from_serde(&data).wasm_result()
  }

  // ===========================================================================
  // Verification
  // ===========================================================================

  /// Verifies the authenticity of `data` using the target verification method.
  #[wasm_bindgen(js_name = verifyData)]
  pub fn verify_data(&self, data: &JsValue, options: WasmVerifierOptions) -> Result<bool> {
    let data: VerifiableProperties = data.into_serde().wasm_result()?;
    Ok(self.0.verify_data(&data, options.0).is_ok())
  }

  /// Verifies that the signature on the DID document `signed` was generated by a valid method from
  /// the `signer` DID document.
  ///
  /// # Errors
  ///
  /// Fails if:
  /// - The signature proof section is missing in the `signed` document.
  /// - The method is not found in the `signer` document.
  /// - An unsupported verification method is used.
  /// - The signature verification operation fails.
  #[wasm_bindgen(js_name = verifyDocument)]
  pub fn verify_document(signed: &WasmDocument, signer: &WasmDocument) -> Result<()> {
    IotaDocument::verify_document(&signed.0, &signer.0).wasm_result()
  }

  /// Verifies whether `document` is a valid root DID document according to the IOTA DID method
  /// specification.
  ///
  /// It must be signed using a verification method with a public key whose BLAKE2b-256 hash matches
  /// the DID tag.
  #[wasm_bindgen(js_name = verifyRootDocument)]
  pub fn verify_root_document(document: &WasmDocument) -> Result<()> {
    IotaDocument::verify_root_document(&document.0).wasm_result()
  }

  // ===========================================================================
  // Diffs
  // ===========================================================================

  /// Generate a `DiffMessage` between two DID Documents and sign it using the specified
  /// `key` and `method`.
  #[wasm_bindgen]
  pub fn diff(&self, other: &WasmDocument, message_id: &str, key: &KeyPair, method: &str) -> Result<WasmDiffMessage> {
    self
      .0
      .diff(
        &other.0,
        MessageId::from_str(message_id).wasm_result()?,
        key.0.private(),
        method,
      )
      .map(WasmDiffMessage::from)
      .wasm_result()
  }

  /// Verifies the signature of the `diff` was created using a capability invocation method
  /// in this DID Document.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used or the verification operation fails.
  #[wasm_bindgen(js_name = verifyDiff)]
  pub fn verify_diff(&self, diff: &WasmDiffMessage) -> Result<()> {
    self.0.verify_diff(&diff.0).wasm_result()
  }

  /// Verifies a `DiffMessage` signature and attempts to merge the changes into `self`.
  #[wasm_bindgen(js_name = mergeDiff)]
  pub fn merge_diff(&mut self, diff: &WasmDiffMessage) -> Result<()> {
    self.0.merge_diff(&diff.0).wasm_result()
  }

  // ===========================================================================
  // Publishing
  // ===========================================================================

  /// Returns the Tangle index of the integration chain for this DID.
  ///
  /// This is simply the tag segment of the `DID`.
  /// E.g.
  /// For a document with DID: did:iota:1234567890abcdefghijklmnopqrstuvxyzABCDEFGHI,
  /// `doc.integration_index()` == "1234567890abcdefghijklmnopqrstuvxyzABCDEFGHI"
  #[wasm_bindgen(js_name = integrationIndex)]
  pub fn integration_index(&self) -> String {
    self.0.integration_index().to_owned()
  }

  /// Returns the Tangle index of the DID diff chain. This should only be called on documents
  /// published on the integration chain.
  ///
  /// This is the Base58-btc encoded SHA-256 digest of the hex-encoded message id.
  #[wasm_bindgen(js_name = diffIndex)]
  pub fn diff_index(message_id: &str) -> Result<String> {
    let message_id = MessageId::from_str(message_id).wasm_result()?;
    IotaDocument::diff_index(&message_id).wasm_result()
  }

  // ===========================================================================
  // Metadata
  // ===========================================================================

  /// Returns the metadata associated with this document.
  ///
  /// NOTE: clones the data. Use the `metadataCreated`, `metadataUpdated`,
  /// `metadataPreviousMessageId`, `metadataProof` properties instead.
  #[wasm_bindgen(getter)]
  pub fn metadata(&self) -> WasmDocumentMetadata {
    WasmDocumentMetadata::from(self.0.metadata.clone())
  }

  /// Returns the timestamp of when the DID document was created.
  #[wasm_bindgen(getter = metadataCreated)]
  pub fn metadata_created(&self) -> WasmTimestamp {
    WasmTimestamp::from(self.0.metadata.created)
  }

  /// Sets the timestamp of when the DID document was created.
  #[wasm_bindgen(setter = metadataCreated)]
  pub fn set_metadata_created(&mut self, timestamp: WasmTimestamp) {
    self.0.metadata.created = timestamp.0;
  }

  /// Returns the timestamp of the last DID document update.
  #[wasm_bindgen(getter = metadataUpdated)]
  pub fn metadata_updated(&self) -> WasmTimestamp {
    WasmTimestamp::from(self.0.metadata.updated)
  }

  /// Sets the timestamp of the last DID document update.
  #[wasm_bindgen(setter = metadataUpdated)]
  pub fn set_metadata_updated(&mut self, timestamp: WasmTimestamp) {
    self.0.metadata.updated = timestamp.0;
  }

  /// Returns the previous integration chain message id.
  #[wasm_bindgen(getter = metadataPreviousMessageId)]
  pub fn metadata_previous_message_id(&self) -> String {
    self.0.metadata.previous_message_id.to_string()
  }

  /// Sets the previous integration chain message id.
  #[wasm_bindgen(setter = metadataPreviousMessageId)]
  pub fn set_metadata_previous_message_id(&mut self, value: &str) -> Result<()> {
    let message_id: MessageId = MessageId::from_str(value).wasm_result()?;
    self.0.metadata.previous_message_id = message_id;
    Ok(())
  }

  /// Returns the `proof` object.
  #[wasm_bindgen(getter = metadataProof)]
  pub fn metadata_proof(&self) -> Result<JsValue> {
    // TODO: implement proper bindings for the proof
    match &self.0.metadata.proof {
      Some(proof) => JsValue::from_serde(proof).wasm_result(),
      None => Ok(JsValue::NULL),
    }
  }

  // ===========================================================================
  // JSON
  // ===========================================================================

  /// Serializes a `Document` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `Document` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmDocument> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl From<IotaDocument> for WasmDocument {
  fn from(document: IotaDocument) -> Self {
    Self(document)
  }
}
