use identity_core::key::KeyIndex;
use serde::Serialize;

use crate::{error::Result, signature::LdSignature};

pub trait LdDocument: Serialize {
    fn verification_method(&self) -> Option<&str>;

    fn resolve_key(&self, index: KeyIndex) -> Result<Vec<u8>>;

    fn set_proof(&mut self, value: LdSignature) -> Result<()>;

    fn set_signature(&mut self, value: String) -> Result<()>;
}

pub trait HasProof {
    fn proof(&self) -> &LdSignature;

    fn proof_mut(&mut self) -> &mut LdSignature;
}
