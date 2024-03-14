# Identity.rs gRPC Bindings
This project provides the functionalities of [Identity.rs](https://github.com/iotaledger/identity.rs) in a language-agnostic way through a [gRPC](https://grpc.io) server.

The server can easily be run with docker using [this dockerfile](https://github.com/iotaledger/identity.rs/blob/grpc-bindings/bindings/grpc/Dockerfile).

## Build
Run `docker build -f bindings/grpc/Dockerfile -t iotaleger/identity-grpc .` from the project root.

### Dockerimage env variables and volume binds
The provided docker image requires the following variables to be set in order to properly work:
- `API_ENDPOINT`: IOTA's node address.
- `STRONGHOLD_PWD`: Stronghold password.
- `SNAPSHOT_PATH`: Stronghold's snapshot location.

Make sure to provide a valid stronghold snapshot at the provided `SNAPSHOT_PATH` prefilled with all the needed key material.

### Available services
| Service description            | Service Id                               | Proto File |
|--------------------------------|------------------------------------------|------------|
| Credential Revocation Checking | `credentials/CredentialRevocation.check` | [credentials.proto](https://github.com/iotaledger/identity.rs/blob/grpc-bindings/bindings/grpc/proto/credentials.proto) |
| SD-JWT Validation              | `sd_jwt/Verification.verify` | [sd_jwt.proto](https://github.com/iotaledger/identity.rs/blob/grpc-bindings/bindings/grpc/proto/sd_jwt.proto) |
| Credential JWT creation | `credentials/Jwt.create` | [credentials.proto](https://github.com/iotaledger/identity.rs/blob/grpc-bindings/bindings/grpc/proto/credentials.proto) |
