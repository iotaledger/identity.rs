use libjose::jwe::JweAlgorithm;
use libjose::jwe::JweCompression;
use libjose::jwe::JweEncryption;
use libjose::jwk::JwkOperation;
use libjose::jwk::JwkType;
use libjose::jwk::JwkUse;
use libjose::jws::JwsAlgorithm;

macro_rules! jstr {
  ($expr:expr) => {
    format!("\"{}\"", $expr)
  };
}

macro_rules! assert_serde {
  ($src:expr, $dst:expr) => {
    assert_eq!($src, ::serde_json::from_str(&$dst).unwrap());
    assert_eq!($dst, ::serde_json::to_string(&$src).unwrap());
  };
}

#[test]
fn test_jwe_algorithm_serde() {
  assert_serde!(JweAlgorithm::RSA1_5, jstr!("RSA1_5"));
  assert_serde!(JweAlgorithm::RSA_OAEP, jstr!("RSA-OAEP"));
  assert_serde!(JweAlgorithm::RSA_OAEP_256, jstr!("RSA-OAEP-256"));
  assert_serde!(JweAlgorithm::RSA_OAEP_384, jstr!("RSA-OAEP-384"));
  assert_serde!(JweAlgorithm::RSA_OAEP_512, jstr!("RSA-OAEP-512"));
  assert_serde!(JweAlgorithm::A128KW, jstr!("A128KW"));
  assert_serde!(JweAlgorithm::A192KW, jstr!("A192KW"));
  assert_serde!(JweAlgorithm::A256KW, jstr!("A256KW"));
  assert_serde!(JweAlgorithm::DIR, jstr!("dir"));
  assert_serde!(JweAlgorithm::ECDH_ES, jstr!("ECDH-ES"));
  assert_serde!(JweAlgorithm::ECDH_ES_A128KW, jstr!("ECDH-ES+A128KW"));
  assert_serde!(JweAlgorithm::ECDH_ES_A192KW, jstr!("ECDH-ES+A192KW"));
  assert_serde!(JweAlgorithm::ECDH_ES_A256KW, jstr!("ECDH-ES+A256KW"));
  assert_serde!(JweAlgorithm::A128GCMKW, jstr!("A128GCMKW"));
  assert_serde!(JweAlgorithm::A192GCMKW, jstr!("A192GCMKW"));
  assert_serde!(JweAlgorithm::A256GCMKW, jstr!("A256GCMKW"));
  assert_serde!(JweAlgorithm::PBES2_HS256_A128KW, jstr!("PBES2-HS256+A128KW"));
  assert_serde!(JweAlgorithm::PBES2_HS384_A192KW, jstr!("PBES2-HS384+A192KW"));
  assert_serde!(JweAlgorithm::PBES2_HS512_A256KW, jstr!("PBES2-HS512+A256KW"));
  assert_serde!(JweAlgorithm::ECDH_ES_C20PKW, jstr!("ECDH-ES+C20PKW"));
  assert_serde!(JweAlgorithm::ECDH_ES_XC20PKW, jstr!("ECDH-ES+XC20PKW"));
}

#[test]
fn test_jwe_compression_serde() {
  assert_serde!(JweCompression::Deflate, jstr!("DEF"));
  assert_serde!(JweCompression::Custom("foo".into()), jstr!("foo"));
}

#[test]
fn test_jwe_encryption_serde() {
  assert_serde!(JweEncryption::A128CBC_HS256, jstr!("A128CBC-HS256"));
  assert_serde!(JweEncryption::A192CBC_HS384, jstr!("A192CBC-HS384"));
  assert_serde!(JweEncryption::A256CBC_HS512, jstr!("A256CBC-HS512"));
  assert_serde!(JweEncryption::A128GCM, jstr!("A128GCM"));
  assert_serde!(JweEncryption::A192GCM, jstr!("A192GCM"));
  assert_serde!(JweEncryption::A256GCM, jstr!("A256GCM"));
  assert_serde!(JweEncryption::C20P, jstr!("C20P"));
  assert_serde!(JweEncryption::XC20P, jstr!("XC20P"));
}

#[test]
fn test_jwk_operation_serde() {
  assert_serde!(JwkOperation::Sign, jstr!("sign"));
  assert_serde!(JwkOperation::Verify, jstr!("verify"));
  assert_serde!(JwkOperation::Encrypt, jstr!("encrypt"));
  assert_serde!(JwkOperation::Decrypt, jstr!("decrypt"));
  assert_serde!(JwkOperation::WrapKey, jstr!("wrapKey"));
  assert_serde!(JwkOperation::UnwrapKey, jstr!("unwrapKey"));
  assert_serde!(JwkOperation::DeriveKey, jstr!("deriveKey"));
  assert_serde!(JwkOperation::DeriveBits, jstr!("deriveBits"));
  assert_serde!(JweCompression::Custom("foo".into()), jstr!("foo"));
}

#[test]
fn test_jwk_type_serde() {
  assert_serde!(JwkType::Ec, jstr!("EC"));
  assert_serde!(JwkType::Rsa, jstr!("RSA"));
  assert_serde!(JwkType::Oct, jstr!("oct"));
  assert_serde!(JwkType::Okp, jstr!("OKP"));
}

#[test]
fn test_jwk_use_serde() {
  assert_serde!(JwkUse::Signature, jstr!("sig"));
  assert_serde!(JwkUse::Encryption, jstr!("enc"));
  assert_serde!(JwkUse::Custom("foo".into()), jstr!("foo"));
}

#[test]
fn test_jws_algorithm_serde() {
  assert_serde!(JwsAlgorithm::HS256, jstr!("HS256"));
  assert_serde!(JwsAlgorithm::HS384, jstr!("HS384"));
  assert_serde!(JwsAlgorithm::HS512, jstr!("HS512"));
  assert_serde!(JwsAlgorithm::RS256, jstr!("RS256"));
  assert_serde!(JwsAlgorithm::RS384, jstr!("RS384"));
  assert_serde!(JwsAlgorithm::RS512, jstr!("RS512"));
  assert_serde!(JwsAlgorithm::ES256, jstr!("ES256"));
  assert_serde!(JwsAlgorithm::ES384, jstr!("ES384"));
  assert_serde!(JwsAlgorithm::ES512, jstr!("ES512"));
  assert_serde!(JwsAlgorithm::PS256, jstr!("PS256"));
  assert_serde!(JwsAlgorithm::PS384, jstr!("PS384"));
  assert_serde!(JwsAlgorithm::PS512, jstr!("PS512"));
  assert_serde!(JwsAlgorithm::NONE, jstr!("none"));
  assert_serde!(JwsAlgorithm::EdDSA, jstr!("EdDSA"));
  assert_serde!(JwsAlgorithm::ES256K, jstr!("ES256K"));
}
