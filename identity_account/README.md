# IOTA Identity - Account

The [`Account`](crate::account::Account) is an interface for creating and managing identities on the IOTA Tangle, handling publishing and secure storage automatically. It provides convenience functions for:

- Creating and publishing a new IOTA DID.
- Updating DID Document contents:
  - Verification Methods.
  - Verification Relationships.
  - Services.
- Managing private cryptographic keys securely.
- Signing credentials.
- Encrypting messages.

## Account Creation

Creating an [`Account`](crate::account::Account) is done through the [`AccountBuilder`](crate::account::AccountBuilder).

```rust,ignore
let account: Account = Account::builder()
  .create_identity(IdentitySetup::default())
  .await?;
```


## Update Operations

Updating a DID Document can be performed through the [`update_identity`](crate::account::Account::update_identity) function on the [`Account`](crate::account::Account). For example, adding a new verification method to the DID Document:

```rust,ignore
account
  .update_identity()
  .create_method()
  .content(MethodContent::GenerateEd25519)
  .fragment("my-next-key")
  .apply()
  .await?;
```

The above code generates a new Ed25519 keypair, writes it to [`Storage`](identity_account_storage::storage::Storage), embeds it in a new verification method, and publishes the updated DID Document to the Tangle. 

See the [`IdentityUpdater`](crate::types::IdentityUpdater) for a list of provided update operations.

