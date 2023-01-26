// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
[
  // https://tools.ietf.org/html/rfc8037#appendix-A.4
  TestVector {
    private_jwk: r#"
      {
        "kty": "OKP",
        "crv": "Ed25519",
        "d": "nWGxne_9WmC6hEr0kuwsxERJxWl7MmkZcDusAxyuf2A",
        "x": "11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo"
      }
    "#,
    public_jwk: r#"
      {
        "kty": "OKP",
        "crv": "Ed25519",
        "x": "11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo"
      }
    "#,
    thumbprint_b64: "kPrK_qmxVWaYVA9wwBF6Iuo3vVzz7TxHCTwXBygrS4k",
    header: r#"{"alg":"EdDSA"}"#,
    payload: "Example of Ed25519 signing",
    encoded: "eyJhbGciOiJFZERTQSJ9.RXhhbXBsZSBvZiBFZDI1NTE5IHNpZ25pbmc.hgyY0il_MGCjP0JzlnLWG1PPOt7-09PGcvMg3AIbQR6dWbhijcNR4ki4iylGjg5BhVsPt9g7sVvpAr_MuM0KAg",
  },
]
