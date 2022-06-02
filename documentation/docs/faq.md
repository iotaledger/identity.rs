---
title: Frequently Asked Questions
sidebar_label: FAQ
description: Frequently Asked Question regarding IOTA Identity.
image: /img/Identity_icon.png
keywords:
- FAQ
- Frequently Asked Question
- Troubleshooting
- IOTA Identity
---

This page contains frequently asked questions regarding the Identity Library and Self Sovereign Identity in general.

### What programming languages are supported by the IOTA Identity framework?
We currently provide a Rust library and a JavaScript library for both the browser and Node.js via WebAssembly (Wasm) bindings. See the "Programming Languages" section for more information.
### Do I need to have IOTA tokens to start building with IOTA Identity?
At the moment you don't need IOTA tokens to create and manage identities, although we are exploring opportunities to utilize the token in the future. 
### How do I prove control over my DID?
Control over an identity is ultimately tied to the control over cryptographic key material (something you have).
### How do I store my private keys?
Theoretically you can store the keys however you like. We provide a secure default using IOTA Stronghold where possible, which is a secure software implementation for isolating digital secrets with encrypted storage. For even better guarantees you could look into hardware based key storage.
### Do I need a Permanode to use IOTA Identity?
Current you need to have access to a Permanode (a node with all the history of the tangle) to correctly resolve identities. The trustworthiness of this node is very important, since a malicious node could flat out respond with made up data, so ideally you run that node yourself or make sure the party supplying the node is trustworthy.
### Can I use IOTA Identity on Android or iOS?
We currently do not supply dedicated bindings for Kotlin or Swift. There has been some success running the Wasm bindings on mobile, however.
### Can I use IOTA Identity on embedded devices?
We currently do not supply dedicated bindings catering to embedded devices with restricted capabilities. You can try to compile the Rust library for your target platform or use a gateway in front of the devices to handle IOTA Identity interactions.
### What should I do if my private key is compromised?
If you still have control over your identity, rotate the key material ASAP! If an attacker has locked you out of your identity, there is not much you can do. Notify contacts that your identity has been compromised and start fresh with a new one.

### Are credentials stored on the tangle?
Credentials are supposed to be stored on user devices or systems. As a user you are in charge of storing your credentials and sharing them with other parties on a need-to-know basis.

### Do I need to hide my DID? Will people be able to identify me by my DID?
A DID should not contain any information linking to you as a person, there is the chance of entities monitoring your movement using a DID though. To minimize this risk it is advisable to use different DIDs for different use case.
