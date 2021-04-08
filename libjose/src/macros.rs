// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[macro_export]
#[doc(hidden)]
macro_rules! rsa_padding {
  (@PKCS1_SHA256) => {
    ::rsa::PaddingScheme::new_pkcs1v15_sign(Some(::rsa::Hash::SHA2_256))
  };
  (@PKCS1_SHA384) => {
    ::rsa::PaddingScheme::new_pkcs1v15_sign(Some(::rsa::Hash::SHA2_384))
  };
  (@PKCS1_SHA512) => {
    ::rsa::PaddingScheme::new_pkcs1v15_sign(Some(::rsa::Hash::SHA2_512))
  };
  (@PSS_SHA256) => {
    ::rsa::PaddingScheme::new_pss::<::crypto::hashes::sha::Sha256, _>(::rand::rngs::OsRng)
  };
  (@PSS_SHA384) => {
    ::rsa::PaddingScheme::new_pss::<::crypto::hashes::sha::Sha384, _>(::rand::rngs::OsRng)
  };
  (@PSS_SHA512) => {
    ::rsa::PaddingScheme::new_pss::<::crypto::hashes::sha::Sha512, _>(::rand::rngs::OsRng)
  };
  (@RSA1_5) => {
    ::rsa::PaddingScheme::new_pkcs1v15_encrypt()
  };
  (@RSA_OAEP) => {
    ::rsa::PaddingScheme::new_oaep::<::sha1::Sha1>()
  };
  (@RSA_OAEP_256) => {
    ::rsa::PaddingScheme::new_oaep::<::crypto::hashes::sha::Sha256>()
  };
  (@RSA_OAEP_384) => {
    ::rsa::PaddingScheme::new_oaep::<::crypto::hashes::sha::Sha384>()
  };
  (@RSA_OAEP_512) => {
    ::rsa::PaddingScheme::new_oaep::<::crypto::hashes::sha::Sha512>()
  };
}
