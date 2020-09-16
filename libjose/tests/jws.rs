use libjose::jwk::Jwk;
use libjose::jws::JwsAlgorithm;
use libjose::jws::JwsHeader;

#[macro_use]
mod macros;

#[test]
fn test_header_getset_alg() {
  test_getset!(JwsHeader, alg, set_alg, JwsAlgorithm::EdDSA);
}

#[test]
fn test_header_getset_jku() {
  test_getset!(JwsHeader, jku, set_jku, Url = "https://example.com/");
}

#[test]
fn test_header_getset_jwk() {
  test_getset!(JwsHeader, jwk, set_jwk, OptionRef = Jwk::new());
}

#[test]
fn test_header_getset_kid() {
  test_getset!(JwsHeader, kid, set_kid, Option = "key id");
}

#[test]
fn test_header_getset_x5u() {
  test_getset!(JwsHeader, x5u, set_x5u, Url = "https://example.com/");
}

#[test]
fn test_header_getset_x5c() {
  test_getset!(
    JwsHeader,
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
    JwsHeader,
    x5t,
    set_x5t,
    Option = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
  );
}

#[test]
fn test_header_getset_x5t_s256() {
  test_getset!(
    JwsHeader,
    x5t_s256,
    set_x5t_s256,
    Option = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
  );
}

#[test]
fn test_header_getset_typ() {
  test_getset!(JwsHeader, typ, set_typ, Option = "type");
}

#[test]
fn test_header_getset_cty() {
  test_getset!(JwsHeader, cty, set_cty, Option = "content type");
}

#[test]
fn test_header_getset_crit() {
  test_getset!(JwsHeader, crit, set_crit, Option = vec!["critical"]);
}

#[test]
fn test_header_getset_b64() {
  test_getset!(JwsHeader, b64, set_b64, Option = false);
}

#[test]
fn test_header_getset_url() {
  test_getset!(JwsHeader, url, set_url, Url = "https://example.com/");
}

#[test]
fn test_header_getset_nonce() {
  test_getset!(
    JwsHeader,
    nonce,
    set_nonce,
    Option = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
  );
}

#[test]
fn test_header_getset_ppt() {
  test_getset!(JwsHeader, ppt, set_ppt, Option = "passport");
}
