// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    IotaDID,
    IotaDocument,
    IotaIdentityClient,
    JwkMemStore,
    JwsAlgorithm,
    KeyIdMemStore,
    MethodScope,
    Storage,
} from "@iota/identity-wasm/node";
import {
    Address,
    AliasAddress,
    AliasOutput,
    Client,
    IRent,
    IssuerFeature,
    MnemonicSecretManager,
    StateControllerAddressUnlockCondition,
    UnlockConditionType,
    Utils,
} from "@iota/sdk-wasm/node";
import { API_ENDPOINT, createDid } from "../util";

/** Demonstrates how an identity can control another identity.

For this example, we consider the case where a parent company's DID controls the DID of a subsidiary. */
export async function didControlsDid() {
    // ========================================================
    // Create the company's and subsidiary's Alias Output DIDs.
    // ========================================================

    // Create a new Client to interact with the IOTA ledger.
    const client = new Client({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });
    const didClient = new IotaIdentityClient(client);

    // Generate a random mnemonic for our wallet.
    const secretManager: MnemonicSecretManager = {
        mnemonic: Utils.generateMnemonic(),
    };

    // Creates a new wallet and identity (see "0_create_did" example).
    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
    let { document } = await createDid(
        client,
        secretManager,
        storage,
    );
    let companyDid = document.id();

    // Get the current byte costs.
    const rentStructure: IRent = await didClient.getRentStructure();

    // Get the Bech32 human-readable part (HRP) of the network.
    const networkName: string = await didClient.getNetworkHrp();

    // Construct a new DID document for the subsidiary.
    var subsidiaryDocument: IotaDocument = new IotaDocument(networkName);

    // Create the Alias Address of the company.
    const companyAliasAddress: Address = new AliasAddress(companyDid.toAliasId());

    // Create a DID for the subsidiary that is controlled by the parent company's DID.
    // This means the subsidiary's Alias Output can only be updated or destroyed by
    // the state controller or governor of the company's Alias Output respectively.
    var subsidiaryAlias: AliasOutput = await didClient.newDidOutput(
        companyAliasAddress,
        subsidiaryDocument,
        rentStructure,
    );

    // Optionally, we can mark the company as the issuer of the subsidiary DID.
    // This allows to verify trust relationships between DIDs, as a resolver can
    // verify that the subsidiary DID was created by the parent company.
    subsidiaryAlias = await client.buildAliasOutput({
        ...subsidiaryAlias,
        immutableFeatures: [new IssuerFeature(companyAliasAddress)],
        aliasId: subsidiaryAlias.getAliasId(),
        unlockConditions: subsidiaryAlias.getUnlockConditions(),
    });

    // Adding the issuer feature means we have to recalculate the required storage deposit.
    subsidiaryAlias = await client.buildAliasOutput({
        ...subsidiaryAlias,
        amount: Utils.computeStorageDeposit(subsidiaryAlias, rentStructure),
        aliasId: subsidiaryAlias.getAliasId(),
        unlockConditions: subsidiaryAlias.getUnlockConditions(),
    });

    // Publish the subsidiary's DID.
    subsidiaryDocument = await didClient.publishDidOutput(secretManager, subsidiaryAlias);

    // =====================================
    // Update the subsidiary's Alias Output.
    // =====================================

    // Add a verification method to the subsidiary.
    // This only serves as an example for updating the subsidiary DID.
    await subsidiaryDocument.generateMethod(
        storage,
        JwkMemStore.ed25519KeyType(),
        JwsAlgorithm.EdDSA,
        "#key-2",
        MethodScope.VerificationMethod(),
    );

    // Update the subsidiary's Alias Output with the updated document
    // and increase the storage deposit.
    let subsidiaryAliasUpdate: AliasOutput = await didClient.updateDidOutput(subsidiaryDocument);
    subsidiaryAliasUpdate = await client.buildAliasOutput({
        ...subsidiaryAliasUpdate,
        amount: Utils.computeStorageDeposit(subsidiaryAliasUpdate, rentStructure),
        aliasId: subsidiaryAliasUpdate.getAliasId(),
        unlockConditions: subsidiaryAliasUpdate.getUnlockConditions(),
    });

    // Publish the updated subsidiary's DID.
    //
    // This works because `secret_manager` can unlock the company's Alias Output,
    // which is required in order to update the subsidiary's Alias Output.
    subsidiaryDocument = await didClient.publishDidOutput(secretManager, subsidiaryAliasUpdate);

    // ===================================================================
    // Determine the controlling company's DID given the subsidiary's DID.
    // ===================================================================

    // Resolve the subsidiary's Alias Output.
    const subsidiaryOutput: AliasOutput = await didClient.resolveDidOutput(subsidiaryDocument.id());

    // Extract the company's Alias Id from the state controller unlock condition.
    //
    // If instead we wanted to determine the original creator of the DID,
    // we could inspect the issuer feature. This feature needs to be set when creating the DID.
    //
    // Non-null assertion is safe to use since every Alias Output has a state controller unlock condition.
    // Cast to StateControllerAddressUnlockCondition is safe as we check the type in find.
    const stateControllerUnlockCondition: StateControllerAddressUnlockCondition = subsidiaryOutput.getUnlockConditions()
        .find(
            unlockCondition => unlockCondition.getType() == UnlockConditionType.StateControllerAddress,
        )! as StateControllerAddressUnlockCondition;

    // Cast to IAliasAddress is safe because we set an Alias Address earlier.
    const companyAliasId: string = (stateControllerUnlockCondition.getAddress() as AliasAddress).getAliasId();

    // Reconstruct the company's DID from the Alias Id and the network.
    companyDid = IotaDID.fromAliasId(companyAliasId, networkName);

    // Resolve the company's DID document.
    const companyDocument: IotaDocument = await didClient.resolveDid(companyDid);

    console.log("Company ", JSON.stringify(companyDocument, null, 2));
    console.log("Subsidiary ", JSON.stringify(subsidiaryDocument, null, 2));
}
