// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::decode_b58;
use identity::core::FromJson;
use identity::crypto::merkle_key::MerkleDigestTag;
use identity::crypto::merkle_key::MerkleKey;
use identity::crypto::merkle_key::Sha256;
use identity::crypto::merkle_tree::Proof;
use identity::crypto::PublicKey;
use identity::crypto::SecretKey;
use identity::did::verifiable;
use identity::did::MethodScope;
use identity::iota::DocumentDiff;
use identity::iota::Error;
use identity::iota::IotaDID;
use identity::iota::IotaDocument;
use identity::iota::IotaVerificationMethod;
use identity::iota::MessageId;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use crate::credential::VerifiableCredential;
use crate::credential::VerifiablePresentation;
use crate::crypto::KeyPair;
use crate::crypto::KeyType;
use crate::service::Service;
use crate::utils::err;
use crate::wasm_did::WasmDID;
use crate::wasm_verification_method::WasmVerificationMethod;

#[wasm_bindgen(inspectable)]
pub struct NewDocument {
  key: KeyPair,
  doc: WasmDocument,
}

#[wasm_bindgen]
impl NewDocument {
  #[wasm_bindgen(getter)]
  pub fn key(&self) -> KeyPair {
    self.key.clone()
  }

  #[wasm_bindgen(getter)]
  pub fn doc(&self) -> WasmDocument {
    self.doc.clone()
  }
}

// =============================================================================
// =============================================================================

#[wasm_bindgen(js_name = Document, inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct WasmDocument(pub(crate) IotaDocument);

#[wasm_bindgen(js_class = Document)]
impl WasmDocument {
  /// Creates a new DID Document from the given KeyPair.
  #[wasm_bindgen(constructor)]
  #[allow(clippy::new_ret_no_self)]
  pub fn new(type_: KeyType, network: Option<String>, tag: Option<String>) -> Result<NewDocument, JsValue> {
    let keypair: KeyPair = KeyPair::new(type_)?;
    let public: &PublicKey = keypair.0.public();

    let did: IotaDID = if let Some(network) = network.as_deref() {
      IotaDID::with_network(public.as_ref(), network).map_err(err)?
    } else {
      IotaDID::new(public.as_ref()).map_err(err)?
    };

    let method: IotaVerificationMethod =
      IotaVerificationMethod::from_did(did, &keypair.0, tag.as_deref()).map_err(err)?;
    let document: IotaDocument = IotaDocument::from_authentication(method).map_err(err)?;

    Ok(NewDocument {
      key: keypair,
      doc: Self(document),
    })
  }

  /// Creates a new DID Document from the given KeyPair and optional network.
  ///
  /// If unspecified, network defaults to the mainnet.
  #[wasm_bindgen(js_name = fromKeyPair)]
  pub fn from_keypair(key: &KeyPair, network: Option<String>) -> Result<WasmDocument, JsValue> {
    let doc = match network {
      Some(net) => IotaDocument::from_keypair_with_network(&key.0, &net),
      None => IotaDocument::from_keypair(&key.0),
    };
    doc.map_err(err).map(Self)
  }

