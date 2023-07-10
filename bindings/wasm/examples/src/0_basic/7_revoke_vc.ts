// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Credential,
    FailFast,
    IotaDocument,
    IotaIdentityClient,
    JwkMemStore,
    JwsSignatureOptions,
    JwtCredentialValidationOptions,
    JwtCredentialValidator,
    KeyIdMemStore,
    Resolver,
    RevocationBitmap,
    Service,
    Storage,
    VerificationMethod,
} from "@iota/identity-wasm/node";
import { AliasOutput, Client, IRent, MnemonicSecretManager, Utils } from "@iota/sdk-wasm/node";
import { API_ENDPOINT, createDid } from "../util";

/**
 * This example shows how to revoke a verifiable credential.
 * It demonstrates two methods for revocation. The first uses a revocation bitmap of type `RevocationBitmap2022`,
 * while the second method simply removes the verification method (public key) that signed the credential
 * from the DID Document of the issuer.
 *
 * Note: make sure `API_ENDPOINT` and `FAUCET_ENDPOINT` are set to the correct network endpoints.
 */
export async function revokeVC() {
    // ===========================================================================
    // Create a Verifiable Credential.
    // ===========================================================================

    const client = new Client({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });
    const didClient = new IotaIdentityClient(client);

    // Generate a random mnemonic for our wallet.
    const issuerSecretManager: MnemonicSecretManager = {
        mnemonic: Utils.generateMnemonic(),
    };

    // Create an identity for the issuer with one verification method `key-1`.
    const issuerStorage: Storage = new Storage(
        new JwkMemStore(),
        new KeyIdMemStore(),
    );
    let { document: issuerDocument, fragment: issuerFragment } = await createDid(
        client,
        issuerSecretManager,
        issuerStorage,
    );

    // Create an identity for the holder, in this case also the subject.
    const aliceStorage: Storage = new Storage(
        new JwkMemStore(),
        new KeyIdMemStore(),
    );
    let { document: aliceDocument } = await createDid(
        client,
        issuerSecretManager,
        aliceStorage,
    );

    // Create a new empty revocation bitmap. No credential is revoked yet.
    const revocationBitmap = new RevocationBitmap();

    // Add the revocation bitmap to the DID Document of the issuer as a service.
    const service: Service = new Service({
        id: issuerDocument.id().join("#my-revocation-service"),
        type: RevocationBitmap.type(),
        serviceEndpoint: revocationBitmap.toEndpoint(),
    });
    issuerDocument.insertService(service);

    // Resolve the latest output and update it with the given document.
    let aliasOutput: AliasOutput = await didClient.updateDidOutput(
        issuerDocument,
    );

    // Because the size of the DID document increased, we have to increase the allocated storage deposit.
    // This increases the deposit amount to the new minimum.
    let rentStructure: IRent = await didClient.getRentStructure();
    aliasOutput.amount = Utils.computeStorageDeposit(
        aliasOutput,
        rentStructure,
    ).toString();

    // Publish the document.
    issuerDocument = await didClient.publishDidOutput(
        issuerSecretManager,
        aliasOutput,
    );

    // Create a credential subject indicating the degree earned by Alice, linked to their DID.
    const subject = {
        id: aliceDocument.id(),
        name: "Alice",
        degreeName: "Bachelor of Science and Arts",
        degreeType: "BachelorDegree",
        GPA: "4.0",
    };

    // Create an unsigned `UniversityDegree` credential for Alice.
    // The issuer also chooses a unique `RevocationBitmap` index to be able to revoke it later.
    const CREDENTIAL_INDEX = 5;
    const unsignedVc = new Credential({
        id: "https://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        credentialStatus: {
            id: issuerDocument.id() + "#my-revocation-service",
            type: RevocationBitmap.type(),
            revocationBitmapIndex: CREDENTIAL_INDEX.toString(),
        },
        issuer: issuerDocument.id(),
        credentialSubject: subject,
    });

    // Create signed JWT credential.
    const credentialJwt = await issuerDocument.createCredentialJwt(
        issuerStorage,
        issuerFragment,
        unsignedVc,
        new JwsSignatureOptions(),
    );
    console.log(`Credential JWT > ${credentialJwt.toString()}`);

    // Validate the credential using the issuer's DID Document.
    let jwtCredentialValidator = new JwtCredentialValidator();
    jwtCredentialValidator.validate(
        credentialJwt,
        issuerDocument,
        new JwtCredentialValidationOptions({}),
        FailFast.FirstError,
    );

    // ===========================================================================
    // Revocation of the Verifiable Credential.
    // ===========================================================================

    // Update the RevocationBitmap service in the issuer's DID Document.
    // This revokes the credential's unique index.
    issuerDocument.revokeCredentials("my-revocation-service", CREDENTIAL_INDEX);

    // Publish the changes.
    aliasOutput = await didClient.updateDidOutput(issuerDocument);
    rentStructure = await didClient.getRentStructure();
    aliasOutput.amount = Utils.computeStorageDeposit(
        aliasOutput,
        rentStructure,
    ).toString();
    const update2: IotaDocument = await didClient.publishDidOutput(
        issuerSecretManager,
        aliasOutput,
    );

    // Credential verification now fails.
    try {
        jwtCredentialValidator.validate(
            credentialJwt,
            issuerDocument,
            new JwtCredentialValidationOptions({}),
            FailFast.FirstError,
        );
        console.log("Revocation Failed!");
    } catch (e) {
        console.log(`Error during validation: ${e}`);
    }

    // ===========================================================================
    // Alternative revocation of the Verifiable Credential.
    // ===========================================================================

    // By removing the verification method, that signed the credential, from the issuer's DID document,
    // we effectively revoke the credential, as it will no longer be possible to validate the signature.
    let originalMethod = issuerDocument.resolveMethod(
        `#${issuerFragment}`,
    ) as VerificationMethod;
    await issuerDocument.purgeMethod(issuerStorage, originalMethod.id());

    // Publish the changes.
    aliasOutput = await didClient.updateDidOutput(issuerDocument);
    rentStructure = await didClient.getRentStructure();
    aliasOutput.amount = Utils.computeStorageDeposit(
        aliasOutput,
        rentStructure,
    ).toString();
    issuerDocument = await didClient.publishDidOutput(
        issuerSecretManager,
        aliasOutput,
    );

    // We expect the verifiable credential to be revoked.
    const resolver = new Resolver({ client: didClient });
    try {
        // Resolve the issuer's updated DID Document to ensure the key was revoked successfully.
        const resolvedIssuerDoc = await resolver.resolve(
            issuerDocument.id().toString(),
        );
        jwtCredentialValidator.validate(
            credentialJwt,
            resolvedIssuerDoc,
            new JwtCredentialValidationOptions({}),
            FailFast.FirstError,
        );

        // `jwtCredentialValidator.validate` will throw an error, hence this will not be reached.
        console.log("Revocation failed!");
    } catch (e) {
        console.log(`Error during validation: ${e}`);
        console.log(`Credential successfully revoked!`);
    }
}
