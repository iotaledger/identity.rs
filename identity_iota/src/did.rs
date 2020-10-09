use bs58::encode;
use identity_core::did::DID;
use iota::transaction::bundled::Address;

use crate::{
    error::{Error, Result},
    utils::{create_address_from_trits, utf8_to_trytes},
};

pub fn method_id(did: &DID) -> Result<&str> {
    did.id_segments
        .last()
        .map(|string| string.as_str())
        .ok_or_else(|| Error::InvalidMethodId)
}

/// Creates an 81 Trytes IOTA address from the DID
pub fn create_address_hash(did: &DID) -> Result<String> {
    let digest: &[u8] = &Blake2b256::digest(method_id(did)?.as_bytes());
    let encoded: String = encode(digest).into_string();
    let mut trytes: String = utf8_to_trytes(&encoded);

    trytes.truncate(iota_constants::HASH_TRYTES_SIZE);

    Ok(trytes)
}

pub fn create_address(did: &DID) -> Result<Address> {
    create_address_hash(did).and_then(create_address_from_trits)
}

// =============================================================================
// TODO: Move to crypto.rs
// =============================================================================

use blake2::digest::{
    self,
    consts::{U128, U32},
    generic_array::GenericArray,
    Digest as _, VariableOutput as _,
};

#[derive(Clone, Debug)]
pub struct Blake2b256(blake2::VarBlake2b);

impl Default for Blake2b256 {
    fn default() -> Self {
        Self(blake2::VarBlake2b::new(32).unwrap())
    }
}

impl digest::Update for Blake2b256 {
    fn update(&mut self, data: impl AsRef<[u8]>) {
        self.0.update(data);
    }
}

impl digest::Reset for Blake2b256 {
    fn reset(&mut self) {
        self.0.reset()
    }
}

impl digest::BlockInput for Blake2b256 {
    type BlockSize = U128;
}

impl digest::FixedOutputDirty for Blake2b256 {
    type OutputSize = U32;

    fn finalize_into_dirty(&mut self, out: &mut GenericArray<u8, Self::OutputSize>) {
        let mut result: GenericArray<u8, Self::OutputSize> = GenericArray::default();

        self.0.finalize_variable_reset(|slice| result.copy_from_slice(slice));

        *out = result;
    }
}
