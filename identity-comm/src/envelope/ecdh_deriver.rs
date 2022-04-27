// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account_storage::storage::Storage;
use identity_account_storage::types::KeyLocation;
use identity_iota_core::did::IotaDIDUrl;
use identity_iota_core::document::IotaVerificationMethod;
use libjose::jwe::JweHeader;
use libjose::jwk::EcdhCurve;
use libjose::jwk::EcxCurve;
use libjose::jwk::Jwk;
use libjose::jwt::JwtHeaderSet;
use libjose::utils::concat_kdf;
use libjose::utils::decode_b64;
use libjose::utils::diffie_hellman;
use libjose::utils::Secret;

use crate::error::Result;

pub type HeaderSet<'a> = JwtHeaderSet<'a, JweHeader>;

#[async_trait::async_trait]
pub trait EcdhDeriver {
  async fn derive_ecdh_es(
    &self,
    header: &HeaderSet<'_>,
    algorithm: &str,
    key_url: IotaDIDUrl,
    key_len: usize,
  ) -> Result<Vec<u8>>;
  async fn derive_ecdh_1pu(
    &self,
    header: &HeaderSet<'_>,
    algorithm: &str,
    key_url: IotaDIDUrl,
    key_len: usize,
  ) -> Result<Vec<u8>>;
}

#[derive(Debug, Clone)]
pub struct LocalEcdhDeriver<'a> {
  ecdh_curve: EcdhCurve,
  secret: Secret<'a>,
  public: Option<Secret<'a>>,
}

impl<'a> LocalEcdhDeriver<'a> {
  pub fn new(secret: Secret<'a>, public: Option<Secret<'a>>) -> Self {
    Self {
      ecdh_curve: EcdhCurve::Ecx(EcxCurve::X25519),
      secret,
      public,
    }
  }

  fn derive_ecdh_key(
    &self,
    header: &HeaderSet<'_>,
    algorithm: &str,
    key_len: usize,
    key_exchange: impl Fn(&Jwk) -> libjose::Result<Vec<u8>>,
  ) -> libjose::Result<Vec<u8>> {
    let (apu, apv): (Vec<u8>, Vec<u8>) = decode_agreement_info(header)?;

    concat_kdf(
      algorithm,
      key_len,
      &key_exchange(header.try_epk()?)?,
      apu.as_ref(),
      apv.as_ref(),
    )
  }
}

fn decode_agreement_info(header: &HeaderSet<'_>) -> libjose::Result<(Vec<u8>, Vec<u8>)> {
  let apu: Option<Vec<u8>> = header.apu().map(decode_b64).transpose()?;
  let apv: Option<Vec<u8>> = header.apv().map(decode_b64).transpose()?;

  Ok((apu.unwrap_or_default(), apv.unwrap_or_default()))
}

#[async_trait::async_trait]
impl<'a> EcdhDeriver for LocalEcdhDeriver<'a> {
  async fn derive_ecdh_es(
    &self,
    header: &HeaderSet<'_>,
    algorithm: &str,
    key_url: IotaDIDUrl,
    key_len: usize,
  ) -> Result<Vec<u8>> {
    self
      .derive_ecdh_key(header, algorithm, key_len, |epk| {
        diffie_hellman(self.ecdh_curve, epk, self.secret)
      })
      .map_err(Into::into)
  }

  async fn derive_ecdh_1pu(
    &self,
    header: &HeaderSet<'_>,
    algorithm: &str,
    key_url: IotaDIDUrl,
    key_len: usize,
  ) -> Result<Vec<u8>> {
    self
      .derive_ecdh_key(header, algorithm, key_len, |epk| {
        let public: Secret<'a> = self
          .public
          .ok_or(libjose::Error::EncError("missing ECDH-1PU Public Key"))?;
        let ze: Vec<u8> = diffie_hellman(self.ecdh_curve, epk, self.secret)?;
        let zs: Vec<u8> = diffie_hellman(self.ecdh_curve, public, self.secret)?;

        Ok([ze, zs].concat())
      })
      .map_err(Into::into)
  }
}

pub struct RemoteEcdhDeriver<'a> {
  ecdh_curve: EcdhCurve,
  storage: &'a dyn Storage,
  public: Option<Secret<'a>>,
}

impl<'a> RemoteEcdhDeriver<'a> {
  async fn resolve_key_location(&self, private_key_url: IotaDIDUrl) -> Result<KeyLocation> {
    let document = self
      .storage
      .document_get(private_key_url.did())
      .await
      .expect("TODO")
      .expect("TODO");

    let method: &IotaVerificationMethod = document.resolve_method(private_key_url, None).expect("TODO");

    Ok(KeyLocation::from_verification_method(method).expect("TODO"))
  }
}

#[async_trait::async_trait]
impl<'a> EcdhDeriver for RemoteEcdhDeriver<'a> {
  async fn derive_ecdh_es(
    &self,
    header: &HeaderSet<'_>,
    algorithm: &str,
    key_url: IotaDIDUrl,
    key_len: usize,
  ) -> Result<Vec<u8>> {
    let (apu, apv): (Vec<u8>, Vec<u8>) = decode_agreement_info(header)?;
    todo!()
    // self
    //   .derive_ecdh_key(header, algorithm, key_len, |epk| {
    //     diffie_hellman(self.ecdh_curve, epk, self.secret)
    //   })
    //   .map_err(Into::into)
  }

  async fn derive_ecdh_1pu(
    &self,
    header: &HeaderSet<'_>,
    algorithm: &str,
    key_url: IotaDIDUrl,
    key_len: usize,
  ) -> Result<Vec<u8>> {
    let (apu, apv): (Vec<u8>, Vec<u8>) = decode_agreement_info(header)?;
    todo!()
    // self
    //   .derive_ecdh_key(header, algorithm, key_len, |epk| {
    //     let public: Secret<'a> = self
    //       .public
    //       .ok_or(libjose::Error::EncError("missing ECDH-1PU Public Key"))?;
    //     let ze: Vec<u8> = diffie_hellman(self.ecdh_curve, epk, self.secret)?;
    //     let zs: Vec<u8> = diffie_hellman(self.ecdh_curve, public, self.secret)?;

    //     Ok([ze, zs].concat())
    //   })
    //   .map_err(Into::into)
  }
}
