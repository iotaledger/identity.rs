import {
  KeyPair,
  KeyType,
  MethodScope,
  IotaDID,
  IotaDocument,
  IotaIdentityClient,
  IotaVerificationMethod
} from '../../node';
import type { Client, SecretManager } from "@iota/iota-client-wasm/node";
import { AddressTypes, Bech32Helper, IAliasOutput } from '@iota/iota.js';

export const API_ENDPOINT = "https://api.testnet.shimmer.network/";
export const FAUCET_ENDPOINT = "https://faucet.testnet.shimmer.network/api/enqueue";

export async function createDid(client: Client, secretManager: SecretManager): Promise<{
  address: AddressTypes,
  did: IotaDID
}> {
  const didClient = new IotaIdentityClient(client);
  const networkHrp: string = await didClient.getNetworkHrp();

  const walletAddressBech32 = (await client.generateAddresses(secretManager, {
    accountIndex: 0,
    range: {
      start: 0,
      end: 1,
    },
  }))[0];
  console.log("Wallet address Bech32:", walletAddressBech32);

  await ensureAddressHasFunds(client, walletAddressBech32);

  const address = Bech32Helper.addressFromBech32(walletAddressBech32, networkHrp);

  // Create a new DID document with a placeholder DID.
  // The DID will be derived from the Alias Id of the Alias Output after publishing.
  const document = new IotaDocument(networkHrp);

  // Insert a new Ed25519 verification method in the DID document.
  let keypair = new KeyPair(KeyType.Ed25519);
  let method = new IotaVerificationMethod(document.id(), keypair.type(), keypair.public(), "#key-1");
  document.insertMethod(method, MethodScope.VerificationMethod());

  // Construct an Alias Output containing the DID document, with the wallet address
  // set as both the state controller and governor.
  const aliasOutput: IAliasOutput = await didClient.newDidOutput(address, document);

  // Publish the Alias Output and get the published DID document.
  const published = await didClient.publishDidOutput(secretManager, aliasOutput);

  return { address, did: published.id() };
}

/** Request funds from the testnet faucet API, if needed, and wait for them to show in the wallet. */
export async function ensureAddressHasFunds(client: Client, addressBech32: string) {
  let balance = await getAddressBalance(client, addressBech32);
  if (balance > 0) {
    return;
  }

  await requestFundsFromFaucet(addressBech32);

  for (let i = 0; i < 9; i++) {
    // Wait for the funds to reflect.
    await new Promise(f => setTimeout(f, 5000));

    let balance = await getAddressBalance(client, addressBech32);
    if (balance > 0) {
      break;
    }
  }
}

/** Returns the balance of the given Bech32-encoded address. */
async function getAddressBalance(client: Client, addressBech32: string): Promise<number> {
  // TODO: use the `addresses/ed25519/<addressHex>` API to get the balance?
  const outputIds = await client.basicOutputIds([
    { address: addressBech32 },
    { hasExpiration: false },
    { hasTimelock: false },
    { hasStorageDepositReturn: false }
  ]);
  const outputs = await client.getOutputs(outputIds);

  let totalAmount = 0;
  for (const output of outputs) {
    totalAmount += Number(output.output.amount);
  }

  return totalAmount;
}

/** Request tokens from the testnet faucet API. */
async function requestFundsFromFaucet(addressBech32: string) {
  const requestObj = JSON.stringify({ address: addressBech32 });
  let errorMessage, data;
  try {
    const response = await fetch(FAUCET_ENDPOINT, {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: requestObj,
    });
    if (response.status === 202) {
      errorMessage = "OK";
    } else if (response.status === 429) {
      errorMessage = "too many requests, please try again later.";
    } else {
      data = await response.json();
      // @ts-ignore
      errorMessage = data.error.message;
    }
  } catch (error) {
    errorMessage = error;
  }

  if (errorMessage != "OK") {
    throw new Error(`failed to get funds from faucet: ${errorMessage}`);
  }
}
