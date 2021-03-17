// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::iter::once;

use crate::types::CredentialMetadata;
use crate::types::Identity;
use crate::types::IdentityMetadata;
use crate::types::PresentationMetadata;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
#[rustfmt::skip]
pub enum ResourceType {
  Identity         = 0x00,
  IdentityMeta     = 0x01,
  IdentityDiff     = 0x02,
  IdentityDiffMeta = 0x03,
  Credential       = 0x04,
  CredentialMeta   = 0x05,
  Presentation     = 0x06,
  PresentationMeta = 0x07,
}

// =============================================================================
// =============================================================================

#[derive(Clone, Copy, Debug)]
pub struct ResourceId<'a> {
  type_: ResourceType,
  data: &'a [u8],
}

impl<'a> ResourceId<'a> {
  pub fn new_identity(tag: &'a str) -> Self {
    Self::new(ResourceType::Identity, tag.as_bytes())
  }

  pub fn new_identity_meta(data: &'a IdentityMetadata) -> Self {
    Self::new(ResourceType::IdentityMeta, data.id().as_bytes())
  }

  pub fn new_credential(tag: &'a str) -> Self {
    Self::new(ResourceType::Credential, tag.as_bytes())
  }

  pub fn new_credential_meta(data: &'a CredentialMetadata) -> Self {
    Self::new(ResourceType::CredentialMeta, data.id().as_bytes())
  }

  pub fn new_presentation(tag: &'a str) -> Self {
    Self::new(ResourceType::Presentation, tag.as_bytes())
  }

  pub fn new_presentation_meta(data: &'a PresentationMetadata) -> Self {
    Self::new(ResourceType::PresentationMeta, data.id().as_bytes())
  }

  pub const fn new(type_: ResourceType, data: &'a [u8]) -> Self {
    Self { type_, data }
  }

  pub const fn type_(&self) -> ResourceType {
    self.type_
  }

  pub const fn data(&self) -> &[u8] {
    self.data
  }

  pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
    once(self.type_ as u8).chain(self.data.iter().copied())
  }

  pub fn to_vec(&self) -> Vec<u8> {
    self.iter().collect()
  }
}
