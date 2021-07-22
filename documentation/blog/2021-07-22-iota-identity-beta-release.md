---
slug: IOTA Identity Beta Release
title: IOTA Identity Beta Release
author: Jelle Millenaar
author_title: Lead Identity @ IOTA Foundation
author_url: https://github.com/JelleMillenaar
author_image_url: https://avatars.githubusercontent.com/u/29886335?v=4
tags: [Beta, Account, DID Comm, Communications]
---


We’re thrilled to announce the release of IOTA Identity Beta. This marks the framework as “feature complete” on the road to a 1.0 version. It includes several major milestones such as the upgrade to Chrysalis Phase 2, a simplified higher-level API including Stronghold support and an initial implementation of the DID Communications messages based on the Decentralized Identity Foundation (DIF) standard and Hyperledger Aries work.

This builds on the existing work that was done for the Alpha release, in which we implemented the W3C standards for Decentralized Identifiers (DID) and Verifiable Credentials. We have further refined this implementation and have submitted our DID Method to W3C to be listed as one of the DID Method specifications.

## IOTA Identity Recap
IOTA Identity is a Self Sovereign Identity (SSI) framework that enables people, organizations, or machines to create and have complete control over a digital identity - without the permission of an intermediary or central party. In addition, it gives control over how personal data is shared and used.

Social media platforms like Facebook and Linkedin show us every day that online accounts are heavily reliant on user-submitted content. The information provided on these platforms are not trustworthy and can easily be faked - we want to prevent that with our digital identity solution.

This is where Verifiable Credentials (VCs) come in. These are digital statements about an identity aspect like “name”, digitally signed by another identity, that has authority on the subject. For example: My bachelor's degree is trustworthy if a university signs it, but not so much if it is signed by your mother. Such statements make data reliable and valuable. VCs allow a SSI solution to not just be more privacy friendly, but also compete with Facebook / LinkedIn profiles as they are more trustworthy and accurate.

We want to take that concept one step further by applying it to devices. Countless IoT devices and sensors interact with each other every day. Large amounts of data are generated as a result, but so far, programs have rarely used data from 3rd party devices for decision-making. One of the reasons we do not, is that the origin of data and ultimately the trust in a device can not sufficiently be determined with traditional technology. IOTA Identity allows IoT devices to create their own identities and collect verifiable statements about themselves, such for example as:

* The manufacturer of the device
* The software installed and by whom
* Certificate of calibration
* Statements from other identities that used the data and found it to be accurate (Reviews)
We enable IoT devices to build a reputation and become trustworthy. We can then leverage the data provided by the devices to make decisions. Think about the traffic flow in a city:

Right now, a city only trusts sensors it has deployed itself to determine traffic flow. Imagine how much more information, and in turn higher precision, could be achieved if cars can identify themselves and share their generated data. This would constitute the groundwork for a trusted data economy, enabled by IOTA Identity. Without providing for an architecture that enables the creation of trust, anyone could fake traffic data and cause havoc in an automated traffic system relying on external data. Trusted identities therefore are the basis for making any use of any data that is being generated. Any digital system benefits from trust, not just traffic systems like in the example above.

## Identity Account
The first major feature we introduced with this beta release is the account, a higher-level API to use IOTA Identity. Similar to the recent Chrysalis update, IOTA Identity becomes a lot easier to use by utilizing the account. It is intended to provide a much more simplified interface that is perfect for 90%+ of the use cases. The other 10% are more complex use cases that may still want to utilize the lower-level APIs to have more control over the Identities. The account doesn’t just simplify the interactions with DID Documents, but also with the Tangle and Stronghold.

Identity messages on the Tangle are quite complicated. We have two formats in order to optimize DID resolution times and they need to be created and used in a specific way. With the account, a single line of code takes care of all the message formatting and publishing to the Tangle. Similarly, creating an identity is a rather complicated process as you first need to generate a keypair, from which you derive a DID. Afterwards, a DID Document is created, in which the public key must be embedded and this must be published to the Tangle. Lastly, the private key needs to be stored and managed in a secure manner. The account does all this in a single line of code, storing the private key inside a Stronghold, or a different storage method defined by the developer.

