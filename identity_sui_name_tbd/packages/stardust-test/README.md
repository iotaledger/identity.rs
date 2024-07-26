# stardust test package

This package is a copy of the in development [stardust package](https://github.com/iotaledger/kinesis/tree/develop/crates/iota-framework/packages/stardust) from [a commit]([7899dc9ce682c3d0a97f249ce7eaa27b9473b920](https://github.com/iotaledger/kinesis/commit/7899dc9ce682c3d0a97f249ce7eaa27b9473b920)) from Wed May 1 12:52:25 2024 +0000.

The changes introduced in the local copy are for testing purposes only. Depending on how we will be able to access test data, we might be able to drop this folder in the future.

The local changes are:

- remove `#[test_only]` from `create_for_testing` in `./sources/alias/alias.move` to be able to create test data
- update dependencies `MoveStdlib` and `Iota` to use kinesis project version (same commit hash as mentioned above)