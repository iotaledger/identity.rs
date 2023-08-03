# IOTA Identity - Credentials

This crate contains types representing verifiable credentials and verifiable presentations as defined in the [W3C Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/).

Convenience methods for validating [Verifiable Credentials](https://wiki.iota.org/identity.rs/concepts/verifiable_credentials/overview) and [Verifiable Presentations](https://wiki.iota.org/identity.rs/concepts/verifiable_credentials/verifiable_presentations) are also provided:

- [`JwtCredentialValidator`](crate::validator::JwtCredentialValidator)
- [`JwtPresentationValidator`](crate::validator::JwtPresentationValidator)

The [IOTA Identity Framework Wiki](https://wiki.iota.org/identity.rs/concepts/verifiable_credentials/overview) offers a comprehensive overview of verifiable credentials and presentations along with practical demonstrations and examples showcasing the capabilities of this crate in creating and validating them.
