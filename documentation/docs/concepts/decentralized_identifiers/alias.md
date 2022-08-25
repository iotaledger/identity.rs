---
title: Alias Output 
sidebar_label: Alias Output
description: UTXO Alias Ouput
image: /img/Identity_icon.png
keywords:
- public keys
- utxo
- Method Specification
- Decentralized Identifiers
- overview
- DLT
---

# Alias Output

The IOTA method uses the IOTA ledger which uses the [unspent transaction output (UTXO) model](https://wiki.iota.org/goshimmer/protocol_specification/components/ledgerstate). Also, the features of the [Stardust](https://blog.shimmer.network/stardust-upgrade-in-a-nutshell/) upgrade are fundamental to the IOTA DID method.

The Alias Output is used for storing the DID Document on the ledger. It is a specific implementation of the UTXO state machine that can hold arbitrary data in its `State Metadata`. The Alias Output has two kinds of controllers, a state controller and a governor. A state controller can execute a state transition which allows updating the data in the `State Metadata`. The governor, on the contrary, can't update the `State Metadata` but can change both controllers and destroy the Alias Output.
A controller can be either Ed25519 Address, Alias Address or an NFT Address and at most one of each can be set for an Alias Output.

In order to create a new Alias Output, a transaction must be made that includes another Output, for example a Basic Output, as input and the new Alias Output, along with other outputs if needed, as outputs.

### Storage Deposit

The arbitrary data stored in the `State Metadata` of the Alias output must be covered by a storage deposit using IOTA coins. This helps to control the ledger size from growing uncontrollably while guaranteeing the data is indefinitely stored on the ledger which is important for resolving DID Documents. This deposit is fully refundable and can be reclaimed when the output is destroyed. Both, the state controller and the governor can control the IOTA coins stored in the Alias Output. Nodes expose an API to calculate the required deposit depending on the size of the data stored. 

### Alias Id

Each Alias Output has an `Alias ID`. This ID is assigned after a transaction creates a new Alias Output. The actual DID is derived from this `Alias ID`, hence it is be unknown before publishing the transaction. Consequently, the DID inside the `State Metadata` will be replaced by the placeholder `did:0:0` to indicate self. 

If a transaction has an Alias Output as input, its `Alias ID` can be kept by one of its outputs. This feature is necessary for updating the DID Documents since the DID itself is derived from the Alias Output.






