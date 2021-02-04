[
  TestVector {
    document_unsigned: r##"
      {
        "id": "did:example:123",
        "verificationMethod": [
          {
            "id": "did:example:123#key-1",
            "type": "JcsEd25519Key2020",
            "controller": "did:example:123",
            "publicKeyBase58": "6b23ioXQSAayuw13PGFMCAKqjgqoLTpeXWCy5WRfw28c"
          }
        ],
        "service": [
          {
            "id": "did:schema:id",
            "type": "schema",
            "serviceEndpoint": "https://example.com"
          }
        ]
      }
    "##,
    document_signed: r##"
      {
        "id": "did:example:123",
        "verificationMethod": [
          {
            "id": "did:example:123#key-1",
            "type": "JcsEd25519Key2020",
            "controller": "did:example:123",
            "publicKeyBase58": "6b23ioXQSAayuw13PGFMCAKqjgqoLTpeXWCy5WRfw28c"
          }
        ],
        "service": [
          {
            "id": "did:schema:id",
            "type": "schema",
            "serviceEndpoint": "https://example.com"
          }
        ],
        "proof": {
          "verificationMethod": "#key-1",
          "type": "JcsEd25519Signature2020",
          "signatureValue": "piKnvB438vWsinW1dqq2EYRzcYFuR7Qm9X8t2S6TPPLDokLwcFBXnnERk6jmS8RXKTJnXKWw1Q9oNhYTwbR7vJkaJT8ZGgwDHNxa6mrMNsQsWkM4rg6EYY99xQko7FnpAMn"
        }
      }
    "##,
    secret_b58: "3qsrFcQqVuPpuGrRkU4wkQRvw1tc1C5EmEDPioS1GzQ2pLoThy5TYS2BsrwuzHYDnVqcYhMSpDhTXGst6H5ttFkG",
    signature: "piKnvB438vWsinW1dqq2EYRzcYFuR7Qm9X8t2S6TPPLDokLwcFBXnnERk6jmS8RXKTJnXKWw1Q9oNhYTwbR7vJkaJT8ZGgwDHNxa6mrMNsQsWkM4rg6EYY99xQko7FnpAMn",
  }
]
