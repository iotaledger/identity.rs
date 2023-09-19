import {
    IJwsVerifier,
    IotaDocument,
    IotaIdentityClient,
    Jwk,
    JwkMemStore,
    JwsAlgorithm,
    MethodScope,
    Storage,
    verifyEdDSA,
} from "@iota/identity-wasm/node";
import {
    type Address,
    AliasOutput,
    type Client,
    IOutputsResponse,
    SecretManager,
    SecretManagerType,
    Utils,
} from "@iota/sdk-wasm/node";

export const API_ENDPOINT = "http://localhost:14265";
export const FAUCET_ENDPOINT = "http://localhost:8091/api/enqueue";

// A JWS Verifier capabale of verifying EdDSA signatures with curve Ed25519.
export class EdDSAJwsVerifier implements IJwsVerifier {
    verify(alg: JwsAlgorithm, signingInput: Uint8Array, decodedSignature: Uint8Array, publicKey: Jwk) {
        switch (alg) {
            case JwsAlgorithm.EdDSA:
                return verifyEdDSA(alg, signingInput, decodedSignature, publicKey);
            default:
                throw new Error(`unsupported jws algorithm ${alg}`);
        }
    }
}

/** Creates a DID Document and publishes it in a new Alias Output.

Its functionality is equivalent to the "create DID" example
and exists for convenient calling from the other examples. */
export async function createDid(client: Client, secretManager: SecretManagerType, storage: Storage): Promise<{
    address: Address;
    document: IotaDocument;
    fragment: string;
}> {
    const didClient = new IotaIdentityClient(client);
    const networkHrp: string = await didClient.getNetworkHrp();

    const secretManagerInstance = new SecretManager(secretManager);
    const walletAddressBech32 = (await secretManagerInstance.generateEd25519Addresses({
        accountIndex: 0,
        range: {
            start: 0,
            end: 1,
        },
        bech32Hrp: networkHrp,
    }))[0];

    console.log("Wallet address Bech32:", walletAddressBech32);

    await ensureAddressHasFunds(client, walletAddressBech32);

    const address: Address = Utils.parseBech32Address(walletAddressBech32);

    // Create a new DID document with a placeholder DID.
    // The DID will be derived from the Alias Id of the Alias Output after publishing.
    const document = new IotaDocument(networkHrp);

    const fragment = await document.generateMethod(
        storage,
        JwkMemStore.ed25519KeyType(),
        JwsAlgorithm.EdDSA,
        "#jwk",
        MethodScope.AssertionMethod(),
    );

    // Construct an Alias Output containing the DID document, with the wallet address
    // set as both the state controller and governor.
    const aliasOutput: AliasOutput = await didClient.newDidOutput(address, document);

    // Publish the Alias Output and get the published DID document.
    const published = await didClient.publishDidOutput(secretManager, aliasOutput);

    return { address, document: published, fragment };
}

/** Request funds from the faucet API, if needed, and wait for them to show in the wallet. */
export async function ensureAddressHasFunds(client: Client, addressBech32: string) {
    let balance = await getAddressBalance(client, addressBech32);
    if (balance > BigInt(0)) {
        return;
    }

    await requestFundsFromFaucet(addressBech32);

    for (let i = 0; i < 9; i++) {
        // Wait for the funds to reflect.
        await new Promise(f => setTimeout(f, 5000));

        let balance = await getAddressBalance(client, addressBech32);
        if (balance > BigInt(0)) {
            break;
        }
    }
}

/** Returns the balance of the given Bech32-encoded address. */
async function getAddressBalance(client: Client, addressBech32: string): Promise<bigint> {
    const outputIds: IOutputsResponse = await client.basicOutputIds([
        { address: addressBech32 },
        { hasExpiration: false },
        { hasTimelock: false },
        { hasStorageDepositReturn: false },
    ]);
    const outputs = await client.getOutputs(outputIds.items);

    let totalAmount = BigInt(0);
    for (const output of outputs) {
        totalAmount += output.output.getAmount();
    }

    return totalAmount;
}

/** Request tokens from the faucet API. */
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
