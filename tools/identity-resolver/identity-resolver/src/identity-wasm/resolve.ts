import * as identity from "@iota/identity-wasm/web";

export async function resolveIdentity(inputDid: string) {


  console.log("resolving identity");
  
  await identity.init("/identity_wasm_bg.wasm");

  console.log('init success');
  
  const mainNet = identity.Network.mainnet();

  console.log(mainNet);
  
  const CLIENT_CONFIG = {
    network: mainNet,
    defaultNodeURL: mainNet.defaultNodeURL,
    explorerURL: mainNet.explorerURL,
  };

  // Create a default client configuration from network.
  const config = identity.Config.fromNetwork(mainNet);

  // Create a client instance to publish messages to the Tangle.
  const client = identity.Client.fromConfig(config);

  const res = await client.resolve('did:iota:' + inputDid);
  return res;
}
