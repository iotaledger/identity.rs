// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

use crate::error::Error;
use crate::error::Result;
use crate::types::Key;

const DEFAULT_INDEX: u32 = 1;
const DEFAULT_IDENT: &str = "Identity";

pub trait Metadata {
  fn tag(&self) -> &str;
  fn ident(&self) -> &str;
  fn index(&self) -> u32;
}

#[derive(Clone)]
pub struct MetadataList<T> {
  data: Vec<T>,
}

impl<T> MetadataList<T> {
  pub fn new() -> Self {
    Self { data: Vec::new() }
  }

  pub fn as_slice(&self) -> &[T] {
    &self.data
  }
}

impl<T: Metadata> MetadataList<T> {
  pub fn find<'a, K>(&self, id: K) -> Result<&T>
  where
    K: Into<Key<'a>>,
  {
    match id.into() {
      Key::DID(did) => self.find_by_id(did.tag()),
      Key::Ident(ident) => self.find_by_ident(ident),
      Key::Index(index) => self.find_by_index(index),
    }
  }

  pub fn find_id<'a, K>(&self, id: K) -> Result<&[u8]>
  where
    K: Into<Key<'a>>,
  {
    self.find(id).map(|meta| meta.tag().as_bytes())
  }

  pub fn find_by_id(&self, id: &str) -> Result<&T> {
    self
      .data
      .iter()
      .find(|meta| meta.tag() == id)
      .ok_or(Error::MetadataNotFound)
  }

  pub fn find_by_index(&self, index: u32) -> Result<&T> {
    self
      .data
      .iter()
      .find(|meta| meta.index() == index)
      .ok_or(Error::MetadataNotFound)
  }

  pub fn find_by_ident(&self, ident: &str) -> Result<&T> {
    self
      .data
      .iter()
      .find(|meta| meta.ident() == ident)
      .ok_or(Error::MetadataNotFound)
  }

  pub(crate) fn push(&mut self, data: T) {
    self.data.push(data);
  }

  pub(crate) fn load(&mut self, data: &mut Vec<T>) {
    self.data.clear();
    self.data.append(data);
  }

  pub(crate) fn remove(&mut self, index: u32) -> Result<T> {
    self
      .data
      .iter()
      .position(|meta| meta.index() == index)
      .map(|index| self.data.remove(index))
      .ok_or(Error::MetadataNotFound)
  }

  pub(crate) fn generate_next_index(&self) -> u32 {
    self
      .data
      .iter()
      .map(|meta| meta.index())
      .max()
      .map(|index| index + 1)
      .unwrap_or(DEFAULT_INDEX)
  }

  pub(crate) fn generate_next_ident(&self, index: u32, base: Option<&str>) -> String {
    match base {
      Some(ident) => Ident::generate(self, ident),
      None => Ident::generate(self, &format!("{} {}", DEFAULT_IDENT, index)),
    }
  }
}

impl<T> Debug for MetadataList<T>
where
  T: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_set().entries(self.data.iter()).finish()
  }
}

// =============================================================================
// =============================================================================

struct Ident<'a> {
  ident: &'a str,
  index: u32,
  buffer: Option<String>,
}

impl<'a> Ident<'a> {
  fn generate<T>(metadata: &MetadataList<T>, ident: &'a str) -> String
  where
    T: Metadata,
  {
    let mut this: Self = Self::new(ident);

    while metadata.data.iter().any(|meta| meta.ident() == this.get()) {
      this.index += 1;
    }

    this.into_string()
  }

  fn new(ident: &'a str) -> Self {
    Self {
      ident,
      index: 0,
      buffer: None,
    }
  }

  fn fill(&mut self) {
    let mut buffer: &mut String = self.buffer.get_or_insert_with(String::new);

    buffer.push_str(self.ident);
    buffer.push_str(" (");
    itoa::fmt(&mut buffer, self.index).unwrap();
    buffer.push(')');
  }

  fn get(&mut self) -> &str {
    if self.index == 0 {
      self.ident
    } else {
      self.fill();
      self.buffer.as_deref().unwrap_or_default()
    }
  }

  fn into_string(self) -> String {
    if self.index == 0 {
      self.ident.to_string()
    } else {
      self.buffer.unwrap_or_default()
    }
  }
}
