# Iota Client SDK Abstraction

The folder `sdk_types`, contained in this folder, provides a selection of
code copied from the iotaledger/iota.git repository:

| Folder Name                           | Original Source in iotaledger/iota.git |
| ------------------------------------- | -------------------------------------- |
| sdk_types/iota_json_rpc_types         | crates/iota-json-rpc-types |
| sdk_types/iota_types                  | crates/iota-types |
| sdk_types/move_command_line_common    | external-crates/move/crates/move-command-line-common |
| sdk_types/move_core_types             | external-crates/move/crates/move-core-types |
| sdk_types/shared_crypto               | crates/shared-crypto/Cargo.toml |

The folder structure in `sdk_types` matches the way the original IOTA Client Rust SDK
provides the above listed crates via `pub use`.

This module (file 'mod.rs' contained in this folder) provides several
`build target` specific `pub use` and `type` expressions.
For **NON wasm32 targets**, these target specific dependency switches,
provide the original IOTA Client Rust SDK sources.
For **WASM32 targets** the code contained in the `sdk_types` folder is used.

Please make sure always to use `use crate::iota_sdk_abstraction:: ...`
instead of `use iota_sdk:: ...` in you code. This way the dependencies needed for your
code are automatically switched according to the currently used build target.

The Advantage of this target specific dependency switching is,
that for NON wasm32 targets no type marshalling is needed because
the original Rust SDK types are used.

The drawback of target specific dependency switching is, that code of
the original Rust SDK can be used in your code that is not contained in the
`sdk_types` folder. The following todos result from this drawback:

TODOs:
* Allways build your code additionally for the wasm32-unknown-unknown target
  before committing your code:<br>
  `cargo build --package identity_sui_name_tbd --lib --target wasm32-unknown-unknown` 
* We need to add tests for the wasm32-unknown-unknown target in the CI toolchain
  to make sure the code is always buildable for wasm32 targets. 
  
All cross platform usable types and traits (cross platform traits)
are contained in this folder.
Platform specific adapters (implementing the cross platform traits) are contained in 
the folder 
[identity_sui_name_tbd/src/sui/iota_sdk_adapter](../sui/iota_sdk_adapter).