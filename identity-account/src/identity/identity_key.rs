// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::IotaDID;

use crate::identity::IdentityId;
use crate::identity::IdentityTag;

type Item<'a> = (&'a IdentityTag, &'a IdentityId);

pub trait IdentityKey {
  fn equals(&self, tag: &IdentityTag, id: IdentityId) -> bool;

  fn scan<'a, I: Iterator<Item = Item<'a>>>(&self, mut iter: I) -> Option<IdentityId> {
    iter.find(|(tag, id)| self.equals(tag, **id)).map(|(_, id)| *id)
  }
}

impl<'a, T> IdentityKey for &'a T
where
  T: IdentityKey + ?Sized,
{
  fn equals(&self, tag: &IdentityTag, id: IdentityId) -> bool {
    (**self).equals(tag, id)
  }
}

impl IdentityKey for IotaDID {
  fn equals(&self, tag: &IdentityTag, _: IdentityId) -> bool {
    tag.method_id() == self.method_id()
  }
}

impl IdentityKey for str {
  fn equals(&self, tag: &IdentityTag, id: IdentityId) -> bool {
    tag.fullname(id).as_ref() == self
  }
}

impl IdentityKey for String {
  fn equals(&self, tag: &IdentityTag, id: IdentityId) -> bool {
    self[..].equals(tag, id)
  }
}

impl IdentityKey for IdentityId {
  fn equals(&self, _: &IdentityTag, id: IdentityId) -> bool {
    id == *self
  }

  fn scan<'a, I: Iterator<Item = Item<'a>>>(&self, _: I) -> Option<IdentityId> {
    Some(*self)
  }
}
