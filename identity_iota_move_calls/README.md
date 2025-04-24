# Platform Agnostic Interfaces for Identity Move Call

This crate gathers interfaces that provide platform specific implementations to
interact with the identity Move package.

Platform specific adapters (implementing the cross-platform-traits defined in this crate) are contained in
the crates [bindings/wasm/iota_move_calls](../bindings/wasm/iota_move_calls_ts)
and [iota_move_calls_rust](../identity_iota_core/src/iota_move_calls_rust).