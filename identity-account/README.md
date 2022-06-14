# IOTA Identity - Account

The [`Account`](crate::account::Account) is an interface for creating and managing identities on the IOTA Tangle, handling publishing and secure storage automatically. It provides convenience functions such as:

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

```{.ignore .rust}
  let account: Account = Account::builder()
    .create_identity(IdentitySetup::default())
    .await?;
```


## Operations

Operations can be done by calling their specific function on the account, for instance:

```{.ignore .rust}
  account
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateEd25519)
    .fragment("my-next-key")
    .apply()
    .await?;
```

is used to add a verification method to the DID Document.

