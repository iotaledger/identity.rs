// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum SignatureData {
  #[serde(skip)]
  None,
  #[serde(rename = "jws")]
  Jws(String),
  #[serde(rename = "proofValue")]
  Proof(String),
  #[serde(rename = "signatureValue")]
  Signature(String),
}

impl SignatureData {
  pub const fn is_none(&self) -> bool {
    matches!(self, Self::None)
  }

  pub const fn is_jws(&self) -> bool {
    matches!(self, Self::Jws(_))
  }

  pub const fn is_proof(&self) -> bool {
    matches!(self, Self::Proof(_))
  }

  pub const fn is_signature(&self) -> bool {
    matches!(self, Self::Signature(_))
  }

  pub fn as_str(&self) -> &str {
    match self {
      Self::None => "",
      Self::Jws(inner) => &*inner,
      Self::Proof(inner) => &*inner,
      Self::Signature(inner) => &*inner,
    }
  }

  pub fn try_jws(&self) -> Option<&str> {
    match self {
      Self::None => None,
      Self::Jws(inner) => Some(&*inner),
      Self::Proof(_) => None,
      Self::Signature(_) => None,
    }
  }

  pub fn try_proof(&self) -> Option<&str> {
    match self {
      Self::None => None,
      Self::Jws(_) => None,
      Self::Proof(inner) => Some(&*inner),
      Self::Signature(_) => None,
    }
  }

  pub fn try_signature(&self) -> Option<&str> {
    match self {
      Self::None => None,
      Self::Jws(_) => None,
      Self::Proof(_) => None,
      Self::Signature(inner) => Some(&*inner),
    }
  }
}
