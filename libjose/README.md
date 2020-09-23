# libjose

A library for JSON Object Signing and Encryption (JOSE)

## TODO

- [ ] New JWS encoding/decoding API
  - Read to avoid common pitfalls -> [JSON Web Token Best Current Practices](https://tools.ietf.org/html/rfc8725)
  - Need to support JSON-style encoding of JWS tokens (DIDComm)
  - Need to support detached content where applicable (LD-Proofs)
  - Need to support unencoded content where applicable (?)
  - MIGHT need a way to run user-provided content validations during encoding
    - Eg. unencoded content may have context-specific restrictions
  - Validations (confirm RFCs)
    - jwe - Validate `zip` ONLY in protected header
    - jwk - Validate `use` and `key_ops` are consistent
    - jwe/jws - Validate `crit` is not empty list
    - jwe/jws - Validate `crit` ONLY in protected header
    - jws - Validate `b64` ONLY in protected header
    - jws - Validate `crit` contains `b64`

- [ ] Support Nested Tokens
  - Make use of enums for "typ" and "cty" header parameters
  - Provide encoding/decoding conveniences that set/verify these automatically.

- [ ] JSON Web Encryption (everything)
