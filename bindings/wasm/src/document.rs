// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::decode_b58;
use identity::core::encode_b58;
use identity::core::FromJson;
use identity::crypto::merkle_key::MerkleKey;
use identity::crypto::merkle_key::MerkleSignature;
use identity::crypto::merkle_key::MerkleTag;
use identity::crypto::merkle_key::Sha256;
use identity::crypto::merkle_tree::Hash;
use identity::crypto::JcsEd25519Signature2020 as Ed25519;
use identity::crypto::PublicKey;
use identity::crypto::SecretKey;
use identity::did::verifiable;
use identity::did::verifiable::Public;
use identity::did::verifiable::Secret;
use identity::did::MethodIdent;
use identity::did::MethodScope;
use identity::did::MethodWrap;
use identity::iota::Document as IotaDocument;
use identity::iota::DocumentDiff;
use identity::iota::Method as IotaMethod;
use wasm_bindgen::prelude::*;

use crate::crypto::Digest;
use crate::crypto::KeyCollection;
use crate::crypto::KeyPair;
use crate::crypto::KeyType;
use crate::did::DID;
use crate::method::Method;
use crate::utils::err;

#[wasm_bindgen(inspectable)]
pub struct NewDocument {
  key: KeyPair,
  doc: Document,
}

#[wasm_bindgen]
impl NewDocument {
  #[wasm_bindgen(getter)]
  pub fn key(&self) -> KeyPair {
    self.key.clone()
  }

  #[wasm_bindgen(getter)]
  pub fn doc(&self) -> Document {
    self.doc.clone()
  }
}

// =============================================================================
// =============================================================================

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct Document(pub(crate) IotaDocument);

impl Document {
  pub const AUTH_QUERY: (usize, MethodScope) = IotaDocument::AUTH_QUERY;
}

#[wasm_bindgen]
impl Document {
  /// Creates a new DID Document from the given KeyPair.
  #[wasm_bindgen(constructor)]
  pub fn new(type_: KeyType, tag: Option<String>) -> Result<NewDocument, JsValue> {
    let key: KeyPair = KeyPair::new(type_)?;
    let method: IotaMethod = IotaMethod::from_keypair(&key.0, tag.as_deref()).map_err(err)?;
    let document: IotaDocument = IotaDocument::from_authentication(method).map_err(err)?;

    Ok(NewDocument {
      key,
      doc: Self(document),
    })
  }

  /// Creates a new DID Document from the given verification [`method`][`Method`].
  #[wasm_bindgen(js_name = fromAuthentication)]
  pub fn from_authentication(method: Method) -> Result<Document, JsValue> {
    IotaDocument::from_authentication(method.0).map_err(err).map(Self)
  }

  /// Returns the DID Document `id`.
  #[wasm_bindgen(getter)]
  pub fn id(&self) -> DID {
    DID(self.0.id().clone())
  }

  /// Returns the DID Document `proof` object.
  #[wasm_bindgen(getter)]
  pub fn proof(&self) -> Result<JsValue, JsValue> {
    match self.0.proof() {
      Some(proof) => JsValue::from_serde(proof).map_err(err),
      None => Ok(JsValue::NULL),
    }
  }

  /// Signs the DID Document with the default authentication method.
  #[wasm_bindgen]
  pub fn sign(&mut self, key: &KeyPair) -> Result<JsValue, JsValue> {
    self.0.sign(key.0.secret()).map_err(err).map(|_| JsValue::NULL)
  }

  /// Verify the signature with the authentication_key
  #[wasm_bindgen]
  pub fn verify(&self) -> JsValue {
    self.0.verify().is_ok().into()
  }

  /// Creates a Merkle Key Collection signature for the given `data` with the
  /// DID Document verification method identified by `method`.
  ///
  /// A key collection (`keys`) is required and the keypair at `index` is
  /// used for signing.
  #[wasm_bindgen(js_name = signMerkleKey)]
  pub fn sign_merkle_key(
    &self,
    data: &JsValue,
    keys: &KeyCollection,
    method: &str,
    index: usize,
  ) -> Result<JsValue, JsValue> {
    let mut data: verifiable::Properties = data.into_serde().map_err(err)?;

    // Resolve the verification method and determine the digest algorithm
    let digest: MerkleTag = {
      let method: MethodWrap<'_> = self.0.try_resolve(method).map_err(err)?;
      let public: Vec<u8> = method.key_data().try_decode().map_err(err)?;
      let tags: (MerkleTag, MerkleTag) = MerkleKey::extract_tags(&public).map_err(err)?;

      // Ensure the signature algorithm matches the key collection
      if tags.0 != keys.0.type_().tag() {
        return Err("Invalid Merkle Key Algorithm".into());
      }

      tags.1
    };

    // Extract the secret key from the collection.
    let secret: &SecretKey = match keys.0.secret(index) {
      Some(secret) => secret,
      None => return Err("Invalid Secret Key Index".into()),
    };

    match digest {
      MerkleTag::SHA256 => match keys.0.merkle_proof::<Sha256>(index) {
        Some(proof) => {
          self
            .0
            .sign_that(&mut data, method, Secret::with_merkle_proof(secret.as_ref(), &proof))
            .map_err(err)?;
        }
        None => return Err("Invalid Public Key Proof".into()),
      },
      _ => return Err("Invalid Merkle Key Digest".into()),
    }

    JsValue::from_serde(&data).map_err(err)
  }

