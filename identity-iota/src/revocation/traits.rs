// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

/// An interface for a revocation method.
///
/// A [`RevocationMethod`] interface is used for checking if a credential has been revoked, for revoking credentials,
/// and for performing commun operations such as serialization and deserialization
pub trait RevocationMethod<'a>: Serialize + Deserialize<'a> {
  type Item;
  /// Returns the name of the revocation method.
  fn name() -> &'static str;

  // Returns the name of the property that contains the index of the credential to be checked.
  fn credential_list_index_property() -> &'static str;

  /// Creates a new revocation method of type [`Self::Item`].
  fn new() -> Self::Item;

  /// Returns `true` if the credential at the given `index` is revoked.
  fn is_revoked(&self, index: u32) -> bool;

  /// Revokes the credential at the given `index`.
  fn revoke(&mut self, index: u32) -> bool;

  /// The credential at the given `index` will be set to valid.
  fn undo_revocation(&mut self, index: u32) -> bool;
}
