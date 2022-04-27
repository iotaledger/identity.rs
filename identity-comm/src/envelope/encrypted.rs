// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Functionality for creating [DIDComm encrypted messages](https://identity.foundation/didcomm-messaging/spec/#didcomm-encrypted-message)

#![allow(non_camel_case_types)]

use crypto::ciphers::aes::Aes256Gcm;
use crypto::ciphers::aes_kw::Aes256Kw;
use crypto::ciphers::traits::Aead;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::KeyPair;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_iota_core::did::IotaDID;
use identity_iota_core::did::IotaDIDUrl;
use libjose::jwe::Decoder;
use libjose::jwe::DecoderRecipient;
use libjose::jwe::Encoder;
use libjose::jwe::Expanded;
use libjose::jwe::JweAlgorithm;
use libjose::jwe::JweEncryption;
use libjose::jwe::JweFormat;
use libjose::jwe::JweHeader;
use libjose::jwe::Recipient;
use libjose::jwe::Token;

use libjose::jwt::JwtHeader;
use libjose::jwt::JwtHeaderSet;
use libjose::utils::create_aad;
use libjose::utils::decode_b64;

use crate::envelope::EnvelopeExt;
use crate::envelope::Plaintext;
use crate::envelope::Signed;
use crate::error::Result;

use super::EcdhDeriver;
use super::HeaderSet;

/// A DIDComm Encrypted Message
///
/// [Reference](https://identity.foundation/didcomm-messaging/spec/#didcomm-encrypted-message)
///
/// # Layout
///
///   `JWE(Plaintext | Signed)`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DidCommEncryptedMessage(pub String);

impl DidCommEncryptedMessage {
  pub fn pack<T: ToJson>(
    message: &T,
    cek_algorithm: CEKAlgorithm,
    enc_algorithm: EncryptionAlgorithm,
    recipients: &[(PublicKey, String)],
    sender: &KeyPair,
  ) -> Result<Self> {
    Plaintext::pack(message)
      .and_then(|plaintext| Self::pack_plaintext(&plaintext, cek_algorithm, enc_algorithm, recipients, sender))
  }

  pub fn pack_plaintext(
    envelope: &Plaintext,
    cek_algorithm: CEKAlgorithm,
    enc_algorithm: EncryptionAlgorithm,
    recipients: &[(PublicKey, String)],
    sender: &KeyPair,
  ) -> Result<Self> {
    Self::pack_envelope(envelope, cek_algorithm, enc_algorithm, recipients, sender)
  }

  pub fn pack_signed(
    envelope: &Signed,
    cek_algorithm: CEKAlgorithm,
    enc_algorithm: EncryptionAlgorithm,
    recipients: &[(PublicKey, String)],
    sender: &KeyPair,
  ) -> Result<Self> {
    Self::pack_envelope(envelope, cek_algorithm, enc_algorithm, recipients, sender)
  }

  fn pack_envelope<T: EnvelopeExt>(
    envelope: &T,
    cek_algorithm: CEKAlgorithm,
    enc_algorithm: EncryptionAlgorithm,
    recipients: &[(PublicKey, String)],
    sender: &KeyPair,
  ) -> Result<Self> {
    let header: JweHeader = JweHeader::new(JweAlgorithm::from(cek_algorithm), enc_algorithm.into());

    let encoder: Encoder<'_> = Encoder::new()
      .format(JweFormat::General)
      .protected(&header)
      .secret(sender.private());

    recipients
      .iter()
      .fold(encoder, |encoder, (recipient, kid)| {
        let mut recipient: Recipient<'_> = recipient.into();

        let mut recipient_header: JweHeader = header.clone();
        let mut jwt_header = JwtHeader::new();
        jwt_header.set_kid(kid);
        recipient_header.set_common(jwt_header);

        recipient.header = Some(recipient_header);
        encoder.recipient(recipient)
      })
      .encode(envelope.as_bytes())
      .map_err(Into::into)
      .map(Self)
  }

  pub fn unpack<T: FromJson>(
    &self,
    cek_algorithm: CEKAlgorithm,
    enc_algorithm: EncryptionAlgorithm,
    recipient: &PrivateKey,
    sender: &PublicKey,
  ) -> Result<T> {
    let bytes: Vec<u8> = self.unpack_vec(cek_algorithm, enc_algorithm, recipient, sender)?;

    T::from_json_slice(&bytes).map_err(Into::into)
  }

