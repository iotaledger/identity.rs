use libjose::jwk::Jwk;
use libjose::jwk::JwkType;

#[macro_use]
mod macros;

#[test]
fn test_key_getset_kty() {
  test_getset!(Jwk, kty, set_kty, JwkType::Ec);
}

#[test]
fn test_header_getset_use() {
  test_getset!(Jwk, use_, set_use, Option = "public key use");
}

#[test]
fn test_header_getset_key_ops() {
  test_getset!(Jwk, key_ops, set_key_ops, Option = vec!["key operations"]);
}

#[test]
fn test_header_getset_alg() {
  test_getset!(Jwk, alg, set_alg, Option = "algorithm");
}

#[test]
fn test_header_getset_kid() {
  test_getset!(Jwk, kid, set_kid, Option = "key id");
}

#[test]
fn test_key_getset_x5u() {
  test_getset!(Jwk, x5u, set_x5u, Url = "https://example.com/");
}

#[test]
fn test_key_getset_x5c() {
  test_getset!(
    Jwk,
    x5c,
    set_x5c,
    Option = vec![
      vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
      vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    ]
  );
}

#[test]
fn test_key_getset_x5t() {
  test_getset!(Jwk, x5t, set_x5t, Option = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_key_getset_x5t_s256() {
  test_getset!(
    Jwk,
    x5t_s256,
    set_x5t_s256,
    Option = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
  );
}
