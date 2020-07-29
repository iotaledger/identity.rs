# Identity.rs 

This is a work-in-progress library for Digital Identity on IOTA. It follows the Decentralized Identifiers (DIDs) and Verifiable Credentials standards created by the W3C. The concept of digital identity allows people, businesses, devices and anything else to identify themselves online, while remaining fully in control of this process.

**WARNING: THE CURRENT VERSION IS FEATURE INCOMPLETE AND WILL STILL UNDERGO MASSIVE CHANGES**
If you are interested in using this project or contributing, join our [Discord](https://discord.iota.org) and visit the channel #identity-dev. 

## Prerequisits
TODO

## Decentralized Identifers (DID)

This DID implementation is based on [v0.13 of the DID specification from W3C](https://w3c-ccg.github.io/did-spec/).
DID's are authenticated using the [DID-Authentication protocol](https://github.com/WebOfTrustInfo/rwot6-santabarbara/blob/master/final-documents/did-auth.md), which proves to an inspection party that they are communicating with the owner of the DID.
[According to the DID specification](https://w3c-ccg.github.io/did-spec/#did-documents) a DID Document is outputted when a DID is resolved. 
This DID Document may be stored on IOTA, however this is immutabily stored and **might** contain personal data according to the GDPR. 
It is therefore recommended that any DID's that represent people, will not be published on the Tangle, while issueing entities and devices should publish these to IOTA. 

To create, retrieve and manage DID Documents look at the [DID Documention](src/DID/README.md).

## Verifiable Credentials 

Verifiable Credentials are implemented according to the [Verifiable Credentials Data Model 1.0 by W3C Community Group](https://www.w3.org/TR/vc-data-model/) standard.
Verifiable Credentials works closely together with the DID standard. Where a DID can just be authenticated, Verifiable Credentials can add verifiable attributes to the identifier. 
The acquisition, communication, management and storage of Verifiable Credentials are out of the scope of this implementation. 
For a general introduction to the concept, please [read the explanation on the specification page](https://www.w3.org/TR/vc-data-model/#what-is-a-verifiable-credential).

To create and verify Verifiable Credentials look at the [Verifiable Credentials Documentation](src/VC/README.md).

## Verifiable Presentations

To prevent a replay-attack where another party can also pass on the credential as if it is talking about their DID, Verifiable Presentation are introduced. 
The [Verifiable Presentation data model](https://www.w3.org/TR/vc-data-model/#presentations) groups a set of excisting Verifiable Credentials of the subject together for the inspecting party and adds a signature, including a challenge from the inspecting party. It is therefore recommended to not communicate credentials directly, but rather presentations.

## Schematics of Credentials

TODO: Describe schematics

## Encryption techniques

TODO: Describe current Encryption techniques and wanted / planned techniques.

## Future of this project

Identity will be used for:
- Replace physical documents
- Improved KYC
- Replace passwords
- IoT Security
- Access Control
- Trust
- Smart Cities
- Vehicle Identities (VID)

## API Reference

TODO: Add Module overview