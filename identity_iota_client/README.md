IOTA Identity - Client
===

This crate provides interfaces for publishing and resolving DID Documents to and from the Tangle according to the [IOTA DID Method Specification](https://wiki.iota.org/identity.rs/specs/did/iota_did_method_spec).

- [`Client`](crate::tangle::Client)
- [`Resolver`](crate::tangle::Resolver)

Convenience methods for validating [Verifiable Credentials](https://wiki.iota.org/identity.rs/concepts/verifiable_credentials/overview) and [Verifiable Presentations](https://wiki.iota.org/identity.rs/concepts/verifiable_credentials/verifiable_presentations) are also provided:

- [`CredentialValidator`](crate::credential::CredentialValidator)
- [`PresentationValidator`](crate::credential::PresentationValidator)