  /// Verifies the authenticity of `data` using the given Merkle Key Collection
  /// target public key.
  ///
  /// The target public key is expected to be a Base58-encoded string.
  #[wasm_bindgen(js_name = verifyMerkleKey)]
  pub fn verify_merkle_key(&self, data: &JsValue, target: String) -> Result<JsValue, JsValue> {
    let data: verifiable::Properties = data.into_serde().map_err(err)?;

    let public: PublicKey = decode_b58(&target).map_err(err).map(Into::into)?;
    let public: Public<'_> = Public::with_merkle_target(public.as_ref());

    self.0.verify_that(&data, public).map_err(err)?;

    Ok(JsValue::TRUE)
  }

  /// Generate the difference between two DID Documents and sign it
  #[wasm_bindgen]
  pub fn diff(&self, other: &Document, message: String, key: &KeyPair) -> Result<JsValue, JsValue> {
    self
      .0
      .diff(&other.0, message.into(), key.0.secret())
      .map_err(err)
      .and_then(|diff| JsValue::from_serde(&diff).map_err(err))
  }

  /// Verifies the `diff` signature and merges the changes into `self`.
  #[wasm_bindgen]
  pub fn merge(&mut self, diff: &str) -> Result<JsValue, JsValue> {
    let diff: DocumentDiff = DocumentDiff::from_json(diff).map_err(err)?;

    self.0.merge(&diff).map_err(err)?;

    Ok(JsValue::NULL)
  }

  #[wasm_bindgen(js_name = insertMethod)]
  pub fn insert_method(&mut self, method: &Method, scope: Option<String>) -> Result<JsValue, JsValue> {
    let scope: MethodScope = scope
      .map(|scope| scope.parse::<MethodScope>())
      .transpose()
      .map_err(err)?
      .unwrap_or(MethodScope::None);

    self
      .0
      .insert_method(scope, method.0.clone())
      .map_err(err)
      .map(Into::into)
  }

  #[wasm_bindgen(js_name = removeMethod)]
  pub fn remove_method(&mut self, did: &DID) -> Result<JsValue, JsValue> {
    self.0.remove_method(&did.0).map_err(err).map(|_| JsValue::NULL)
  }

  /// Creates a Merkle Key Collection public key value for the given key
  /// collection instance.
  ///
  /// The public key value will be encoded using Base58 encoding.
  #[wasm_bindgen(js_name = encodeMerkleKey)]
  pub fn encode_merkle_key(digest: Digest, keys: &KeyCollection) -> Result<JsValue, JsValue> {
    match (keys.0.type_().into(), digest) {
      (KeyType::Ed25519, Digest::Sha256) => {
        let root: Hash<Sha256> = keys.0.merkle_root();
        let data: Vec<u8> = MerkleKey::encode_key::<Ed25519, Sha256>(&Ed25519, &root);

        Ok(encode_b58(&data).into())
      }
    }
  }

  #[wasm_bindgen(js_name = resolveKey)]
  pub fn resolve_key(&mut self, ident: JsValue, scope: Option<String>) -> Result<Method, JsValue> {
    let borrow: String;

    let ident: MethodIdent = if let Some(number) = ident.as_f64() {
      number.to_string().parse().map_err(err).map(MethodIdent::Index)?
    } else if let Some(ident) = ident.as_string() {
      borrow = ident;
      MethodIdent::Ident(&borrow)
    } else {
      return Err("Invalid Verification Method Identifier".into());
    };

    let scope: MethodScope = scope
      .map(|scope| scope.parse::<MethodScope>())
      .transpose()
      .map_err(err)?
      .unwrap_or(MethodScope::None);

    self
      .0
      .resolve((ident, scope))
      .map(|wrap| wrap.into_method().clone())
      .ok_or_else(|| "Verification Method Not Found".into())
      .and_then(|method| IotaMethod::try_from_core(method).map_err(err))
      .map(Method)
  }

  /// Serializes a `Document` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Document` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<Document, JsValue> {
    json.into_serde().map_err(err).map(Self)
  }
}
