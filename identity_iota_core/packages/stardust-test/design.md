# Stardust on Move

## Migrating Basic Outputs

Every Basic Output has an `Address Unlock` and an `IOTA` balance (u64). Depending on what other fields we have, we will create different objects.

[Decision graph on what to do with a basic output during migration](./basic_migration_graph.svg)

![](./basic_migration_graph.svg)

## User Flow

Majority of user funds are sitting in Basic Outputs without unlock conditions. Such tokens will be migrated to `0x2::coin::Coin<IOTA>` which one can directly use as a gas payment object.
If a user does not end up with such coin objects at migration, we will have to sponsor their transaction to extract assets.

- We can directly ask back the gas fee from the migrated object
- Take a look at the `test` function inside [`stardust::basic_output`](./sources/basic/basic_output.move) on how to construct a PTB for a user to claim all assets and fuflill unlock conditions.

## Alias Object

- Alias ID must be kept between the migration from Stardust to Move (for applications like Identity). During the migration, any Alias Output with a zeroed ID must have its corresponding computed Alias ID set.
- The Foundry Counter in Stardust is used to give foundries a unique ID. The Foundry ID is the concatenation of `Address || Serial Number || Token Scheme Type`. In Move the foundries are represented by unique packages that define the corresponding Coin Type (a one time witness) of the Native Token. Because the foundry counter can no longer be enforced to be incremented when a new package is deployed, which defines a native token and is owned by that Alias, the Foundry Counter becomes meaningless. Hence, we should remove it. The same count can be determined (off-chain) by counting the number of `TreasuryCap`s the Alias owns.
- State Controller is represented as a `StateCap` that can be updated by the governor. This happens by increasing the `StateCap` version.
- The Governor Capability contains the ID of the alias which it controls. This is needed such that not _any_ `GovernorCap` can be used to update _any_ Alias.
- No way for user to create a new Alias in Move by not providing a constructor. These are only created during the migration.
- We would most likely want to receive Alias, Basic and NFT Output objects or Treasury Caps objects and return them in a receiving function (`unlock_alias_address_owned_output`), so we do not have to store them in the Alias itself. Then they can be transferred somewhere else in the calling PTB.
