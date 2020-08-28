use chrono::{DateTime, Utc};
use std::{
  ops::{Deref, DerefMut},
  convert::TryFrom
};

use crate::error::Error;

type Inner = DateTime<Utc>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Timestamp(Inner);

impl Timestamp {
  pub fn into_inner(self) -> Inner {
    self.0
  }
}

impl Default for Timestamp {
  fn default() -> Self {
    Self(Utc::now())
  }
}

impl From<Inner> for Timestamp {
  fn from(other: Inner) -> Self {
    Self(other)
  }
}

impl From<Timestamp> for Inner {
  fn from(other: Timestamp) -> Self {
    other.into_inner()
  }
}

impl Deref for Timestamp {
  type Target = Inner;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl TryFrom<&'_ str> for Timestamp {
  type Error = Error;

  fn try_from(string: &'_ str) -> Result<Self, Self::Error> {
    match DateTime::parse_from_rfc3339(string) {
      Ok(datetime) => Ok(Self(datetime.into())),
      Err(error) => Err(Error::InvalidTimestamp(error)),
    }
  }
}
