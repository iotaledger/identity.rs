// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Bip39 } from "@iota/crypto.js";
import {
    IotaDID,
    IotaDocument,
    IotaIdentityClient,
    IotaVerificationMethod,
    KeyPair,
    KeyType,
    MethodScope,
} from "@iota/identity-wasm/node";
import { Client, MnemonicSecretManager } from "@iota/iota-client-wasm/node";
import {
    AddressTypes,
    ALIAS_ADDRESS_TYPE,
    IAliasAddress,
    IAliasOutput,
    IRent,
    ISSUER_FEATURE_TYPE,
    IStateControllerAddressUnlockCondition,
    STATE_CONTROLLER_ADDRESS_UNLOCK_CONDITION_TYPE,
    TransactionHelper,
} from "@iota/iota.js";
import { Converter } from "@iota/util.js";
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
        mnemonic: Bip39.randomMnemonic(),
    };

    // Creates a new wallet and identity (see "0_create_did" example).
    var { document } = await createDid(client, secretManager);
    let companyDid = document.id();

    // Get the current byte costs.
    const rentStructure: IRent = await didClient.getRentStructure();

    // Get the Bech32 human-readable part (HRP) of the network.
    const networkName: string = await didClient.getNetworkHrp();

    // Construct a new DID document for the subsidiary.
    var subsidiaryDocument: IotaDocument = new IotaDocument(networkName);

    // Create the Alias Address of the company.
    const companyAliasAddress: AddressTypes = {
        aliasId: companyDid.toAliasId(),
        type: ALIAS_ADDRESS_TYPE,
    };

    // Create a DID for the subsidiary that is controlled by the parent company's DID.
    // This means the subsidiary's Alias Output can only be updated or destroyed by
    // the state controller or governor of the company's Alias Output respectively.
    var subsidiaryAlias: IAliasOutput = await didClient.newDidOutput(
        companyAliasAddress,
        subsidiaryDocument,
        rentStructure,
    );

    // Optionally, we can mark the company as the issuer of the subsidiary DID.
    // This allows to verify trust relationships between DIDs, as a resolver can
    // verify that the subsidiary DID was created by the parent company.
    subsidiaryAlias = {
        ...subsidiaryAlias,
        immutableFeatures: [
            {
                type: ISSUER_FEATURE_TYPE,
                address: companyAliasAddress,
            },
        ],
    };

    // Adding the issuer feature means we have to recalculate the required storage deposit.
    subsidiaryAlias.amount = TransactionHelper.getStorageDeposit(subsidiaryAlias, rentStructure).toString();

    // Publish the subsidiary's DID.
    subsidiaryDocument = await didClient.publishDidOutput(secretManager, subsidiaryAlias);

    // =====================================
    // Update the subsidiary's Alias Output.
    // =====================================

    // Add a verification method to the subsidiary.
    // This only serves as an example for updating the subsidiary DID.
    const keyPair: KeyPair = new KeyPair(KeyType.Ed25519);
    const method: IotaVerificationMethod = new IotaVerificationMethod(
        subsidiaryDocument.id(),
        KeyType.Ed25519,
        keyPair.public(),
        "#key-2",
    );
    subsidiaryDocument.insertMethod(method, MethodScope.VerificationMethod());

    // Update the subsidiary's Alias Output with the updated document
    // and increase the storage deposit.
    const subsidiaryAliasUpdate: IAliasOutput = await didClient.updateDidOutput(subsidiaryDocument);
    subsidiaryAliasUpdate.amount = TransactionHelper.getStorageDeposit(subsidiaryAliasUpdate, rentStructure).toString();

    // Publish the updated subsidiary's DID.
    //
    // This works because `secret_manager` can unlock the company's Alias Output,
    // which is required in order to update the subsidiary's Alias Output.
    subsidiaryDocument = await didClient.publishDidOutput(secretManager, subsidiaryAliasUpdate);

    // ===================================================================
    // Determine the controlling company's DID given the subsidiary's DID.
    // ===================================================================

    // Resolve the subsidiary's Alias Output.
    const subsidiaryOutput: IAliasOutput = await didClient.resolveDidOutput(subsidiaryDocument.id());

    // Extract the company's Alias Id from the state controller unlock condition.
    //
    // If instead we wanted to determine the original creator of the DID,
    // we could inspect the issuer feature. This feature needs to be set when creating the DID.
    //
    // Non-null assertion is safe to use since every Alias Output has a state controller unlock condition.
    // Cast to IStateControllerAddressUnlockCondition is safe as we check the type in find.
    const stateControllerUnlockCondition: IStateControllerAddressUnlockCondition = subsidiaryOutput.unlockConditions
        .find(
            unlockCondition => unlockCondition.type == STATE_CONTROLLER_ADDRESS_UNLOCK_CONDITION_TYPE,
        )! as IStateControllerAddressUnlockCondition;

    // Cast to IAliasAddress is safe because we set an Alias Address earlier.
    const companyAliasId: string = (stateControllerUnlockCondition.address as IAliasAddress).aliasId;

    // Reconstruct the company's DID from the Alias Id and the network.
    companyDid = IotaDID.fromAliasId(companyAliasId, networkName);

    // Resolve the company's DID document.
    const companyDocument: IotaDocument = await didClient.resolveDid(companyDid);

    console.log("Company ", JSON.stringify(companyDocument, null, 2));
    console.log("Subsidiary ", JSON.stringify(subsidiaryDocument, null, 2));
}
