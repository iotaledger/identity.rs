---
title:  Rust Cheat Sheet
sidebar_label: Cheat Sheet
description: IOTA Identity Rust Library Cheat Sheet
image: /img/Identity_icon.png
keywords:
- rust
- identity
- decentralized identifiers
- did
- verifiable credentials
- verifiable presentations
- create
- update
- resolve
- remove
- reference
---

## Import the Library

To include IOTA Identity in your project, add it as a dependency in your Cargo.toml.

### Latest Stable Release

This version matches the `main` branch of this repository. It is **stable** and will have **changelogs**.

```rust
[dependencies]
identity = { git = "https://github.com/iotaledger/identity.rs", branch = "main"}
```

### Development Release

This version matches the `dev` branch of this repository. It has all the **latest features**, but it **may also have undocumented breaking changes**.

```rust
[dependencies]
identity = { git = "https://github.com/iotaledger/identity.rs", branch = "dev"}
```

## Decentralized Identifiers (DID)

A DID is a unique identifier that contains information that can be resolved to a DID Document. This document contains data such as public keys, enabling the holder to prove ownership over their personal data, but also URIs that link to public information about the identity. This implementation complies with the DID specifications v1.0 Working.

### [Create](../../decentralized_identifiers/create.mdx)

#### Account::builder().build()

Creates a new Account with the default configuration

```rs
let account: Account = Account::builder().build().await?;
```
##### Returns

<details>
<summary>IdentitySnapshot</summary>

```log
Account {
    config: Config {
        autosave: Every,
        autopublish: true,
        dropsave: true,
        testmode: false,
        milestone: 1,
    },
    state: State {
        actions: 1,
        clients: ClientMap {
            data: {
                NetworkName(
                    "main",
                ): Client {
                    client: Client {
                        node_manager: NodeManager {
                            primary_node: None,
                            primary_pow_node: None,
                            nodes: {
                                Node {
                                    url: Url {
                                        scheme: "https",
                                        cannot_be_a_base: false,
                                        username: "",
                                        password: None,
                                        host: Some(
                                            Domain(
                                                "chrysalis-nodes.iota.org",
                                            ),
                                        ),
                                        port: None,
                                        path: "/",
                                        query: None,
                                        fragment: None,
                                    },
                                    jwt: None,
                                },
                            },
                            permanodes: None,
                            sync: true,
                            sync_interval: 60s,
                            synced_nodes: RwLock {
                                data: {
                                    Node {
                                        url: Url {
                                            scheme: "https",
                                            cannot_be_a_base: false,
                                            username: "",
                                            password: None,
                                            host: Some(
                                                Domain(
                                                    "chrysalis-nodes.iota.org",
                                                ),
                                            ),
                                            port: None,
                                            path: "/",
                                            query: None,
                                            fragment: None,
                                        },
                                        jwt: None,
                                    },
                                },
                                poisoned: false,
                                ..
                            },
                            quorum: false,
                            quorum_size: 3,
                            quorum_threshold: 66,
                        },
                        network_info: RwLock {
                            data: NetworkInfo {
                                network: Some(
                                    "main",
                                ),
                                network_id: Some(
                                    1454675179895816119,
                                ),
                                bech32_hrp: "iota",
                                min_pow_score: 4000.0,
                                local_pow: false,
                                tips_interval: 15,
                            },
                            poisoned: false,
                            ..
                        },
                    },
                    network: Mainnet,
                },
            },
        },
    },
    store: MemStore,
    index: RwLock {
        mr: 536870911,
        s: Semaphore {
            permits: 536870911,
        },
        c: UnsafeCell { .. },
    },
}
```

</details>


####  Account.create_identity(IdentityCreate::default())

Create a new Identity with default settings.

```rs
let snapshot: IdentitySnapshot = account.create_identity(IdentityCreate::default()).await?;
```

#### Returns

<details>
<summary>IdentitySnapshot</summary>

```log
 IdentitySnapshot {
    sequence: Generation(4),
    identity: IdentityState {
        id: IdentityId(0x00000001),
        integration_generation: Generation(1),
        diff_generation: Generation(0),
        this_message_id: MessageId(1b802018b0fcf2acbf292fd231e1407cd1db21509ee17aa71e7ef5bf564c6c51),
        last_integration_message_id: MessageId(0000000000000000000000000000000000000000000000000000000000000000),
        last_diff_message_id: MessageId(0000000000000000000000000000000000000000000000000000000000000000),
        did: Some(
            did:iota:2Gihsa2TXGCAhfHLfS4qtUtW13h4ayKeT5C58KtUcj9s,
        ),
        controller: None,
        also_known_as: None,
        methods: Methods {
            data: {
                Authentication: [
                    Refer(
                        Fragment(_sign-0),
                    ),
                ],
                VerificationMethod: [
                    Embed(
                        TinyMethod {
                            location: KeyLocation(0:0:_sign-0:0),
                            key_data: PublicKeyBase58(FVTfZXkbTtRcnBUGaTvYDbJSJZ9QFQp9CBRM9fvJQhX5),
                            properties: None,
                        },
                    ),
                ],
            },
        },
        services: Services {
            data: [],
        },
        created: UnixTimestamp(1634124718),
        updated: UnixTimestamp(1634124718),
    },
}
```

