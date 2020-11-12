![banner](./.meta/identity_banner.png)

<p align="center">
  <a href="https://discord.iota.org/" style="text-decoration:none;"><img src="https://img.shields.io/badge/Discord-9cf.svg?logo=discord" alt="Discord"></a>
  <a href="https://iota.stackexchange.com/" style="text-decoration:none;"><img src="https://img.shields.io/badge/StackExchange-9cf.svg?logo=stackexchange" alt="StackExchange"></a>
  <a href="https://github.com/iotaledger/identity.rs/blob/master/LICENSE" style="text-decoration:none;"><img src="https://img.shields.io/github/license/iotaledger/bee.svg" alt="Apache 2.0 license"></a>
</p>

<p align="center">
  <a href="#introduction">Introduction</a> ◈
  <a href="#warning">Warning</a> ◈
  <a href="#planned-milestones">Planned Milestones</a> ◈  
  <a href="#roadmap">Roadmap</a> ◈
  <a href="#joining-the-discussion">Joining the discussion</a>
</p>

---

## Introduction

IOTA Identity is an implementation of decentralized digital identity also known as Self Sovereign Identity (SSI). It implements standards such as [DID](https://www.w3.org/TR/did-core/) and [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/) from W3C and other related (proposed) standards. This framework can be utilized to create and authenticate digital identities, creating a trusted connection and sharing verifiable information, establishing trust in the digital world. 

The individual libraries are developed to be agnostic of Distributed Ledger Technology (DLT), with the exception of the IOTA integration and higher level libraries. Written in stable rust, it has strong guarantees of memory safety, process integrity while maintaining performance. 

## Warning

This library is currently under development and might undergo large changes. It is currently in its alpha stage. Until a formal third-party security audit has taken place, the IOTA Foundation makes no guarantees to the fitness of this library.

As such they are to be seen as **experimental** and not ready for real-world applications.

Nevertheless, we are very interested in feedback about the design and implementation, and encourage you to reach out with any concerns or suggestions you may have.

## Planned Milestones

At the current state, the framework is not fit for any projects, however as the framework matures we expect to support more and more type of applications. We recommend no use in real-world applications until the consumed libraries are audited, but experimentation and Proof-of-Concept projects are encouraged at the different stages.

**Current Stage: 2**

**Stage 1: DID (Q4 2020)**

As the DID standard is implemented and the IOTA ledger is integrated the first experimentations are possible. DIDs can be created, updated and ownership can be proven. This allows simple experimentations where ownership of an identity is the main requirement. 

**Stage 2: Verifiable Credentials (Q4 2020)**

With the Verifiable Credentials standard implemented, not only ownership can be proven, but also other attributes. At this stage PoCs are possible similarly to [Selv](https://selv.iota.org). However, the communications between actors are not yet implemented, identities are not easily recognized nor are credential layouts standardized. Real-world applications are possible at this stage (after audit), but require extra effort.

**Stage 3: Communication Standardization (Q1 2021)**

Once the communications between DID actors have been implemented, any application using identity can communicate out-of-the-box in an interoperable manner. This makes applications easier to develop, yet as mentioned in Stage 2, identities are still not easily recognized nor are the credential layouts standarized. Real-world applications are therefore easier to develop (after audit), but scaling the application outside of a consortium is difficult.

Stage 4: TBD

## Roadmap

### Documentation and Specification
- [x] Examples
- [ ] Specification Documentation

### Basic Framework
- [x] DID Document Manager
- [x] IOTA Integration
- [x] Resolver
- [ ] Stronghold Integration
- [ ] DID Comms
- [x] Verifiable Credentials
- [ ] VC Comms
- [ ] Schema Validation
- [ ] C FFI Bindings
- [x] Javascript FFI Bindings

### Extended Features (2021+)
- [ ] Mobile App Wrapper
- [ ] Credential standardization
- [ ] Identity Agent
- [ ] Pairwise DIDs
- [ ] Zero Knowledge Proofs
- [ ] Trust Fabric
- [ ] eId Integrations
- [ ] IoT reputation system
- [ ] Identity for Objects

## Joining the discussion

If you want to get involved in discussions about this framework, or you're looking for support, go to the #identity-discussion channel on [Discord](http://discord.iota.org).