  /// Creates a new DID Document from the given verification [`method`][`Method`].
  #[wasm_bindgen(js_name = fromAuthentication)]
  pub fn from_authentication(method: &WasmVerificationMethod) -> Result<WasmDocument, JsValue> {
    IotaDocument::from_authentication(method.0.clone())
      .map_err(err)
      .map(Self)
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns the DID Document `id`.
  #[wasm_bindgen(getter)]
  pub fn id(&self) -> WasmDID {
    WasmDID(self.0.id().clone())
  }

  /// Returns the DID Document `proof` object.
  #[wasm_bindgen(getter)]
  pub fn proof(&self) -> Result<JsValue, JsValue> {
    match self.0.proof() {
      Some(proof) => JsValue::from_serde(proof).map_err(err),
      None => Ok(JsValue::NULL),
    }
  }

  #[wasm_bindgen(getter = previousMessageId)]
  pub fn previous_message_id(&self) -> String {
    self.0.previous_message_id().to_string()
  }

  #[wasm_bindgen(setter = previousMessageId)]
  pub fn set_previous_message_id(&mut self, value: &str) -> Result<(), JsValue> {
    let message: MessageId = MessageId::from_str(value).map_err(err)?;

    self.0.set_previous_message_id(message);

    Ok(())
  }

  // ===========================================================================
  // Verification Methods
  // ===========================================================================

  #[wasm_bindgen(js_name = insertMethod)]
  pub fn insert_method(&mut self, method: &WasmVerificationMethod, scope: Option<String>) -> Result<bool, JsValue> {
    let scope: MethodScope = scope.unwrap_or_default().parse().map_err(err)?;

    Ok(self.0.insert_method(scope, method.0.clone()))
  }

  #[wasm_bindgen(js_name = removeMethod)]
  pub fn remove_method(&mut self, did: &WasmDID) -> Result<(), JsValue> {
    self.0.remove_method(&did.0).map_err(err)
  }

  #[wasm_bindgen(js_name = insertService)]
  pub fn insert_service(&mut self, service: &Service) -> Result<bool, JsValue> {
    Ok(self.0.insert_service(service.0.clone()))
  }

  #[wasm_bindgen(js_name = removeService)]
  pub fn remove_service(&mut self, did: &WasmDID) -> Result<(), JsValue> {
    self.0.remove_service(&did.0).map_err(err)
  }

  // ===========================================================================
  // Signatures
  // ===========================================================================

  /// Signs the DID Document with the default authentication method.
  #[wasm_bindgen]
  pub fn sign(&mut self, key: &KeyPair) -> Result<(), JsValue> {
    self.0.sign(key.0.secret()).map_err(err)
  }

  /// Verify the signature with the authentication_key
  #[wasm_bindgen]
  pub fn verify(&self) -> bool {
    self.0.verify().is_ok()
  }

  #[wasm_bindgen(js_name = signCredential)]
  pub fn sign_credential(&self, data: &JsValue, args: &JsValue) -> Result<VerifiableCredential, JsValue> {
    let json: JsValue = self.sign_data(data, args)?;
    let data: VerifiableCredential = VerifiableCredential::from_json(&json)?;

    Ok(data)
  }

  #[wasm_bindgen(js_name = signPresentation)]
  pub fn sign_presentation(&self, data: &JsValue, args: &JsValue) -> Result<VerifiablePresentation, JsValue> {
    let json: JsValue = self.sign_data(data, args)?;
    let data: VerifiablePresentation = VerifiablePresentation::from_json(&json)?;

    Ok(data)
  }

  /// Creates a signature for the given `data` with the specified DID Document
  /// Verification Method.
  ///
  /// An additional `proof` property is required if using a Merkle Key
  /// Collection verification Method.
  #[wasm_bindgen(js_name = signData)]
  pub fn sign_data(&self, data: &JsValue, args: &JsValue) -> Result<JsValue, JsValue> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Args {
      MerkleKey {
        method: String,
        public: String,
        secret: String,
        proof: String,
      },
      Default {
        method: String,
        secret: String,
      },
    }

    let mut data: verifiable::Properties = data.into_serde().map_err(err)?;
    let args: Args = args.into_serde().map_err(err)?;

    match args {
      Args::MerkleKey {
        method,
        public,
        secret,
        proof,
      } => {
        let merkle_key: Vec<u8> = self
          .0
          .try_resolve(&*method)
          .and_then(|method| method.key_data().try_decode().map_err(Error::InvalidDoc))
          .map_err(err)?;

        let public: PublicKey = decode_b58(&public).map_err(err).map(Into::into)?;
        let secret: SecretKey = decode_b58(&secret).map_err(err).map(Into::into)?;

        let digest: MerkleDigestTag = MerkleKey::extract_tags(&merkle_key).map_err(err)?.1;
        let proof: Vec<u8> = decode_b58(&proof).map_err(err)?;

        let signer: _ = self.0.signer(&secret).method(&method);

        match digest {
          MerkleDigestTag::SHA256 => match Proof::<Sha256>::decode(&proof) {
            Some(proof) => signer.merkle_key((&public, &proof)).sign(&mut data).map_err(err)?,
            None => return Err("Invalid Public Key Proof".into()),
          },
          _ => return Err("Invalid Merkle Key Digest".into()),
        }
      }
      Args::Default { method, secret } => {
        let secret: SecretKey = decode_b58(&secret).map_err(err).map(Into::into)?;

        self.0.signer(&secret).method(&method).sign(&mut data).map_err(err)?;
      }
    }

    JsValue::from_serde(&data).map_err(err)
  }

  /// Verifies the authenticity of `data` using the target verification method.
  #[wasm_bindgen(js_name = verifyData)]
  pub fn verify_data(&self, data: &JsValue) -> Result<bool, JsValue> {
    let data: verifiable::Properties = data.into_serde().map_err(err)?;
    let result: bool = self.0.verifier().verify(&data).is_ok();

    Ok(result)
  }

  #[wasm_bindgen(js_name = resolveKey)]
  pub fn resolve_key(&mut self, query: &str) -> Result<WasmVerificationMethod, JsValue> {
    Ok(WasmVerificationMethod(self.0.try_resolve(query).map_err(err)?.clone()))
  }

  #[wasm_bindgen(js_name = revokeMerkleKey)]
  pub fn revoke_merkle_key(&mut self, query: &str, index: usize) -> Result<bool, JsValue> {
    let method: &mut IotaVerificationMethod = self
      .0
      .try_resolve_mut(query)
      .and_then(IotaVerificationMethod::try_from_mut)
      .map_err(err)?;

    method.revoke_merkle_key(index).map_err(err)
  }

  // ===========================================================================
  // Diffs
  // ===========================================================================

  /// Generate the difference between two DID Documents and sign it
  #[wasm_bindgen]
  pub fn diff(&self, other: &WasmDocument, message: &str, key: &KeyPair) -> Result<JsValue, JsValue> {
    self
      .0
      .diff(&other.0, MessageId::from_str(message).map_err(err)?, key.0.secret())
      .map_err(err)
      .and_then(|diff| JsValue::from_serde(&diff).map_err(err))
  }

  /// Verifies the `diff` signature and merges the changes into `self`.
  #[wasm_bindgen]
  pub fn merge(&mut self, diff: &str) -> Result<(), JsValue> {
    let diff: DocumentDiff = DocumentDiff::from_json(diff).map_err(err)?;

    self.0.merge(&diff).map_err(err)?;

    Ok(())
  }

  /// Serializes a `Document` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Document` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmDocument, JsValue> {
    json.into_serde().map_err(err).map(Self)
  }
}
