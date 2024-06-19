
const { IotaDID,
  IotaIdentityClient } = require("@iota/identity-wasm/node");

const { Client } = require("@iota/sdk-wasm/node");

(async () => {
  const did = "did:iota:rms:0x803c66ccd334aa4196618f66baecf0c54ab1d3267633fe975aef54f3d4d161ed";

  const client = new Client({
    primaryNode: "https://api.testnet.shimmer.network/",
    localPow: true,
  });
  const didClient = new IotaIdentityClient(client);

  // Resolve the associated Alias Output and extract the DID document from it.
  const didDocument = await didClient.resolveDid(
    IotaDID.fromJSON(did)
  );
  console.debug("Resolved DID document:", JSON.stringify(didDocument, null, 2));

  // We can also resolve the Alias Output directly.
  const aliasOutput = await didClient.resolveDidOutput(
    IotaDID.fromJSON(did)
  );
  console.log(
    "The Alias Output holds " + aliasOutput.getAmount() + " tokens"
  );
})();