//TODO: ZKP - JwkExt

use std::str::FromStr;
use identity_core::common::Url;
use jsonprooftoken::{jwk::{key::{Jwk as JwkExt, KeyOps, PKUse}, alg_parameters::{JwkAlgorithmParameters, JwkOctetKeyPairParameters, Algorithm}, types::KeyType, curves::EllipticCurveTypes}, jpa::algs::ProofAlgorithm};
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

impl Into<KeyOps> for JwkOperation {
    fn into(self) -> KeyOps {
        match self {
            Self::Sign => KeyOps::Sign,
            Self::Verify => KeyOps::Verify,
            Self::Encrypt => KeyOps::Encrypt,
            Self::Decrypt => KeyOps::Decrypt,
            Self::WrapKey => KeyOps::WrapKey,
            Self::UnwrapKey => KeyOps::UnwrapKey,
            Self::DeriveKey => KeyOps::DeriveKey,
            Self::DeriveBits => KeyOps::DeriveBits,
            Self::ProofGeneration => KeyOps::ProofGeneration,
            Self::ProofVerification => KeyOps::ProofVerification,
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

impl Into<PKUse> for JwkUse {
    fn into(self) -> PKUse {
        match self {
            Self::Signature => PKUse::Signature,
            Self::Encryption => PKUse::Encryption,
            Self::Proof => PKUse::Proof,
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

impl TryInto<JwkOctetKeyPairParameters> for &JwkParamsOkp {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<JwkOctetKeyPairParameters, Self::Error> {
        Ok(JwkOctetKeyPairParameters {
            kty: KeyType::OctetKeyPair,
            crv: EllipticCurveTypes::from_str(&self.crv).map_err(|_| {
                return Self::Error::KeyError("Invalid crv!") })?,
            x: self.x.clone(),
            d: self.d.clone(),
        })
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


impl TryInto<JwkExt> for &Jwk {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<JwkExt, Self::Error> {

        let params = match &self.params {

            JwkParams::Okp(p) => JwkAlgorithmParameters::OctetKeyPair(p.try_into()?),
            _ => return Err(Self::Error::InvalidParam("Parameters not supported!"))
        };

        let alg = match &self.alg {
            Some(a) => {
                Some(Algorithm::Proof(ProofAlgorithm::from_str(&a).map_err(|_| Self::Error::KeyError("Invalid alg"))?))
            },
            None => None,
        };

        Ok(JwkExt{ 
            kid: self.kid.clone(), 
            pk_use: self.use_.and_then(|u| Some(u.into())), 
            key_ops: self.key_ops.as_deref().and_then(|vec_key_ops| {
                vec_key_ops.iter().map(|o| Some((*o).into())).collect()
            }), 
            alg: alg, 
            x5u: match &self.x5u {
                Some(v) => {
                    Some(
                        v.as_str().to_string()
                    )
                },
                None => None,
            }, 
            x5c: self.x5c.clone(), 
            x5t: self.x5t.clone(), 
            key_params: params 
        })
        
    }
}
