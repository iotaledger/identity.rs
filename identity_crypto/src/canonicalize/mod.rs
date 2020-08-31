use cjson::to_vec;
use identity_core::common::Object;
use serde_json::to_value;

use crate::error::{Error, Result};

pub trait Canonicalize {
  fn canonicalize(object: Object) -> Result<Vec<u8>>;
}

/// Document normalization via canonical JSON
///
/// Ref: http://wiki.laptop.org/go/Canonical_JSON
#[derive(Clone, Copy, Debug)]
pub struct CanonicalJSON;

impl Canonicalize for CanonicalJSON {
  fn canonicalize(object: Object) -> Result<Vec<u8>> {
    let value = to_value(object).map_err(|_| Error::InvalidCanonicalization)?;
    let bytes = to_vec(&value).map_err(|_| Error::InvalidCanonicalization)?;

    Ok(bytes)
  }
}

/// Universal RDF Graph Normalization Algorithm 2012
///
/// Ref: http://json-ld.github.io/normalization/spec/#urgna2012
#[derive(Clone, Copy, Debug)]
pub struct URGNA2012;

impl Canonicalize for URGNA2012 {
  fn canonicalize(_object: Object) -> Result<Vec<u8>> {
    todo!("URGNA2012::canonicalize")
  }
}

/// Universal RDF Dataset Normalization Algorithm 2015 (AKA GCA2015)
///
/// Ref: http://json-ld.github.io/normalization/spec/#introduction
#[derive(Clone, Copy, Debug)]
pub struct URDNA2015;

impl Canonicalize for URDNA2015 {
  fn canonicalize(_object: Object) -> Result<Vec<u8>> {
    todo!("URDNA2015::canonicalize")
  }
}
