// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
[
  // https://tools.ietf.org/html/rfc7797#section-4.1
  TestVector {
    detach: false,
    header: br#"{"alg":"HS256"}"#,
    encoded: b"eyJhbGciOiJIUzI1NiJ9.JC4wMg.5mvfOroL-g7HyqJoozehmsaqmvTYGEq5jTI1gVvoEoQ",
    payload: b"$.02",
    public_key: r#"
      {
        "kty": "oct",
        "k": "AyM1SysPpbyDfgZld3umj1qzKObwVMkoqQ-EstJQLr_T-1qS0gZH75aKtMN3Yj0iPS4hcgUuTwjAzZr1Z9CAow"
      }
    "#,
  },
  // https://tools.ietf.org/html/rfc7797#section-4.2
  TestVector {
    detach: true,
    header: br#"{"alg":"HS256","b64":false,"crit":["b64"]}"#,
    encoded: b"eyJhbGciOiJIUzI1NiIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19..A5dxf2s96_n5FLueVuW1Z_vh161FwXZC4YLPff6dmDY",
    payload: b"$.02",
    public_key: r#"
      {
        "kty": "oct",
        "k": "AyM1SysPpbyDfgZld3umj1qzKObwVMkoqQ-EstJQLr_T-1qS0gZH75aKtMN3Yj0iPS4hcgUuTwjAzZr1Z9CAow"
      }
    "#,
  },
]
