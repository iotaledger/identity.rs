// Examples from:
//
// https://tools.ietf.org/html/rfc7520
//
use libjose::jwk::Jwk;
use libjose::jwk::JwkParams;
use libjose::jwk::JwkType;
use libjose::jwk::JwkUse;
use serde_json::from_slice;

const FX_JWK_1: &[u8] = include_bytes!("fixtures/cookbook/jwk/3_1_ec_public_key.json");
const FX_JWK_2: &[u8] = include_bytes!("fixtures/cookbook/jwk/3_2_ec_private_key.json");
const FX_JWK_3: &[u8] = include_bytes!("fixtures/cookbook/jwk/3_3_rsa_public_key.json");
const FX_JWK_4: &[u8] = include_bytes!("fixtures/cookbook/jwk/3_4_rsa_private_key.json");
const FX_JWK_5: &[u8] =
  include_bytes!("fixtures/cookbook/jwk/3_5_symmetric_key_mac_computation.json");
const FX_JWK_6: &[u8] = include_bytes!("fixtures/cookbook/jwk/3_6_symmetric_key_encryption.json");

macro_rules! assert_matches {
  ($($tt:tt)*) => {
    assert!(matches!($($tt)*))
  };
}

#[test]
fn test_cookbook_jwk_1() {
  let jwk: Jwk = from_slice(FX_JWK_1).unwrap();
  assert_eq!(jwk.kty(), JwkType::Ec);
  assert_eq!(jwk.kid(), Some("bilbo.baggins@hobbiton.example"));
  assert_eq!(jwk.use_(), Some(&JwkUse::Signature));

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Ec(params) if params.crv == "P-521"
  );

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Ec(params) if params.x == "AHKZLLOsCOzz5cY97ewNUajB957y-C-U88c3v13nmGZx6sYl_oJXu9A5RkTKqjqvjyekWF-7ytDyRXYgCF5cj0Kt"
  );

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Ec(params) if params.y == "AdymlHvOiLxXkEhayXQnNCvDX4h9htZaCJN34kfmC6pV5OhQHiraVySsUdaQkAgDPrwQrJmbnX9cwlGfP-HqHZR1"
  );
}

#[test]
fn test_cookbook_jwk_2() {
  let jwk: Jwk = from_slice(FX_JWK_2).unwrap();
  assert_eq!(jwk.kty(), JwkType::Ec);
  assert_eq!(jwk.kid(), Some("bilbo.baggins@hobbiton.example"));
  assert_eq!(jwk.use_(), Some(&JwkUse::Signature));

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Ec(params) if params.crv == "P-521"
  );

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Ec(params) if params.x == "AHKZLLOsCOzz5cY97ewNUajB957y-C-U88c3v13nmGZx6sYl_oJXu9A5RkTKqjqvjyekWF-7ytDyRXYgCF5cj0Kt"
  );

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Ec(params) if params.y == "AdymlHvOiLxXkEhayXQnNCvDX4h9htZaCJN34kfmC6pV5OhQHiraVySsUdaQkAgDPrwQrJmbnX9cwlGfP-HqHZR1"
  );

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Ec(params) if params.d.as_deref() == Some("AAhRON2r9cqXX1hg-RoI6R1tX5p2rUAYdmpHZoC1XNM56KtscrX6zbKipQrCW9CGZH3T4ubpnoTKLDYJ_fF3_rJt")
  );
}

#[test]
fn test_cookbook_jwk_3() {
  let jwk: Jwk = from_slice(FX_JWK_3).unwrap();
  assert_eq!(jwk.kty(), JwkType::Rsa);
  assert_eq!(jwk.kid(), Some("bilbo.baggins@hobbiton.example"));
  assert_eq!(jwk.use_(), Some(&JwkUse::Signature));

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Rsa(params) if params.n == "n4EPtAOCc9AlkeQHPzHStgAbgs7bTZLwUBZdR8_KuKPEHLd4rHVTeT-O-XV2jRojdNhxJWTDvNd7nqQ0VEiZQHz_AJmSCpMaJMRBSFKrKb2wqVwGU_NsYOYL-QtiWN2lbzcEe6XC0dApr5ydQLrHqkHHig3RBordaZ6Aj-oBHqFEHYpPe7Tpe-OfVfHd1E6cS6M1FZcD1NNLYD5lFHpPI9bTwJlsde3uhGqC0ZCuEHg8lhzwOHrtIQbS0FVbb9k3-tVTU4fg_3L_vniUFAKwuCLqKnS2BYwdq_mzSnbLY7h_qixoR7jig3__kRhuaxwUkRz5iaiQkqgc5gHdrNP5zw"
  );

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Rsa(params) if params.e == "AQAB"
  );
}

