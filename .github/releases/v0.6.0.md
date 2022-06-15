The main feature of this release is the addition of the `RevocationBitmap2022` specification, offering efficient credential revocation on-Tangle. This is the replacement for the `MerkleKeyCollection` removed in v0.5.0, which offered similar functionality but fundamentally failed to scale beyond a few thousand revocations.
\n\n
Other changes include encryption support using Elliptic Curve Diffie-Hellman (ECDH) and quality of life improvements for verifiable credential and presentation types in the Wasm bindings.
\n\n
DID Documents created with v0.5.0 remain compatible with v0.6.0. This will be the last major release prior to changes for the Stardust update. 
\n\n