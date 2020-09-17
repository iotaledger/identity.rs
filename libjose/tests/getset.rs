use libjose::jwe::JweAlgorithm;
use libjose::jwe::JweCompression;
use libjose::jwe::JweEncryption;
use libjose::jwe::JweHeader;
use libjose::jwk::Jwk;
use libjose::jwk::JwkOperation;
use libjose::jwk::JwkType;
use libjose::jwk::JwkUse;
use libjose::jws::JwsAlgorithm;
use libjose::jws::JwsHeader;
use libjose::jwt::JwtClaims;

macro_rules! test_getset {
  ($ty:ty, $get:ident, $set:ident, Url = $value:expr) => {
    let mut header = <$ty>::new();
    assert_eq!(header.$get(), None);
    header.$set(::url::Url::parse($value).unwrap());
    assert_eq!(header.$get().unwrap().as_str(), $value);
  };
  ($ty:ty, $get:ident, $set:ident, Option = $value:expr) => {
    let mut header = <$ty>::new();
    assert_eq!(header.$get(), None);
    header.$set($value);
    assert_eq!(header.$get().unwrap(), $value);
  };
  ($ty:ty, $get:ident, $set:ident, OptionRef = $value:expr) => {
    let mut header = <$ty>::new();
    assert_eq!(header.$get(), None);
    header.$set($value);
    assert_eq!(header.$get().unwrap(), &$value);
  };
  ($ty:ty, $get:ident, $set:ident, $value:expr) => {
    assert!($value != Default::default());
    let mut header = <$ty>::new();
    assert_eq!(header.$get(), Default::default());
    header.$set($value);
    assert_eq!(header.$get(), $value);
  };
}

#[test]
fn test_jwe_header_getset() {
  test_getset!(JweHeader, alg, set_alg, JweAlgorithm::ECDH_ES_A256KW);
  test_getset!(JweHeader, enc, set_enc, JweEncryption::A256CBC_HS512);
  test_getset!(JweHeader, zip, set_zip, OptionRef = JweCompression::Deflate);
  test_getset!(JweHeader, jku, set_jku, Url = "https://foo.com/jku");
  test_getset!(JweHeader, jwk, set_jwk, OptionRef = Jwk::new());
  test_getset!(JweHeader, kid, set_kid, Option = "key id");
  test_getset!(JweHeader, x5u, set_x5u, Url = "https://foo.com/x509");
  test_getset!(JweHeader, x5c, set_x5c, Option = vec![vec![1, 2, 3, 4]]);
  test_getset!(JweHeader, x5t, set_x5t, Option = vec![1, 2, 3, 4]);
  test_getset!(JweHeader, x5t_s256, set_x5t_s256, Option = vec![1, 2, 3, 4]);
  test_getset!(JweHeader, typ, set_typ, Option = "type");
  test_getset!(JweHeader, cty, set_cty, Option = "content type");
  test_getset!(JweHeader, crit, set_crit, Option = vec!["critical"]);
  test_getset!(JweHeader, url, set_url, Url = "https://foo.com/url");
  test_getset!(JweHeader, nonce, set_nonce, Option = vec![1, 2, 3, 4]);
  test_getset!(JweHeader, epk, set_epk, OptionRef = Jwk::new());
  test_getset!(JweHeader, apu, set_apu, Option = "agreement partyuinfo");
  test_getset!(JweHeader, apv, set_apv, Option = "agreement partyvinfo");
  test_getset!(JweHeader, iv, set_iv, Option = "initialization vector");
  test_getset!(JweHeader, tag, set_tag, Option = "authentication tag");
  test_getset!(JweHeader, p2s, set_p2s, Option = "pbes2 salt input");
  test_getset!(JweHeader, p2c, set_p2c, Option = 123456789u64);
}

#[test]
fn test_jwk_getset() {
  test_getset!(Jwk, kty, set_kty, JwkType::Ec);
  test_getset!(Jwk, use_, set_use, OptionRef = JwkUse::Signature);
  test_getset!(Jwk, key_ops, set_key_ops, Option = vec![JwkOperation::DeriveBits]);
  test_getset!(Jwk, alg, set_alg, Option = "algorithm");
  test_getset!(Jwk, kid, set_kid, Option = "key id");
  test_getset!(Jwk, x5u, set_x5u, Url = "https://foo.com/");
  test_getset!(Jwk, x5c, set_x5c, Option = vec![vec![1, 2, 3, 4],]);
  test_getset!(Jwk, x5t, set_x5t, Option = vec![1, 2, 3, 4]);
  test_getset!(Jwk, x5t_s256, set_x5t_s256, Option = vec![1, 2, 3, 4]);
}

#[test]
fn test_jws_header_getset() {
  test_getset!(JwsHeader, alg, set_alg, JwsAlgorithm::EdDSA);
  test_getset!(JwsHeader, jku, set_jku, Url = "https://foo.com/");
  test_getset!(JwsHeader, jwk, set_jwk, OptionRef = Jwk::new());
  test_getset!(JwsHeader, kid, set_kid, Option = "key id");
  test_getset!(JwsHeader, x5u, set_x5u, Url = "https://foo.com/");
  test_getset!(JwsHeader, x5c, set_x5c, Option = vec![vec![1, 2, 3, 4],]);
  test_getset!(JwsHeader, x5t, set_x5t, Option = vec![1, 2, 3, 4]);
  test_getset!(JwsHeader, x5t_s256, set_x5t_s256, Option = vec![1, 2, 3, 4]);
  test_getset!(JwsHeader, typ, set_typ, Option = "type");
  test_getset!(JwsHeader, cty, set_cty, Option = "content type");
  test_getset!(JwsHeader, crit, set_crit, Option = vec!["critical"]);
  test_getset!(JwsHeader, b64, set_b64, Option = false);
  test_getset!(JwsHeader, url, set_url, Url = "https://foo.com/");
  test_getset!(JwsHeader, nonce, set_nonce, Option = vec![1, 2, 3, 4]);
  test_getset!(JwsHeader, ppt, set_ppt, Option = "passport");
}

#[test]
fn test_jwt_claims_getset() {
  test_getset!(JwtClaims, iss, set_iss, Option = "issuer");
  test_getset!(JwtClaims, sub, set_sub, Option = "subject");
  test_getset!(JwtClaims, aud, set_aud, Option = vec!["audience"]);
  test_getset!(JwtClaims, exp, set_exp, Option = 123456789);
  test_getset!(JwtClaims, nbf, set_nbf, Option = 123456789);
  test_getset!(JwtClaims, iat, set_iat, Option = 123456789);
  test_getset!(JwtClaims, jti, set_jti, Option = "jwt id");
  test_getset!(JwtClaims, did, set_did, OptionRef = libjose::jwt::DID("did:example:123456789abcdefghi".into()));
  test_getset!(JwtClaims, vc, set_vc, OptionRef = libjose::jwt::Credential::Standard);
  test_getset!(JwtClaims, vp, set_vp, OptionRef = libjose::jwt::Presentation::Standard);
}
