//TODO: hybrid - composite public key

use std::fmt::Display;

use identity_jose::{jwk::Jwk, jws::JwsAlgorithm};

//TODO: hybrid - move to identity_jose


//TODO: hybrid - to be removed
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum CompositeAlgId {
    #[serde(rename = "id-MLDSA44-Ed25519-SHA512")]
    IdMldsa44Ed25519Sha512,
    
    #[serde(rename = "id-MLDSA65-Ed25519-SHA512")]
    IdMldsa65Ed25519Sha512
}

impl CompositeAlgId {
    pub fn der_oid(&self) -> &'static [u8] {
        match self {
            CompositeAlgId::IdMldsa44Ed25519Sha512 => &[
                0x06, 0x0B, 0x60, 0x86, 0x48, 0x01, 0x86, 0xFA, 0x6B, 0x50, 0x08, 0x01, 0x03,
            ],
            CompositeAlgId::IdMldsa65Ed25519Sha512 => &[
                0x06, 0x0B, 0x60, 0x86, 0x48, 0x01, 0x86, 0xFA, 0x6B, 0x50, 0x08, 0x01, 0x0A,
            ],
        }
    }


    /// Returns the JWS algorithm as a `str` slice.
    pub const fn name(self) -> &'static str {
        match self {
            Self::IdMldsa44Ed25519Sha512 => "id-MLDSA44-Ed25519-SHA512",      
            Self::IdMldsa65Ed25519Sha512 => "id-MLDSA65-Ed25519-SHA512",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct CompositePublicKey {
    #[serde(rename = "algId")]
    alg_id: CompositeAlgId,
    #[serde(rename = "traditionalPublicKey")]
    traditional_public_key: Jwk,
    #[serde(rename = "pqPublicKey")]
    pq_public_key: Jwk,
}

impl CompositePublicKey {
    pub fn new(alg_id: CompositeAlgId, traditional_public_key: Jwk, pq_public_key: Jwk) -> Self {
        Self { alg_id, traditional_public_key, pq_public_key }
    }

    pub fn alg_id(&self) -> CompositeAlgId {
        self.alg_id
    }

    pub fn pq_public_key(&self) -> &Jwk {
        &self.pq_public_key
    }

    pub fn traditional_public_key(&self) -> &Jwk {
        &self.traditional_public_key
    }

}