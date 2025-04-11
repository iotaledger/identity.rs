# Product agnostic Transaction trait and future Product Client Improvements

This MD file provides some ideas to have a good basis to achieve a common interface design for
IOTA products and to facilitate product composability.

The most important step into this direction (step-1) is to make the `Transaction` trait,
currently used only in `iotaledger/identity.rs` usable for all IOTA products.

A second step which might be desirably probably after the v1.6 Identity release
(step-2) can be to simplify the overall PTB construction and execution process,
when multiple IOTA Products are used together (composability).

## Step-1: Product agnostic `Transaction` trait

To make the `Transaction` trait usable for all IOTA products the `IdentityClient` needs to be
turned into a product agnostic client which can be achieved by using a trait called `CoreClient`:

``` Rust
/// A trait defining the core functionalities required for interacting
/// with IOTA identities, suitable for both read-only and full clients.
#[async_trait(?Send)]
pub trait CoreClient<S>: Sized + Clone {
  /// Returns the underlying IOTA client adapter.
  fn client_adapter(&self) -> &IotaClientAdapter;

  /// Returns the sender address of the client.
  fn sender_address(&self) -> IotaAddress;

  /// Returns the public key of the client.
  fn public_key(&self) -> PublicKey;

  /// Returns a reference to this [`CoreClient`]'s [`Signer`].
  fn signer(&self) -> &S;

  /// Returns `iota_identity`'s package ID.
  fn package_id(&self) -> ObjectID;

  /// Returns the name of the network the client is connected to.
  fn network(&self) -> &NetworkName;

  /// Resolves a _Move_ Object of ID [id](cci:1://file:///Users/yasirdev/Documents/Work/IOTA/identity-rs/identity_iota_core/src/rebased/client/read_only.rs:67:2-72:3) and parses it to a value of type `T`.
  async fn get_object_by_id<T>(&self, id: ObjectID) -> Result<T, Error>
  where
    T: DeserializeOwned;

  /// Returns an object's [`ObjectRef`], if any.
  /// Note: This only provides the basic ObjectRef, not the OwnedObjectRef.
  /// If ownership information is needed, the implementing struct's specific method
  /// should be used, or this trait could be extended.
  async fn get_basic_object_ref_by_id(&self, obj_id: ObjectID) -> Result<Option<ObjectRef>, Error>;
}
```

A modified version of the current `Transaction` trait (branch: `feat/transaction-interface-refactor`)
using the `CoreClient` trait would look like this:

```
pub trait Transaction {
  /// Output type for this transaction.
  type Output;

  /// Encode this operation into a [ProgrammableTransaction].
  async fn build_programmable_transaction<S>(
    &self,
    client: &impl CoreClient<S>,
  ) -> Result<ProgrammableTransaction, Error>;

  /// Parses a transaction result in order to compute its effects.
  async fn apply<S>(
    self,
    tx_results: &IotaTransactionBlockEffects,
    client: &impl CoreClient<S>,
  ) -> Result<Self::Output, Error>;
}
```
An example implementation of these traits can be found in the files
[transaction_builder_idea.rs](./src/rebased/transaction_builder_idea.rs)
and
[rebased/client/mod.rs](./src/rebased/client/mod.rs).

The `TransactionBuilder<Tx>` has also been migrated to the modified `Transaction` trait
in [transaction_builder_idea.rs](./src/rebased/transaction_builder_idea.rs).

## Step 2: Multiple product PTB management for easier product composability

The `ProgrammableTransactionBlockManager` (short name *PtbManager*) described in the glossary below
aims to facilitate the overall PTB construction and execution process,
when multiple IOTA Products are used together.

To explain the functionality of the *PtbManager* the following terminology needs to
be introduced:

### Glossary for ProgrammableTransactionBlockManager Terminology

* `ProductClient`<br>
  Manages product addresses, public_keys, id_documents and other
  product specific data, needed to create `ResourceTxBuilder`s for
  specific `ProductResource`s.
  Examples: `IdentityClient`, `NotarizationClient`
* `ProductResource`<br>
  References one or more existing object(s) on the IOTA ledger.
  Examples: ' AuthenticatedAsset<T>', `DynamicNotarization`, `LockedNotarization`
* `ResourceTxBuilder`<br>
  A transaction builder (current name `TransactionBuilder<Tx>`),
  providing a `Transaction` to perform actions (create, update, destroy, ...)
  on a `ProductResource`.
* `Transaction`<br>
  The currently used `Transaction` trait, but in a `ProductClient` agnostic way.
  which results from Step-1 (see above).
  Provides the transaction needed to perform an action on a `ProductResource` object
  on the ledger (fn `build_programmable_transaction()`) and to evaluate the
  transaction results (fn `apply()`).
* `ProgrammableTransactionBlockManager` (short name *PtbManager*)<br>
  Manages the overall PTB construction and execution process.
  An extended `ProgrammableTransactionBuilder` (from IOTA Rust- or TS-SDK) providing
  additional functionality to facilitate the usage of `ProductResource`s.