  pub fn unpack_vec(
    &self,
    cek_algorithm: CEKAlgorithm,
    enc_algorithm: EncryptionAlgorithm,
    recipient: &PrivateKey,
    sender: &PublicKey,
  ) -> Result<Vec<u8>> {
    let token: Token = Decoder::new(recipient)
      .public(sender)
      .format(JweFormat::General)
      .algorithm(JweAlgorithm::from(cek_algorithm))
      .encryption(enc_algorithm.into())
      .decode(self.as_bytes())?;

    Ok(token.1)
  }

  pub async fn unpack_vec_with_storage(&self, local_recipient: IotaDID, deriver: impl EcdhDeriver) -> Result<Vec<u8>> {
    let (expanded, protected, unprotected): (Expanded<'_>, Option<JweHeader>, Option<JweHeader>) =
      Decoder::expand(JweFormat::General, &None, self.as_bytes()).unwrap();

    let mut recipient_kid: Option<(&DecoderRecipient<'_>, IotaDIDUrl)> = None;

    // TODO: We may need to fetch all urls where the did matches,
    // as the sender can encrypt for multiple keys of the same DID,
    // but not all keys may be present locally.
    for recipient in expanded.recipients.iter() {
      if let Some(kid) = recipient
        .header
        .as_ref()
        .map(JweHeader::common)
        .and_then(JwtHeader::kid)
      {
        let url = IotaDIDUrl::parse(kid).expect("TODO");
        if url.did() == &local_recipient {
          recipient_kid = Some((recipient, url))
        }
      }
    }

    let (recipient, key_url) = recipient_kid.ok_or(libjose::Error::KeyError(
      "message cannot be decrypted by this recipient",
    ))?;

    let merged: JwtHeaderSet<'_, JweHeader> = JwtHeaderSet::new()
      .header(recipient.header.as_ref())
      .protected(protected.as_ref())
      .unprotected(unprotected.as_ref());

    // TODO: Technically, we could add the validation from decrypt_cek here, but we probably have no "alg" or "enc"
    // claims to validate against, nor an expected kid.

    let cek = decrypt_cek(&merged, recipient, key_url, deriver).await?;

    let iv: Vec<u8> = expanded.iv.map(decode_b64).transpose()?.unwrap_or_default();
    let tag: Vec<u8> = expanded.tag.map(decode_b64).transpose()?.unwrap_or_default();
    let aad: Vec<u8> = create_aad(expanded.protected, expanded.aad);
    let ciphertext: Vec<u8> = decode_b64(expanded.ciphertext)?;
    let encryption: JweEncryption = merged.try_enc()?;
    let plaintext: Vec<u8> = decrypt_content(encryption, &cek, &iv, &aad, &tag, &ciphertext)?;

    let claims: Vec<u8> = if let Some(zip) = protected.as_ref().and_then(JweHeader::zip) {
      zip.decompress(&plaintext)?
    } else {
      plaintext
    };

    Ok(claims)
  }
}

async fn decrypt_cek(
  header: &HeaderSet<'_>,
  recipient: &DecoderRecipient<'_>,
  key_url: IotaDIDUrl,
  deriver: impl EcdhDeriver,
) -> Result<Vec<u8>> {
  let cek = match header.try_alg()? {
    JweAlgorithm::ECDH_ES_A256KW => ecdh_es_a256kw(header, recipient, key_url, deriver).await?,
    // JweAlgorithm::ECDH_1PU_A256KW => {
    //   deriver.derive_ecdh_1pu(&merged, alg.name(), alg.try_key_len()?).await;
    // }
    jwa => unimplemented!("{jwa:?}"),
  };

  if cek.len() == header.try_enc()?.key_len() {
    Ok(cek)
  } else {
    Err(libjose::Error::InvalidContent("CEK (length)").into())
  }
}

async fn ecdh_es_a256kw(
  header: &HeaderSet<'_>,
  recipient: &DecoderRecipient<'_>,
  key_url: IotaDIDUrl,
  deriver: impl EcdhDeriver,
) -> Result<Vec<u8>> {
  let algorithm: &str = header.try_alg()?.name();
  let key_len: usize = header.try_alg()?.try_key_len()?;
  let derived: Vec<u8> = deriver.derive_ecdh_es(header, algorithm, key_url, key_len).await?;
  let ctx: Vec<u8> = libjose::utils::parse_cek(recipient.encrypted_key)?;

  let mut ptx: Vec<u8> = ctx
    .len()
    .checked_sub(Aes256Kw::BLOCK)
    .ok_or(libjose::Error::InvalidContent("CEK (length)"))
    .map(|length| vec![0; length])?;

  Aes256Kw::new(&derived)
    .unwrap_key(&ctx, &mut ptx)
    .map_err(Into::<libjose::Error>::into)?;

  Ok(ptx)
}

fn decrypt_content(
  encryption: JweEncryption,
  key: &[u8],
  iv: &[u8],
  aad: &[u8],
  tag: &[u8],
  ciphertext: &[u8],
) -> Result<Vec<u8>> {
  let mut plaintext: Vec<u8> = vec![0; ciphertext.len()];

  let length: usize = match encryption {
    JweEncryption::A256GCM => {
      Aes256Gcm::try_decrypt(key, iv, aad, &mut plaintext, ciphertext, tag).map_err(Into::<libjose::Error>::into)?
    }
    jwe => unimplemented!("{jwe:?}"),
  };

  plaintext.truncate(length);

  Ok(plaintext)
}

impl EnvelopeExt for DidCommEncryptedMessage {
  const FEXT: &'static str = "dcem";
  const MIME: &'static str = "application/didcomm-encrypted+json";

  fn as_bytes(&self) -> &[u8] {
    self.0.as_bytes()
  }
}

// =============================================================================
// =============================================================================

/// Supported content encryption algorithms
///
/// [Reference (auth)](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption)
/// [Reference (anon)](https://identity.foundation/didcomm-messaging/spec/#anonymous-encryption)
#[derive(Clone, Copy, Debug)]
pub enum EncryptionAlgorithm {
  A256GCM,
  XC20P,
}

impl From<EncryptionAlgorithm> for JweEncryption {
  fn from(other: EncryptionAlgorithm) -> Self {
    match other {
      EncryptionAlgorithm::A256GCM => Self::A256GCM,
      EncryptionAlgorithm::XC20P => Self::XC20P,
    }
  }
}

/// Supported algorithms for the cryptographic algorithm used to encrypt
/// or determine the value of the CEK.
#[derive(Clone, Copy, Debug)]
pub enum CEKAlgorithm {
  /// Can be used for sender-authenticated encryption.
  ECDH_1PU_A256KW,
  /// Can be used for anonymous encryption.
  ECDH_ES_A256KW,
}

impl From<CEKAlgorithm> for JweAlgorithm {
  fn from(other: CEKAlgorithm) -> Self {
    match other {
      CEKAlgorithm::ECDH_1PU_A256KW => JweAlgorithm::ECDH_1PU_A256KW,
      CEKAlgorithm::ECDH_ES_A256KW => JweAlgorithm::ECDH_ES_A256KW,
    }
  }
}

#[cfg(test)]
mod tests {
  use identity_core::convert::FromJson;
  use identity_core::crypto::KeyPair;
  use identity_did::did::DID;
  use identity_iota_core::did::IotaDID;

  use crate::envelope::LocalEcdhDeriver;

  use super::DidCommEncryptedMessage;

  #[tokio::test]
  async fn test_packing() {
    let did: IotaDID = "did:iota:6Hz3Yo2Qzj54L6MtS4otbETuG3GQUF5cHLqTcnsyzH96".parse().unwrap();
    let did2: IotaDID = "did:iota:DQE89CN6GTiF2bkqzEBtBDHpZgGyYZ5SK4kymJ4PiAXW".parse().unwrap();

    let keypair1 = KeyPair::new(identity_core::crypto::KeyType::X25519).unwrap();
    let keypair2 = KeyPair::new(identity_core::crypto::KeyType::X25519).unwrap();
    let sender_keypair = KeyPair::new(identity_core::crypto::KeyType::X25519).unwrap();
    let plaintext = "plaintext".to_owned();

    let dcem = DidCommEncryptedMessage::pack(
      &plaintext,
      super::CEKAlgorithm::ECDH_ES_A256KW,
      super::EncryptionAlgorithm::A256GCM,
      &[
        (
          keypair1.public().to_owned(),
          did.to_url().join("#kex-0").unwrap().to_string(),
        ),
        (
          keypair2.public().to_owned(),
          did2.to_url().join("#kex-1").unwrap().to_string(),
        ),
      ],
      &sender_keypair,
    )
    .unwrap();

    let deriver = LocalEcdhDeriver::new(keypair1.private().into(), Some(sender_keypair.public().into()));
    let deriver2 = LocalEcdhDeriver::new(keypair2.private().into(), Some(sender_keypair.public().into()));

    // Any recipient can decrypt the message.
    let decrypted = dcem.unpack_vec_with_storage(did, deriver).await.unwrap();
    let decrypted2 = dcem.unpack_vec_with_storage(did2, deriver2).await.unwrap();

    let decrypted = String::from_json_slice(decrypted.as_slice()).unwrap();
    let decrypted2 = String::from_json_slice(decrypted2.as_slice()).unwrap();

    assert_eq!(plaintext, decrypted);
    assert_eq!(plaintext, decrypted2);
  }
}
