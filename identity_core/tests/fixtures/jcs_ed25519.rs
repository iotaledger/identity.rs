[
  TestVector {
    public: "4CcKDtU1JNGi8U4D8Rv9CHzfmF7xzaxEAPFA54eQjRHF",
    private: "z3nisqMdwW7nZdWomCfUyRaezHzKEBfwRbvaMcJAqaMSbmjxuRfS5qz4ff3QAf1A5sZT4ZMcxYoGjN4K1VsDV7b",
    input: r#"
      {
        "id": "did:example:abcd",
        "publicKey": [
          {
            "id": "did:example:abcd#key-1",
            "type": "JcsEd25519Signature2020",
            "controller": "foo-issuer",
            "publicKeyBase58": "not-a-real-pub-key"
          }
        ],
        "authentication": null,
        "service": [
          {
            "id": "schema-id",
            "type": "schema",
            "serviceEndpoint": "service-endpoint"
          }
        ],
        "proof": {
          "type": "JcsEd25519Signature2020"
        }
      }
    "#,
    output: r#"
      {
        "id": "did:example:abcd",
        "publicKey": [
          {
            "id": "did:example:abcd#key-1",
            "type": "JcsEd25519Signature2020",
            "controller": "foo-issuer",
            "publicKeyBase58": "not-a-real-pub-key"
          }
        ],
        "authentication": null,
        "service": [
          {
            "id": "schema-id",
            "type": "schema",
            "serviceEndpoint": "service-endpoint"
          }
        ],
        "proof": {
          "signatureValue": "4qtzqwFxFYUifwfpPhxR6AABn94KnzWF768jcmjHHH8JYtUb4kAXxG6PttmJAbn3b6q1dfraXFdnUc1z2EGHqWdt",
          "type": "JcsEd25519Signature2020"
        }
      }
    "#,
  },
  TestVector {
    public: "4CcKDtU1JNGi8U4D8Rv9CHzfmF7xzaxEAPFA54eQjRHF",
    private: "z3nisqMdwW7nZdWomCfUyRaezHzKEBfwRbvaMcJAqaMSbmjxuRfS5qz4ff3QAf1A5sZT4ZMcxYoGjN4K1VsDV7b",
    input: r#"
      {
        "foo": "bar",
        "proof": {
          "type": "JcsEd25519Signature2020"
        }
      }
    "#,
    output: r#"
      {
        "foo": "bar",
        "proof": {
          "signatureValue": "4VCNeCSC4Daru6g7oij3QxUL2CS9FZkCYWRMUKyiLuPPK7GWFrM4YtYYQbmgyUXgGuxyKY5Wn1Mh4mmaRkbah4i4",
          "type": "JcsEd25519Signature2020"
        }
      }
    "#,
  }
]
