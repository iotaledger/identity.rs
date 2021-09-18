---
title: Resolve an IOTA Identity
sidebar_label: Resolve
description: Explain how resolving works including arguments.
image: /img/Identity_icon.png
keywords:
- Resolve
- DID
---

In the previous section we created a decentralized identity and published the associated DID Document to the Tangle; in this section we'll show you how to look up a DID document.

:::info Definition

_DID resolution_ is the process of obtaining a DID document for a given DID.

:::

### Resolving an Identity using the Account

Continuing from the previous example, we'll use the `did` associated with the identity we created to retrieve the published DID document.

```rust
// Retrieve the DID from the newly created Identity state.
let did: &IotaDID = snapshot.identity().try_did()?;

// Fetch the DID Document from the Tangle.
let resolved_document: IotaDocument = account.resolve_identity(did).await?;

println!("[Example] Tangle Document = {:#?}", resolved_document);
```

In simplified terms, resolution works by querying the Tangle for DID Document messages and returning the most recent valid message that matches the specified DID. In practice, the operation is more complex and is explained in full in the [IOTA DID Method specification](../../specs/iota_did_method_spec#read).

:::important

Normal nodes in the IOTA network do not store a full history of the Tangle, which means that at some point DID messages will be dropped. The solution is to use an IOTA permanode ([Chronicle](https://github.com/iotaledger/chronicle.rs)) which stores the entire history of the Tangle. For more information see the section on [Valid DID Documents](../advanced/did_messages#valid-did-documents).

:::

### Dereferencing resources associated with a DID Document

In addition resolving a DID document, another "read" operation that can be performed is dereferencing.

:::info Definitions

 _DID URL dereferencing_ is the process of retrieving a representation of a resource for a given DID URL.

:::

Dereferencing takes a 'DID URL' and 'Dereference Options' returns the 'Dereference Metadata', a 'Content Stream', and 'Content Metadata'. For example, one could retrieve a file associated with a particular identity: `did:example:1234?service=files&relativeRef=%2Fmyresume%2Fdoc%3Fversion%3Dlatest`