Currently the account feature is only available in Rust and not yet available as Javascript bindings as this requires some bigger design improvements in the project that we want to tackle during our road towards a 1.0 version.

## DID Communications
The other major feature that we have implemented is the DID Communications messages. These are standardized messages that two IOTA Identity actors would send to each other to do anything related to identity. For example, they can ask each other to prove control over their identities, but also request verifiable credentials to be shared or even signed. Using these messages doesn’t just make it easier to create IOTA Identity powered applications, but also creates immediate interoperability between applications. As such, an IOTA Identity app made by one company could interact with an app made by another.

So far, we have defined our version of the exact message layout and created a basic example actor on how to automatically generate and respond to DID Communications messages in Rust. The concept will get iterated upon and Javascript bindings will be added before the 1.0 release.

## Next Steps
As mentioned, IOTA Identity will continue to be refined and improved for a 1.0 release, with a focus on performance, code quality, documentation and javascript support. The upcoming 1.0 release will mark a very important milestone for identity as we will support backwards compatibility and version transitioning identities moving forward, making it possible to build production ready applications on top of IOTA Identity and easily upgrade to newer versions in the future.

While our 1.0 release is the next big milestone, the journey for IOTA Identity doesn’t end there. We have many ideas for additional features for future updates such as privacy enhancements, identity agents, but also provide more use case specific libraries to make IOTA Identity easy to adopt on the more promising use cases. In fact, we are very happy to soon start working together with the LINKS Foundation, who are creating a Zero Knowledge Proof (ZKP) specification for IOTA Identity, massively increasing the privacy features of IOTA Identity. They are experienced cybersecurity researchers that will provide a specification based on established scientific literature results.

While IOTA Identity is not widely known yet, with the beta release we are closing in and even overtaking some of the popular SSI frameworks in quality and feature sets. We will continue to focus on developing an amazing framework that is secure, performant but also easy-to-use. If you are interested in working with IOTA Identity, please check out our repository and consider joining the IOTA Identity X-team, where like minded developers, entrepreneurs and students meet on with IOTA Identity devs on a weekly basis to discuss the framework in a relaxing yet educational atmosphere.

In order to achieve the Alpha release, the IOTA Foundation was supported through an EDF funded group of developers: Thoralf, Sebastian (huhn) and Tensor. This time we have to thank Filancore, a startup from within the IOTA community, for their support in making the Beta release possible. While we really enjoy working with community members, we are also excited to be growing the team significantly in order to continue developing the IOTA Identity framework, developer tooling, bindings to other languages and Proof-of-Concepts with our own manpower.

## Full Changelog
This is an incomplete changelog for the changes between version 0.2 alpha and 0.3 beta.

Significant features

* Implemented Account (#151)
* Implemented DID Communication messages and envelopes (#226)
* Changed IOTA code to work only on Chrysalis Phase 2 networks (#161)
* Renamed types and files to reduce confusion between agnostic DID implementation and did:iota implementation. Contributed by m-renaud! (#219, #233, #237, #243)
* Added Libjose implementation (#176)

Documentation / Specification

* Added DID Method specification for the did:iota method and submitted it to W3C (#207)
* Added DID Communication specification (#178, #186, #187, #202, #203)
* Lots of documentation improvements (#174, #164, #171)

Minor changes

* Changed Cryptographic suites to work with Stronghold (#158)
* Merged Verifiable Credential with Credential type to simplify the code (#170)
* Merged Verifiable Presentation with Presentation type to simplify the code (#170)
* Replaced 'immutable' property from DID Document with DID URL query parameter (#141)
* Made Service Endpoints more accessible (#194)
* Added Benchmarking tests (#223)
* Improved examples (#171, #251)