* `TransactionHandle`<br>
  A reference to a `Transaction` for a specific action on a specific `ProductResource`.
  Can be used to receive the resulting `ProductResource` of an executed `Transaction`.
  For example, it can be seen as a promise to receive an `AuthenticatedAsset<u64>`
  (a.k.a. `ProductResource`) after a PTB, containing a `Transaction` of type
  `CreateAsset<u64>`, has been executed by a `ProgrammableTransactionBlockManager`.

### Example e2e Test Implementation

An example implementation of the *PtbManager* can be found in the
[asset.rs e2e test file](./tests/e2e/asset.rs).

Some stub code has also been added in [e2e/common.rs](./tests/e2e/common.rs)
to achieve proper type info augmentation in IDE`s.

All tests, except those based on the current ideas, have been commented out to make
the code compilable.

### Some notes on the implementation of `ProgrammableTransactionBlockManager` and `ResourceTxBuilder`

* To implement a `ProgrammableTransactionBlockManager` the existing `TransactionBuilder<Tx>`
  struct needs to be split into `ProgrammableTransactionBlockManager` and `ResourceTxBuilder`.
* The `ProgrammableTransactionBlockManager` owns a platform specific
  `ProgrammableTransactionBuilder` (IOTA Rust- or TS-SDK) instance and provides it to the
  managed `ResourceTxBuilder` instances (see below).
* The `ProgrammableTransactionBlockManager` internally manages an ordered list of
  `ResourceTxBuilder` instances that will be `build()` when
  `ProgrammableTransactionBlockManager::build_and_execute()` is called (see below test example
  code).
    * A `TransactionHandle` is just a reference or list index to the `ResourceTxBuilder` instance
      that finally provides the `Transaction::output`.
    * When `ProgrammableTransactionBlockManager::build_and_execute()` is called
        * A loop over the ordered list of `ResourceTxBuilder` instances, calls
          `ResourceTxBuilder::build_programmable_transaction()` and provides a mutable reference to
          the `ProgrammableTransactionBuilder` to the `ResourceTxBuilder` so that the low level API
          code (a.k.a MoveCalls) can be called to append the needed transactions/commands to the
          final programmable transaction block (PTB) that is build for all `ResourceTxBuilder` instances
          in the ordered list of `ResourceTxBuilder`s.
        * After the final PTB has been build, it is executed by the
          `ProgrammableTransactionBlockManager`
        * The resulting `IotaTransactionBlockEffects` are provided to all `ResourceTxBuilder` instances
          in the ordered list of `ResourceTxBuilder`s. Each `ResourceTxBuilder` needs to find the
          relevant information to update or create the `ResourceTxBuilder::output`.
* IMPORTANT:<br>
  We must make sure, that `ResourceTxBuilder` instances creating new `ProductResource`s on the
  ledger can find the correct ObjectId in the CreatedObjects section of the ObjectChanges
  in the `IotaTransactionBlockEffects`.<br>
  All other kinds of `ResourceTxBuilder` instances (not creating `ProductResource`s) can find
  the correct transaction effect using the ObjectId. As the ObjectId is not known when a new
  `ProductResource` is created we, need to handle these PTBs in a special way
  so that the `ResourceTxBuilder::apply()` function can parse the `IotaTransactionBlockEffects`
  to find the correct ObjectId after a `ProductResource` has been created.<br>
  Possible simple solutions:
    * PTBs creating new `ProductResource`s must only include
        * one single `ResourceTxBuilder`
        * one single `ResourceTxBuilder` per ObjectType
* The `ProgrammableTransactionBlockManager` can do the gas management and PTB signing as
  currently been implemented in `TransactionBuilder<Tx>`
* The function `ProgrammableTransactionBlockManager::append_tx()` needs to be product/resource
  agnostic and shall be usable for all `ProductResource`s of all IOTA Products.<br>
  ```
  // The generic arguments C and T used here are just placeholders to symbolize<br>
  // the currently unknown exact types.<br>
  async fn append_tx(&self, resource_tx_builder: TransactionBuilder<C>)
     -> anyhow::Result<TransactionHandle<T>>
  ```
* To use the IOTA GasStation together with the `ProgrammableTransactionBlockManager`,
  instead of using `ProgrammableTransactionBlockManager::build_and_execute()` the function
  `ProgrammableTransactionBlockManager::build_programmable_transaction()` can be used.
  The `IotaTransactionBlockEffects` resulting from the PTB execution, needs to be provided
  to the `ProgrammableTransactionBlockManager` by using
  `ProgrammableTransactionBlockManager::apply()`:<br>
  <br>
  Function signatures:
  ```
  impl ProgrammableTransactionBlockManager {
    async fn build_programmable_transaction(&self) -> anyhow::Result<ProgrammableTransaction>
    async fn apply(&self, tx_results: &IotaTransactionBlockEffects) -> anyhow::Result<()>
  }
  ```
  Example:
  ```
  // in the context of the test example code below, this would look like:
  let ptb = alice_tx_manager.build_programmable_transaction().await?;
  let tx_effects = ... // Execute the ptb for example using the GasStation
  alice_tx_manager.apply(tx_effects).await?;
  ```