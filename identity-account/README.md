# Iota Identity Account

The Account is an interface for creating and managing identities on the Iota Tangle. It includes easy-to-use functions that allow many operations including:

- Creating an identity or resolving an existing DID Document.
- Updating a DID Document by adding or removing:
  - Services.
  - Verification Methods.
  - Verification Relationships.
  - Custom properties.
- Managing and storing the state of DID Documents.
- Managing and storing private encryption keys used as verification methods.

## Account Creation

Creating an account is done through the `AccountBuilder`.

```rust
  let account: Account = Account::builder()
    .create_identity(IdentitySetup::default())
    .await?;
```

Some options can be set for the `AccountBuilder` including:

- `autosave`: sets the account auto-save behaviour.
  - `AutoSave::Every` => save to storage on every update
  - `AutoSave::Never` => never save to storage when updating
  - `AutoSave::Batch` => save to storage after every `n` updates.
- `autopublish`: sets the account auto-publish behaviour.
  - `true` => publish to the Tangle on every DID document change
  - `false` => never publish automatically
- `storage`: sets the account storage adapter.
- `client`: sets the IOTA Tangle `Client`, this determines the `Network` used by the identity.
- `create_identity`: creates a new identity based on the builder configuration and returns an `Account` instance to manage it.
- `load_identity`: loads an existing identity with the specified `did` using the current builder configuration. The identity must exist in the configured `Storage`.

## Operations

Operations can be done by calling their specific funciton on the account, for instance:

  ```rust
  account
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateEd25519)
    .fragment("my-next-key")
    .apply()
    .await?;
```

is used to add a verification method to the DID Document.

Similar functions are exposed for different operations including:

- `delete_method`: deletes a verification method.
- `attach_method_relationship`: attaches one or more verification relationships to a method.
- `detach_method_relationship` detaches one or more verification relationships to a method.
- `create_service`: adds a new Service to the DID Document.
- `delete_service`: deletes a Service from the DID Document.
- `set_controller`: sets the controllers of the DID document.
- `set_also_known_as`: sets the `alsoKnownAs` property in the DID document.
- `fetch_document`: fetches the latest changes from the tangle and **overwrites** the local document.
- `encrypt_data`: encrypts the given `plaintext` with the specified `encryption_algorithm` and `cek_algorithm`.
- `decrypt_data`: decrypts the given `data` with the key identified by `fragment` using the given `encryption_algorithm` and `cek_algorithm`.
- `resolve_identity`: resolves the DID Document associated with this `Account` from the Tangle.
- `delete_identity`: removes the identity from the local storage entirely.
- `sign`: signs `data` with the key specified by `fragment`.
- `publish`: pushes all unpublished changes to the tangle in a single message.
- `update_document_unchecked`: overwrites the `IotaDocument` this account manages, **without doing any validation**.
