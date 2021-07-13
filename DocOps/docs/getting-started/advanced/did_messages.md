---
title: DID Messages
sidebar_label: DID Messages
---

TODO: Explain the concept of DID Messages and how they can be used to optimize DID updates.

### Valid DID Documents

Most DID methods are implemented on a Distributed Ledger Technology (DLT), such as Bitcoin, Ethereum or IOTA. Most common DID implementation on DLTs are based on fit-for-purpose Blockchains that store the state of a DID Document in the ledger, or a general purpose Blockchain that utilize smart contracts to store the state. Updating a DID Document where the state is understood by the network is straightforward. The network can determine if an action is legal and if a cryptographic signature is provided correctly, as it understands the underlying data structure, and can update the state accordingly. The individual state updates, or transactions, can be forgotten.

The IOTA Tangle is unable to track state or the understand the data structure. Storing the state is neither possible in the ledger, nor via a Smart contract (yet). Therefore, IOTA Identity has to recreate and validate the state from the origin of the Identity to the current version. The process involves querying all the relevant transactions from the Tangle, ordering them, filtering out the transactions that perform illegal actions or have an incorrect signature and then recreate state. As this requires the full history of the Identity, we recommend utilizing [Chronicle](https://github.com/iotaledger/chronicle.rs), an IOTA permanode, which stores the entire history of the Tangle. Further research will be performed to reduce storage requirements for IOTA Identity based applications.

### DID Messages

Due to this constant need for state recreating, unique performance improvements have been design and implemented for IOTA Identity. Most DID Documents will need few to no updates, however identities that sign a lot of Verifiable Credentials might update more frequently, as will be explained in the Verifiable Credentials section. To support higher frequency identity updates, we have introduced a unique solution called the “Integration Chain” and the “Differentiation Chain” (Diff Chain).

The Integration Chain is a chain of transactions that contain full DID Documents. They are unrestricted in what they can add or remove from the DID Document. Every Integration Chain transaction points to a separate new Diff Chain. These Diff Chain transactions only list the changes to a DID Document and are therefore more compact. It is, however, restricted in rotating the signing key, making it fast and easy to validate the transaction.

Once a new Integration chain transaction is created, it will take all Diff Chain updates and compress them into a new DID Document, essentially combining them all into a single transaction. This reduces the amount of updates that need to be queried and validated tremendously. For example, lets assume every Diff chain contains 100 updates. Then validating a DID that has done 1050 updates, only requires the validation of 10 Integration Chain updates and 40 Diff Chain updates (The latest Diff Chain). We skipped out on 10 Diff Chains each containing 100 updates, and only validated the 10 Integration Chain updates and the last Diff Chain containing 40 updates. If we estimate every update to be on average 1 Kb, we only have to download 50 kb of information and validate it, which is signficantly less then the otherwise 1.025 Mb of information.

The improved performance and ability to handle frequently updated DID Documents is especially beneficial for Verifiable Credential Revocation, by utilizing revocation flags. These concepts will be explained in the Verifiable Credentials and Key Collections sections.


Example of Utilizing a Diff Chain
```
TODO
```

