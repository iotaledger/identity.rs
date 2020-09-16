use libjose::jwe::JweAlgorithm;
use libjose::jwe::JweEncryption;
use libjose::jwe::JweHeader;
use libjose::jwk::Jwk;

#[macro_use]
mod macros;

#[test]
fn test_header_getset_alg() {
  test_getset!(JweHeader, alg, set_alg, JweAlgorithm::ECDH_ES_A256KW);
}

#[test]
fn test_header_getset_enc() {
  test_getset!(JweHeader, enc, set_enc, JweEncryption::A256CBC_HS512);
}

#[test]
fn test_header_getset_zip() {
  test_getset!(JweHeader, zip, set_zip, Option = "compression algorithm");
}

#[test]
fn test_header_getset_jku() {
  test_getset!(
    JweHeader,
    jku,
    set_jku,
    Url = "https://example.com/jwk_set_url"
  );
}

#[test]
fn test_header_getset_jwk() {
  test_getset!(JweHeader, jwk, set_jwk, OptionRef = Jwk::new());
}

#[test]
fn test_header_getset_kid() {
  test_getset!(JweHeader, kid, set_kid, Option = "key id");
}

#[test]
fn test_header_getset_x5u() {
  test_getset!(
    JweHeader,
    x5u,
    set_x5u,
    Url = "https://example.com/x_509_url"
  );
}

#[test]
fn test_header_getset_x5c() {
  test_getset!(
    JweHeader,
    x5c,
    set_x5c,
    Option = vec![
      vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
      vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    ]
  );
}

#[test]
fn test_header_getset_x5t() {
  test_getset!(
    JweHeader,
    x5t,
    set_x5t,
    Option = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
  );
}

#[test]
fn test_header_getset_x5t_s256() {
  test_getset!(
    JweHeader,
    x5t_s256,
    set_x5t_s256,
    Option = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
  );
}

#[test]
fn test_header_getset_typ() {
  test_getset!(JweHeader, typ, set_typ, Option = "type");
}

#[test]
fn test_header_getset_cty() {
  test_getset!(JweHeader, cty, set_cty, Option = "content type");
}

#[test]
fn test_header_getset_crit() {
  test_getset!(JweHeader, crit, set_crit, Option = vec!["critical"]);
}

#[test]
fn test_header_getset_url() {
  test_getset!(JweHeader, url, set_url, Url = "https://example.com/url");
}

#[test]
fn test_header_getset_nonce() {
  test_getset!(
    JweHeader,
    nonce,
    set_nonce,
    Option = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
  );
}

#[test]
fn test_header_getset_epk() {
  test_getset!(JweHeader, epk, set_epk, OptionRef = Jwk::new());
}

#[test]
fn test_header_getset_apu() {
  test_getset!(JweHeader, apu, set_apu, Option = "agreement partyuinfo");
}

#[test]
fn test_header_getset_apv() {
  test_getset!(JweHeader, apv, set_apv, Option = "agreement partyvinfo");
}

#[test]
fn test_header_getset_iv() {
  test_getset!(JweHeader, iv, set_iv, Option = "initialization vector");
}

#[test]
fn test_header_getset_tag() {
  test_getset!(JweHeader, tag, set_tag, Option = "authentication tag");
}

#[test]
fn test_header_getset_p2s() {
  test_getset!(JweHeader, p2s, set_p2s, Option = "pbes2 salt input");
}

#[test]
fn test_claims_getset_p2c() {
  test_getset!(JweHeader, p2c, set_p2c, Option = 123456789u64);
}
