IOTA Identity - SUI tooling
===

## To clarify

### DID resolving part

- [ ] use as handler for actual `identity_resolver`?
- [ ] use builder pattern for initialization?
  - would wrap up optional params (e.g. for network and stardust package ID) more easily
- [ ] maybe move util functions, network into helper / client struct

### Signing of transactions

- [ ] does everyone build their own 