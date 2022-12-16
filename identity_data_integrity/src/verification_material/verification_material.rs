use serde::Deserialize;
use serde::Serialize;

use crate::verification_material::Multikey;
use crate::verification_material::PublicKeyMultibase;

#[non_exhaustive]
/// An enum of supported verification material formats.
///
/// The eventual goal is to represent all formats listed in the
/// [data integrity specification](https://w3c.github.io/vc-data-integrity/#verification-material).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum VerificationMaterial {
  PublicKeyMultibase(PublicKeyMultibase),
  Multikey(Multikey),
}
