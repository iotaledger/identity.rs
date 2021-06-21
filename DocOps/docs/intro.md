---
sidebar_position: 1
title: Introduction
---

# IOTA Identity Framework Guide

![IOTA Identity](https://github.com/iotaledger/identity.rs/raw/dev/.meta/identity_banner.png)

The IOTA Identity framework implements the most common standards and patterns for Decentralized Identity in both a DLT agnostic and `iota` method specification manner. It is designed to work for Identity for People, Organizations, Things, and Objects acting as a unifying-layer of trust between everyone and everything.

In this guide, we will go through the most important concepts that developers will need to know to utilize IOTA Identity to its full potential. The guide is programming language agnostic, with langauge specific guides located in Chapter 4.

### Overview

**Chapter 2: Decentralized Identity**

Describes the concept of Decentralized or Self Sovereign Identities (SSI), how it applies to People, Organizations and Things, and why IOTA is used. 

**Chapter 3.1: Decentralized Identifiers (DID)**

Explains the DID standard from W3C, how to manipulate DID Documents and Merkle Key Collections, the basis of our revocation mechanism.

**Chapter 3.2: Verifiable Credentials (VC)**

Explains the VC standard from W3C, how to create and revoke VCs and how to use Verifiable Presentations.

**Chapter 3.3: DID Communications (DID Comm)**

This chapter covers the DID Comm standard, which is being developed by the Decentralized Identity Foundation (DIF). It also describes the different messages agents may sent each other and what the expected responses may look like.

**Chapter 3.4: Advanced Concepts**

This chapter is ment for those that want to push the IOTA Identity framework to its limits, utilizing the more complex, yet more flexible lower level libraries, allowing developers to optimize their implementation, take control over storage/security, and add features to the framework. 

**Chapter 4: Programming Languages**

While the framework itself is developed in the Rust programming language, we also provide bindings, or "Foreign Function Interfaces" (FFI), to other languages. These will have seperate getting started sections, making the rest of the guide language agnostic, focusing on the conceptual level. 

**Chapter 5: Specification**

While IOTA Identity implements many existing standards, it also adds some additional features we would like to standardize ourselves. This chapter covers these features and how they work in great detail. These are not light reads and can be skipped. 


**Chapter 6: Glossary**

A list of all terminology used in this guide, the framework and all materials surrounding it. 

**Chapter 7: Contribute**

A simple guide on how to contribute to the framework.

**Chapter 8: Contact**

How to contact the maintainers.

**Chapter 9: FAQ**

Overview of the most Frequently Asked Questions, and their answers.