</details>

### [Publish](../../decentralized_identifiers/create.mdx)

#### Account.publish_updates(did)

Publish a DID document.

```rs
Account.publish_updates(did).await?;
```

#### Returns

<details>
<summary>Result</summary>

```log
 {
    Ok(T),
    Err(E),
}
```

</details>

### [Update](../../decentralized_identifiers/update.mdx)

#### Account.update_identity(did).create_method()

Add a new Ed25519 (default) verification method to the identity - the verification method is included as an embedded authentication method.

```rs
account
    .update_identity(did)
    .create_method()
    .scope(MethodScope::Authentication)
    .fragment("my-auth-key")
    .apply()
    .await?;
```

##### Returns

<details>
<summary>Result</summary>

```log
 {
    Ok(T),
    Err(E),
}
```

</details>

#### Account.update_identity(did).create_service()

Add a new service to the identity.

```rs
 account
    .update_identity(did)
    .create_service()
    .fragment("my-service-1")
    .type_("MyCustomService")
    .endpoint(Url::parse("https://example.com")?)
    .apply()
    .await?;    
```

##### Returns

<details>
<summary>Result</summary>

```log
 {
    Ok(T),
    Err(E),
 }
```
</details>


### [Resolve](../../decentralized_identifiers/resolve.mdx)

####  Account.resolve_identity(did)

Resolves a DID into a DID Document by using the “Read” operation of the DID method.

```rs
account.resolve_identity(did).await?;
```

##### Returns

<details>
<summary>
 IotaDocument
</summary>

```log
CoreDocument {
    id: "did:iota:DQE89CN6GTiF2bkqzEBtBDHpZgGyYZ5SK4kymJ4PiAXW",
    controller: None,
    also_known_as: [],
    verification_method: {
        VerificationMethod {
            id: "did:iota:DQE89CN6GTiF2bkqzEBtBDHpZgGyYZ5SK4kymJ4PiAXW#_sign-0",
            controller: "did:iota:DQE89CN6GTiF2bkqzEBtBDHpZgGyYZ5SK4kymJ4PiAXW",
            key_type: Ed25519VerificationKey2018,
            key_data: PublicKeyBase58(3hmPzqVWZDiXyBtgnEaxL2uS8mKoDPnw9V4YkmxoKSPE),
            properties: {},
        },
    },
    authentication: {
        "did:iota:DQE89CN6GTiF2bkqzEBtBDHpZgGyYZ5SK4kymJ4PiAXW#_sign-0",
    },
    assertion_method: {},
    key_agreement: {},
    capability_delegation: {},
    capability_invocation: {},
    service: {
        Service {
            id: "did:iota:DQE89CN6GTiF2bkqzEBtBDHpZgGyYZ5SK4kymJ4PiAXW#my-service-1",
            type_: "MyCustomService",
            service_endpoint: Url(https://example.com/),
            properties: {},
        },
    },
    properties: Properties {
        properties: Properties {
            created: "2021-10-19T12:47:26Z",
            updated: "2021-10-19T12:47:44Z",
            previous_message_id: MessageId(0000000000000000000000000000000000000000000000000000000000000000),
            properties: {},
        },
        proof: Some(
            Signature {
                type_: "JcsEd25519Signature2020",
                value: Signature(2ujinNZYAd5HYkrSwRe5EZ1b7x9ZFJsZMCowzNTho8naqtt8J9bhbZPFs4pn33SFU64kdKnfAKa12k3p2VVzzjp6),
                method: "did:iota:DQE89CN6GTiF2bkqzEBtBDHpZgGyYZ5SK4kymJ4PiAXW#_sign-0",
            },
        ),
    },
}
```

</details>

## Verifiable Credentials (VC)

A Verifiable Credential can be verified by anyone, allowing you to take control of it and share it with anyone.

### [Sign](../../verifiable_credentials/create.mdx)

#### Account.sign(did, key, fragment)

Sign the Credential with a previously created Verification Method.

```rs
account.sign(did, "key-1", &mut credential).await?;
```

#### Returns

<details>
<summary>Result</summary>

```log
 {
    Ok(T),
    Err(E),
 }
```
</details>

## Verifiable Presentations (VP)

A Verifiable Presentation is the format that you can share a (collection of) Verifiable Credential(s). It is signed by the subject to prove control over the Verifiable Credential with a nonce or timestamp.

### [Create](https://wiki.iota.org/identity.rs/verifiable_credentials/verifiable_presentations)
