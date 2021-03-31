// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::slice::Iter;
use identity_core::convert::ToJson;

use crate::chain::AuthChain;
use crate::chain::DocumentChain;
use crate::did::DocumentDiff;
use crate::did::DID;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::MessageExt;
use crate::tangle::MessageIdExt;
use crate::tangle::MessageIndex;
use crate::tangle::TangleRef;
use iota::Message;
use iota::MessageId;

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DiffChain {
  inner: Vec<DocumentDiff>,
}

impl DiffChain {
  /// Constructs a new `DiffChain` for the given `AuthChain` from a slice of `Message`s.
  pub fn try_from_messages(auth: &AuthChain, messages: &[Message]) -> Result<Self> {
    if messages.is_empty() {
      return Ok(Self::new());
    }

    let did: &DID = auth.current().id();

    let mut index: MessageIndex<DocumentDiff> = messages
      .iter()
      .flat_map(|message| message.try_extract_diff(did))
      .collect();

    let mut this: Self = Self::new();

    while let Some(mut list) = index.remove(DocumentChain::__diff_message_id(auth, &this)) {
      'inner: while let Some(next) = list.pop() {
        if auth.current().verify_data(&next).is_ok() {
          this.inner.push(next);
          break 'inner;
        }
      }
    }

    Ok(this)
  }

  /// Creates a new `DiffChain`.
  pub fn new() -> Self {
    Self { inner: Vec::new() }
  }

  /// Returns the total number of diffs.
  pub fn len(&self) -> usize {
    self.inner.len()
  }

  /// Returns `true` if the `DiffChain` is empty.
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  /// Empties the `DiffChain`, removing all diffs.
  pub fn clear(&mut self) {
    self.inner.clear();
  }

  /// Returns an iterator yielding references to `DocumentDiff`s.
  pub fn iter(&self) -> Iter<'_, DocumentDiff> {
    self.inner.iter()
  }

  /// Returns the `MessageId` of the latest diff if the chain, if any.
  pub fn current_message_id(&self) -> Option<&MessageId> {
    self.inner.last().map(|diff| diff.message_id())
  }

  /// Adds a new diff to the `DiffChain`.
  ///
  /// # Errors
  ///
  /// Fails if the diff signature is invalid or the Tangle message
  /// references within the diff are invalid.
  pub fn try_push(&mut self, auth: &AuthChain, diff: DocumentDiff) -> Result<()> {
    self.check_validity(auth, &diff)?;

    // SAFETY: we performed the necessary validation in `check_validity`.
    unsafe {
      self.push_unchecked(diff);
    }

    Ok(())
  }

  /// Adds a new diff to the `DiffChain` with performing validation checks.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check the validity of
  /// the signature or Tangle references of the `DocumentDiff`.
  pub unsafe fn push_unchecked(&mut self, diff: DocumentDiff) {
    self.inner.push(diff);
  }

  /// Returns `true` if the `DocumentDiff` can be added to the `DiffChain`.
  pub fn is_valid(&self, auth: &AuthChain, diff: &DocumentDiff) -> bool {
    self.check_validity(auth, diff).is_ok()
  }

  /// Checks if the `DocumentDiff` can be added to the `DiffChain`n.
  ///
  /// # Errors
  ///
  /// Fails if the `DocumentDiff` is not a valid addition.
  pub fn check_validity(&self, auth: &AuthChain, diff: &DocumentDiff) -> Result<()> {
    if auth.current().verify_data(diff).is_err() {
      return Err(Error::ChainError {
        error: "Invalid Signature",
      });
    }

    if diff.message_id().is_null() {
      return Err(Error::ChainError {
        error: "Invalid Message Id",
      });
    }

    if diff.previous_message_id().is_null() {
      return Err(Error::ChainError {
        error: "Invalid Previous Message Id",
      });
    }

    if diff.previous_message_id() != DocumentChain::__diff_message_id(auth, self) {
      return Err(Error::ChainError {
        error: "Invalid Previous Message Id",
      });
    }

    Ok(())
  }
}

impl Default for DiffChain {
  fn default() -> Self {
    Self::new()
  }
}

impl Display for DiffChain {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if f.alternate() {
      f.write_str(&self.to_json_pretty().map_err(|_| FmtError)?)
    } else {
      f.write_str(&self.to_json().map_err(|_| FmtError)?)
    }
  }
}

#[cfg(test)]
mod test {
  use crate::chain::AuthChain;
  use crate::chain::DocumentChain;
  use crate::did::Document;
  use crate::did::DocumentDiff;
  use crate::tangle::TangleRef;
  use identity_core::{common::Timestamp, crypto::KeyPair};
  use identity_did::verification::{MethodBuilder, MethodData, MethodRef, MethodType};
  use iota::MessageId;

  #[test]
  fn test_diff_chain() {
    let mut chain: DocumentChain;
    let mut keys: Vec<KeyPair> = Vec::new();

    // =========================================================================
    // Create Initial Document
    // =========================================================================

    {
      let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
      let mut document: Document = Document::from_keypair(&keypair).unwrap();
      document.sign(keypair.secret()).unwrap();
      document.set_message_id(MessageId::new([8; 32]));
      chain = DocumentChain::new(AuthChain::new(document).unwrap());
      keys.push(keypair);
    }

    // =========================================================================
    // Push Auth Chain Update
    // =========================================================================
    {
      let mut new: Document = chain.current().clone();
      let keypair: KeyPair = KeyPair::new_ed25519().unwrap();

      let authentication: MethodRef = MethodBuilder::default()
        .id(chain.id().join("#key-2").unwrap().into())
        .controller(chain.id().clone().into())
        .key_type(MethodType::Ed25519VerificationKey2018)
        .key_data(MethodData::new_b58(keypair.public()))
        .build()
        .map(Into::into)
        .unwrap();

      unsafe {
        new.as_document_mut().authentication_mut().clear();
        new.as_document_mut().authentication_mut().append(authentication.into());
      }

      new.set_updated(Timestamp::now());
      new.set_previous_message_id(*chain.auth_message_id());

      assert!(chain.current().sign_data(&mut new, keys[0].secret()).is_ok());

      keys.push(keypair);
      assert!(chain.try_push_auth(new).is_ok());
    }

    // =========================================================================
    // Push Diff Chain Update
    // =========================================================================
    {
      let new: Document = {
        let mut this: Document = chain.current().clone();
        this.properties_mut().insert("foo".into(), 123.into());
        this.properties_mut().insert("bar".into(), 456.into());
        this.set_updated(Timestamp::now());
        this
      };

      let message_id = *chain.diff_message_id();
      let mut diff: DocumentDiff = chain.current().diff(&new, message_id, keys[1].secret()).unwrap();
      diff.set_message_id(message_id);
      assert!(chain.try_push_diff(diff).is_ok());
    }
  }
}
