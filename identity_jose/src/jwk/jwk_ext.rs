//TODO: JwkExt

use std::str::FromStr;

use identity_core::common::Url;
use jsonprooftoken::{jwk::{key::{Jwk as JwkExt, KeyOps, PKUse}, alg_parameters::{JwkAlgorithmParameters, JwkOctetKeyPairParameters}}, jpa::algs::ProofAlgorithm};

use crate::jws::JwsAlgorithm;

use super::{Jwk, JwkOperation, JwkUse, JwkParams, JwkParamsOkp, JwkType};


impl From<KeyOps> for JwkOperation {
    fn from(value: KeyOps) -> Self {
        match value {
            KeyOps::Sign => Self::Sign,
            KeyOps::Verify => Self::Verify,
            KeyOps::Encrypt => Self::Encrypt,
            KeyOps::Decrypt => Self::Decrypt,
            KeyOps::WrapKey => Self::WrapKey,
            KeyOps::UnwrapKey => Self::UnwrapKey,
            KeyOps::DeriveKey => Self::DeriveKey,
            KeyOps::DeriveBits => Self::DeriveBits,
            KeyOps::ProofGeneration => Self::ProofGeneration,
            KeyOps::ProofVerification => Self::ProofVerification,
        }
    }
}


impl From<PKUse> for JwkUse {
    fn from(value: PKUse) -> Self {
        match value {
            PKUse::Signature => Self::Signature,
            PKUse::Encryption => Self::Encryption,
            PKUse::Proof => Self::Proof,
        }
    }
}


impl From<JwkOctetKeyPairParameters> for JwkParamsOkp {
    fn from(value: JwkOctetKeyPairParameters) -> Self {
        Self { 
            crv: value.crv.to_string(), 
            x: value.x, 
            d: value.d 
        }
    }
}

impl TryFrom<JwkExt> for Jwk {

    type Error = crate::error::Error;

    fn try_from(value: JwkExt) -> Result<Self, Self::Error> {

        let x5u = match value.x5u {
            Some(v) => {
                Some(
                    Url::from_str(&v).map_err(|_| {
                        return Self::Error::InvalidClaim("x5u");
                    })?
                )
            },
            None => None,
        };

        let (kty, params) = match value.key_params {
            JwkAlgorithmParameters::OctetKeyPair(p) => (JwkType::Okp, JwkParams::Okp(JwkParamsOkp::from(p))),
        };

        Ok(Self { 
            kty: kty,
            use_: value.pk_use.and_then(|u| Some(JwkUse::from(u))),
            key_ops: value.key_ops.and_then(|vec_key_ops| {
                Some(vec_key_ops.into_iter().map(JwkOperation::from).collect())
            }),
            alg: value.alg.and_then(|a| Some(a.to_string())), 
            kid: value.kid, 
            x5u: x5u, 
            x5c: value.x5c, 
            x5t: value.x5t, 
            x5t_s256: None, 
            params: params
        })
    }
}