#[test]
fn test_cookbook_jwk_4() {
  let jwk: Jwk = from_slice(FX_JWK_4).unwrap();
  assert_eq!(jwk.kty(), JwkType::Rsa);
  assert_eq!(jwk.kid(), Some("bilbo.baggins@hobbiton.example"));
  assert_eq!(jwk.use_(), Some(&JwkUse::Signature));

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Rsa(params) if params.n == "n4EPtAOCc9AlkeQHPzHStgAbgs7bTZLwUBZdR8_KuKPEHLd4rHVTeT-O-XV2jRojdNhxJWTDvNd7nqQ0VEiZQHz_AJmSCpMaJMRBSFKrKb2wqVwGU_NsYOYL-QtiWN2lbzcEe6XC0dApr5ydQLrHqkHHig3RBordaZ6Aj-oBHqFEHYpPe7Tpe-OfVfHd1E6cS6M1FZcD1NNLYD5lFHpPI9bTwJlsde3uhGqC0ZCuEHg8lhzwOHrtIQbS0FVbb9k3-tVTU4fg_3L_vniUFAKwuCLqKnS2BYwdq_mzSnbLY7h_qixoR7jig3__kRhuaxwUkRz5iaiQkqgc5gHdrNP5zw"
  );

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Rsa(params) if params.e == "AQAB"
  );

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Rsa(params) if params.d.as_deref() == Some("bWUC9B-EFRIo8kpGfh0ZuyGPvMNKvYWNtB_ikiH9k20eT-O1q_I78eiZkpXxXQ0UTEs2LsNRS-8uJbvQ-A1irkwMSMkK1J3XTGgdrhCku9gRldY7sNA_AKZGh-Q661_42rINLRCe8W-nZ34ui_qOfkLnK9QWDDqpaIsA-bMwWWSDFu2MUBYwkHTMEzLYGqOe04noqeq1hExBTHBOBdkMXiuFhUq1BU6l-DqEiWxqg82sXt2h-LMnT3046AOYJoRioz75tSUQfGCshWTBnP5uDjd18kKhyv07lhfSJdrPdM5Plyl21hsFf4L_mHCuoFau7gdsPfHPxxjVOcOpBrQzwQ")
  );

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Rsa(params) if params.p.as_deref() == Some("3Slxg_DwTXJcb6095RoXygQCAZ5RnAvZlno1yhHtnUex_fp7AZ_9nRaO7HX_-SFfGQeutao2TDjDAWU4Vupk8rw9JR0AzZ0N2fvuIAmr_WCsmGpeNqQnev1T7IyEsnh8UMt-n5CafhkikzhEsrmndH6LxOrvRJlsPp6Zv8bUq0k"
  ));

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Rsa(params) if params.q.as_deref() == Some("uKE2dh-cTf6ERF4k4e_jy78GfPYUIaUyoSSJuBzp3Cubk3OCqs6grT8bR_cu0Dm1MZwWmtdqDyI95HrUeq3MP15vMMON8lHTeZu2lmKvwqW7anV5UzhM1iZ7z4yMkuUwFWoBvyY898EXvRD-hdqRxHlSqAZ192zB3pVFJ0s7pFc")
  );

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Rsa(params) if params.dp.as_deref() == Some("B8PVvXkvJrj2L-GYQ7v3y9r6Kw5g9SahXBwsWUzp19TVlgI-YV85q1NIb1rxQtD-IsXXR3-TanevuRPRt5OBOdiMGQp8pbt26gljYfKU_E9xn-RULHz0-ed9E9gXLKD4VGngpz-PfQ_q29pk5xWHoJp009Qf1HvChixRX59ehik")
  );

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Rsa(params) if params.dq.as_deref() == Some("CLDmDGduhylc9o7r84rEUVn7pzQ6PF83Y-iBZx5NT-TpnOZKF1pErAMVeKzFEl41DlHHqqBLSM0W1sOFbwTxYWZDm6sI6og5iTbwQGIC3gnJKbi_7k_vJgGHwHxgPaX2PnvP-zyEkDERuf-ry4c_Z11Cq9AqC2yeL6kdKT1cYF8")
  );
}

#[test]
fn test_cookbook_jwk_5() {
  let jwk: Jwk = from_slice(FX_JWK_5).unwrap();
  assert_eq!(jwk.kty(), JwkType::Oct);
  assert_eq!(jwk.kid(), Some("018c0ae5-4d9b-471b-bfd6-eef314bc7037"));
  assert_eq!(jwk.use_(), Some(&JwkUse::Signature));
  assert_eq!(jwk.alg(), Some("HS256"));

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Oct(params) if params.k == "hJtXIZ2uSN5kbQfbtTNWbpdmhkV8FJG-Onbc6mxCcYg"
  );
}

#[test]
fn test_cookbook_jwk_6() {
  let jwk: Jwk = from_slice(FX_JWK_6).unwrap();
  assert_eq!(jwk.kty(), JwkType::Oct);
  assert_eq!(jwk.kid(), Some("1e571774-2e08-40da-8308-e8d68773842d"));
  assert_eq!(jwk.use_(), Some(&JwkUse::Encryption));
  assert_eq!(jwk.alg(), Some("A256GCM"));

  assert_matches!(
    jwk.params().unwrap(),
    JwkParams::Oct(params) if params.k == "AAPapAv4LbFbiVawEjagUBluYqN5rhna-8nuldDvOx8"
  );
}
