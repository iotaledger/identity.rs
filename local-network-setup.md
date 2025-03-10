# Local Network Setup

## Start the local chain

The examples in the repository and in this getting started section require a test network running locally, so if you do not already have, [install the IOTA CLI tool](https://docs.iota.org/developer/getting-started/install-iota).

Now start the local network, e.g. with the following command (more details [here](https://docs.iota.org/developer/getting-started/local-network)).

```bash
RUST_LOG="off,iota_node=info" iota start --force-regenesis --with-faucet
```

## Configure IOTA client for local chain

### Fresh IOTA client

If you haven't started the IOTA CLI client features tool yet, you have to generate a config file during first start, e.g. by calling

```bash
iota client
```

Agree to connect to a full node server. Now you can interactively provide the config values:

- URL: "http://127.0.0.1:9000"
- alias: "localnet"
- key scheme: 0 (ed25519)

### New env

Or if you don't already have an environment pointing to the local node, create a new env with for an IOTA Full node server with the same values as described above.

```bash
iota client new-env --rpc "http://127.0.0.1:9000" --alias localnet
```

### Switch to local node env

If you did one of the steps above or if you already have an env as described above, make sure to switch to it.

Check active env with:

```bash
iota client env
```

If your localnet does not have the asterisk in the "active" column, switch to it with:

```bash
iota client switch --env localnet
```

### Request funds for active accounts

Request funds for the active account, as the next step will need them. This can be done with:

```bash
iota client faucet
```

### Publish IOTA Identity Package

Now that you have your env pointing to a local node, and have an account, you're almost ready to put a checkmark on the last requirement for using the IOTA Identity: The deployment of the smart contracts used by the library.

If you haven't already, clone the IOTA identity [repository](https://github.com/iotaledger/identity.rs), and switch to the tag matching the version, you're working with.

Publish the test identity package to your local network:

```bash
./identity_iota_core/scripts/publish_identity_package.sh
```

The last line of the output will be the id of the package , that was just published. You'll need this id when creating an instance of `IdentityClient` and `IdentityClientReadOnly` for a custom network, like the local network.

The examples fetch this value from the environment variable `IOTA_IDENTITY_PKG_ID`, so in order to run them, set this variable, e.g. for package id "0x20f640b0dc01c50c04a84443d4320d7a77a15ed94f9bc19ebb6d0a5805045ddc" with:

```bash
export IOTA_IDENTITY_PKG_ID=0x20f640b0dc01c50c04a84443d4320d7a77a15ed94f9bc19ebb6d0a5805045ddc
```
