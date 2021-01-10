[
  // https://tools.ietf.org/html/rfc7518#appendix-C
  TestVector {
    alice_jwk: r#"
      {
        "kty": "EC",
        "crv": "P-256",
        "x": "gI0GAILBdu7T53akrFmMyGcsF3n5dO7MmwNBHKW5SV0",
        "y": "SLW_xSffzlPWrHEVI30DHM_4egVwt3NQqeUD7nMFpps",
        "d": "0_NxaRPUMQoAJt50Gz8YiTr8gRTwyEaCumd-MToTmIo"
      }
    "#,
    bob_jwk: r#"
      {
        "kty": "EC",
        "crv": "P-256",
        "x": "weNJy2HscCSM6AEDTDg04biOvhFhyyWvOHQfeF_PxMQ",
        "y": "e8lnCO-AlStT-NJVX-crhB7QRYhiix03illJOVAOyck",
        "d": "VEmDZpDXXK8p8N0Cndsxs924q6nS1RXFASRl6BfUqdw"
      }
    "#,
    header: r#"
      {
        "alg": "ECDH-ES",
        "enc": "A128GCM",
        "apu": "QWxpY2U",
        "apv": "Qm9i",
        "epk": {
          "kty": "EC",
          "crv": "P-256",
          "x": "gI0GAILBdu7T53akrFmMyGcsF3n5dO7MmwNBHKW5SV0",
          "y": "SLW_xSffzlPWrHEVI30DHM_4egVwt3NQqeUD7nMFpps"
        }
      }
    "#,
    apu_bytes: b"Alice",
    apv_bytes: b"Bob",
    derived_key_b64: "VqqN6vgjbSBcIijNcacQGg",
  },
]
