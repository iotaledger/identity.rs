use libjose::jwt::JwtClaims;

#[macro_use]
mod macros;

#[test]
fn test_claims_getset_iss() {
  test_getset!(JwtClaims, iss, set_iss, Option = "issuer");
}

#[test]
fn test_claims_getset_sub() {
  test_getset!(JwtClaims, sub, set_sub, Option = "subject");
}

#[test]
fn test_claims_getset_aud() {
  test_getset!(JwtClaims, aud, set_aud, Option = vec!["audience"]);
}

#[test]
fn test_claims_getset_exp() {
  test_getset!(JwtClaims, exp, set_exp, Option = 123456789);
}

#[test]
fn test_claims_getset_nbf() {
  test_getset!(JwtClaims, nbf, set_nbf, Option = 123456789);
}

#[test]
fn test_claims_getset_iat() {
  test_getset!(JwtClaims, iat, set_iat, Option = 123456789);
}

#[test]
fn test_claims_getset_jti() {
  test_getset!(JwtClaims, jti, set_jti, Option = "jwt id");
}
