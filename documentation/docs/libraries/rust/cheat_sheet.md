---
title:  Rust Cheat Sheet
sidebar_label: Cheat Sheet
description: IOTA Identity Wasm Library Cheat Sheet
image: /img/Identity_icon.png
keywords:
- wasm
- identity
- decentralized identifiers
- did
- verifiable credentials
- verifiable presentations
- create
- update
- resolve
- remove
---

## Decentralized Identifiers (DID)

### Create

```rust
// Create a new Account with the default configuration

(type_, network, tag)

// Create a new Identity with default settings
let snapshot: IdentitySnapshot = account.create_identity(IdentityCreate::default()).await?;
```

### Update

### Resolve

### Remove

## Verifiable Credentials (VC)

### Create

### Update

### Resolve

### Remove

### Verifiable Presentations (VP)

### Create

### Update

### Resolve

### Remove