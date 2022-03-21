# Changelog

## [wasm-v0.5.0-dev.5](https://github.com/iotaledger/identity.rs/tree/wasm-v0.5.0-dev.5) (2022-03-21)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.5.0-dev.4...wasm-v0.5.0-dev.5)
 
This release introduces a breaking change to the proof field of DID Documents created by versions `v0.5.0-dev.1` through `v0.5.0-dev.4`, making all prior documents incompatible. The main feature of this release is the introduction of WebAssembly (Wasm) bindings for the high-level `Account` API for Javascript/Typescript in both Node.js and the browser. This includes Stronghold storage support but only for Node.js, as it was determined that compiling Stronghold to Wasm for private key storage in the browser would not be sufficiently secure.

### Changed

- Move DID Document proof outside metadata [\#728](https://github.com/iotaledger/identity.rs/pull/728)
- Replace Wasm getters and setters with methods [\#706](https://github.com/iotaledger/identity.rs/pull/706)
- Replace Wasm `Config` with `ClientConfig` interface [\#696](https://github.com/iotaledger/identity.rs/pull/696)
- Change `IotaDocument::verify_document` from a static function to a method [\#675](https://github.com/iotaledger/identity.rs/pull/675)
- Make Wasm support dependent on `target_arch` rather than feature [\#666](https://github.com/iotaledger/identity.rs/pull/666)
- Refactor `CoreDocument`, `VerificationMethod`, `Service` to use generic DID [\#655](https://github.com/iotaledger/identity.rs/pull/655)
- Overhaul `CredentialValidator`, add `PresentationValidator` [\#599](https://github.com/iotaledger/identity.rs/pull/599)
- Replace `ClientMap` with new `Resolver` [\#594](https://github.com/iotaledger/identity.rs/pull/594)

### Added

- Add deep clone function in Wasm [\#705](https://github.com/iotaledger/identity.rs/pull/705)
- Add `Duration` for `Timestamp` arithmetic [\#684](https://github.com/iotaledger/identity.rs/pull/684)
- Add `Client` fallback to local PoW option [\#682](https://github.com/iotaledger/identity.rs/pull/682)
- Add Wasm `Service` constructor and field getters [\#680](https://github.com/iotaledger/identity.rs/pull/680)
- Complete `Document` Wasm bindings [\#679](https://github.com/iotaledger/identity.rs/pull/679)
- Add `Document.signDocument` for Wasm [\#674](https://github.com/iotaledger/identity.rs/pull/674)
- Add Wasm bindings for `set_controller` and `set_also_known_as` in the `Account` [\#668](https://github.com/iotaledger/identity.rs/pull/668)
- Add NodeJs bindings for Stronghold `Storage` [\#660](https://github.com/iotaledger/identity.rs/pull/660)
- Add Wasm `Account` `Storage` interface [\#597](https://github.com/iotaledger/identity.rs/pull/597)
- Add Wasm bindings for the `Account` [\#574](https://github.com/iotaledger/identity.rs/pull/574)

### Patch

- Enable Wasm weak references for automatic garbage collection [\#694](https://github.com/iotaledger/identity.rs/pull/694)
- Fix `WasmTimestamp` JSON serialization [\#688](https://github.com/iotaledger/identity.rs/pull/688)

### Removed

- Remove `publicKeyJwk` [\#732](https://github.com/iotaledger/identity.rs/pull/732)

## [wasm-v0.5.0-dev.4](https://github.com/iotaledger/identity.rs/tree/wasm-v0.5.0-dev.4) (2022-02-14)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.5.0-dev.3...wasm-v0.5.0-dev.4)

### Changed

- Change `also_known_as` type to `OrderedSet` [\#632](https://github.com/iotaledger/identity.rs/pull/632)

### Patch

- Fix Wasm `DID` conversion error names [\#651](https://github.com/iotaledger/identity.rs/pull/651)
- Support verification methods with the same fragment [\#623](https://github.com/iotaledger/identity.rs/pull/623)

## [wasm-v0.5.0-dev.3](https://github.com/iotaledger/identity.rs/tree/wasm-v0.5.0-dev.3) (2022-01-25)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.5.0-dev.2...wasm-v0.5.0-dev.3)
 
This release introduces a breaking change for diff updates created by versions `v0.5.0-dev.1` and `v0.5.0-dev.2` (previous diff updates from `<=v0.4.0` are already incompatible due to breaking changes to the document and message structure in `v0.5.0-dev.1`). To migrate, please publish an integration update containing all diff changes to prevent unexpected changes to resolved DID Documents.

### Changed

- Add union type parameters [\#616](https://github.com/iotaledger/identity.rs/pull/616)
- Fix dependent diff updates being rejected [\#605](https://github.com/iotaledger/identity.rs/pull/605)
- Remove JSON string escaping in diff messages [\#598](https://github.com/iotaledger/identity.rs/pull/598)

### Patch

- Use node-fetch \>= 2.6.7 [\#617](https://github.com/iotaledger/identity.rs/pull/617)
- Fix diff properties \(de\)serialization [\#611](https://github.com/iotaledger/identity.rs/pull/611)
- Fix incorrect names for `ResolvedDocument.integrationMessageId` & `mergeDiff`  [\#600](https://github.com/iotaledger/identity.rs/pull/600)

## [wasm-v0.5.0-dev.2](https://github.com/iotaledger/identity.rs/tree/wasm-v0.5.0-dev.2) (2022-01-14)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.5.0-dev.1...wasm-v0.5.0-dev.2)

### Changed

- Rename Wasm `VerifiableCredential`, `VerifiablePresentation`  [\#551](https://github.com/iotaledger/identity.rs/pull/551)
- Add signature `created`, `expires`, `challenge`, `domain`, `purpose` [\#548](https://github.com/iotaledger/identity.rs/pull/548)

### Patch

- Fix node-fetch conflict when multiple versions are included [\#587](https://github.com/iotaledger/identity.rs/pull/587)
- Enable local proof-of-work fallback [\#579](https://github.com/iotaledger/identity.rs/pull/579)

## [wasm-v0.5.0-dev.1](https://github.com/iotaledger/identity.rs/tree/wasm-v0.5.0-dev.1) (2021-12-15)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/wasm-v0.4.0...wasm-v0.5.0-dev.1)
 
This release introduces multiple breaking changes to the structure of IOTA DID Documents and their Tangle messages, rendering any identity created with a prior version incompatible and unresolvable. A versioning system has been introduced so any new identities should hopefully be forward compatible with any future breaking changes to the message structure.

### Changed

- Refactor document metadata [\#540](https://github.com/iotaledger/identity.rs/pull/540)
- Replace `chrono` with `time` [\#529](https://github.com/iotaledger/identity.rs/pull/529)
- Rename `DocumentDiff` to `DiffMessage` [\#511](https://github.com/iotaledger/identity.rs/pull/511)
- Deterministic ordering of competing messages [\#506](https://github.com/iotaledger/identity.rs/pull/506)
- Check for existence & duplication of methods in `CoreDocument` [\#504](https://github.com/iotaledger/identity.rs/pull/504)
- Annotate Wasm async function return types [\#501](https://github.com/iotaledger/identity.rs/pull/501)
- Add `ExplorerUrl` to replace `Network` explorer methods [\#496](https://github.com/iotaledger/identity.rs/pull/496)
- Update `ServiceEndpoint` to support sets and maps [\#485](https://github.com/iotaledger/identity.rs/pull/485)
- Add message compression and versioning [\#466](https://github.com/iotaledger/identity.rs/pull/466)
- Update document signing key constraints and methods [\#458](https://github.com/iotaledger/identity.rs/pull/458)

### Added

- Filter out DiffMessages updating signing methods [\#519](https://github.com/iotaledger/identity.rs/pull/519)
- Add publish with retry method [\#455](https://github.com/iotaledger/identity.rs/pull/455)

### Patch

- Fix `Timestamp` in the Wasm bindings [\#541](https://github.com/iotaledger/identity.rs/pull/541)
- Improve client error messages [\#512](https://github.com/iotaledger/identity.rs/pull/512)
- Fix credential validation failing for documents with diff updates [\#490](https://github.com/iotaledger/identity.rs/pull/490)

## [wasm-v0.4.0](https://github.com/iotaledger/identity.rs/tree/wasm-v0.4.0) (2021-11-01)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/360bf5ce64a7f418249cdeadccb22b9aea7daeb6...wasm-v0.4.0)



\* *This Changelog was automatically generated by [github_changelog_generator](https://github.com/github-changelog-generator/github-changelog-generator)*
