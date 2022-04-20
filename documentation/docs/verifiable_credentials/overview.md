---
title: Verifiable Credentials Overview
sidebar_label: Overview
description: Verifiable Credentials are statements about the holder. They can be verified online or in person and the holder decides who to share them with.
image: /img/Identity_icon.png
keywords:
- verifiable
- credentials
- person
- reference
---

Credentials are statements about an entity, such as properties that the entity possesses or capabilities that they have, like a driver's license, a passport, or a someone's age. Verifiable Credentials (VCs) are statements that can be cryptographically verified by a third party, either online or in person, like someone having a driver's license. The holder of the VC can then decide what is shared and who to share it with.

There are several types of actors that play different roles in a verifiable credential system. This article explains how things work today and how physical credentials and centralized databases are used while outlining the roles that various entities play in the Verifiable Credential system, starting with a common example.

:::note Example - Passport Issuance

A government (the _Issuer_) issues a passport asserting citizenship (the _Verifiable Credential_) to Alice (the _Subject_ and _Holder_), and writes the information to a database (the _Verifiable Data Registry_). When crossing the border, Alice (the _Holder_) presents her passport to a border agent (the _Verifier_) who can verify that Alice (the _Subject_) is indeed a citizen.

:::

**Subject:** An entity about which claims are made – Alice (the _Subject_) is a citizen of this country.

**Holder:** An entity which owns verifiable credentials – Alice (the _Holder_) owns the passport (the _VC_).

**Issuer:** An entity which asserts claims about a subject – The governing body (the _Issuer_), which is trusted, issues Alice a passport.

**Verifier:** An entity which checks if the VC a holder presents is legitimate – The border agent (the _Verifier_) trusts the government (the _Issuer_) which issued Alice her passport and validates that Alice (the _Subject_) is a citizen.

:::note

For more information, you can check out the [Verifiable Credentials Data Model 1.0 Specification page](https://w3c.github.io/vc-data-model/).

:::

### Verifiable Credentials in IOTA

In the IOTA Idenitity framework, Alice would receive a digitally verifiable credential rather than a physical passport. Rather than the passport information being written into a centralized database owned by the government for verfication, the digitial credential is (in the future) written to the Tangle.

The first step in the creation and verification of a VC is to create a verifiable credential. This requires the subject and issuer (Alice and the government, respectively) to have DIDs published to the Tangle with a set of statements being asserted (that Alice has a passport). The issuer signs the credential with their private key and publishes the public key to the Tangle. In the future, a proof can be used by the verifiers to validate the authenticity of the credentials using the issuer's public key.

Anyone can verify the credentials by looking up the key, checking the holder's DID, and the signature of the issuing party.

The remaining chapters in this section explore creation, verification, and revocation of VCs in more detail.