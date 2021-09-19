---
title: Verifiable Credentials Overview
sidebar_label: Overview
description: Verifiable Credentials are statements about the holder. They can be verified online or in person, and the holder decides who to share them with.
image: /img/Identity_icon.png
keywords:
- verifiable
- credentials
- person
---

Credentials are statements about an entity, such as properties that the entity posseses or capabilities that they have (like drivers licences, passports, or a person's age). Verifiable Credentials (VCs) are statements about the holder of the credential that can be cryptographically verified by a third party, either online or in person. Additionally, the holder decides what is shared and who it is shared.

There are several types of actors that play different roles in a verifiable credential system. We'll start with a common example from the world today and outline the roles that various entities play.

:::tip Example - Passport Issuance

A government (the _Issuer_) issues a passport asserting citizenship (the _Verifiable Credential_) to Alice (the _Subject_ and _Holder_), and writes the information to a database (the _Verifiable Data Registry_). When crossing the border, Alice (the _Holder_) presents her passport to a border agent (the _Verifier_) who can verify that Alice (the _Subject_) is indeed a citizen.

:::

**Subject:** An entity about which claims are made – Alice (the _Subject_) is a citizen of this country.

**Holder:** An entity which posseses verifiable credentials – Alice (the _Holder_) posseses the passport (the _VC_).

**Issuer:** An entity which asserts claims about a subject – The governing body (the _Issuer_), which is trusted, issues Alice a passport.

**Verifier:** An entity which check's if the VC a holder presents is legitimate – The border agent (the _Verifier_) trusts the government (the _Issuer_) which issued Alice her passport, and validates that Alice (the _Subject_) is a citizen.

:::note

See the [Verifiable Credentials Data Model 1.0 Specification](https://w3c.github.io/vc-data-model/) for more information.

:::

### Verifiable Credentials in IOTA

In the IOTA Identity framework, instead of a physical passport being given to Alice with the passport information being written into a centralized database owned by the government, Alice receives a digital verifiable credential, and the information required for verification in the future is written to the Tangle.

At a high level, the creation and verification of a VC on IOTA works as follows:

The first step is to create a verifiable credential which requires the subject (Alice) and issuer (the government) to have DIDs published to the tangle, and a set of statements being asserted (that Alice has a passport). The issuer then publishes a credential to the Tangle which contains the `credentialSubject` (Alice's DID in our example), the set of statements, and a proof that can be used to assert the authenticity of the credentials by verifiers.

After the credential is published to the tangle, validation is performed by looking up the credential on the tangle, the holder proving ownership of their DID to the verifier (evidence), and validating that the credential has indeed been signed by the issuing party.

The remaining chapters in this section explore creation, verification, and revocation of VCs in more detail.