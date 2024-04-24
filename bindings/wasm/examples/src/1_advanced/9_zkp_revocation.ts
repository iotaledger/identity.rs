import {
    Credential,
    Duration,
    FailFast,
    IotaDID,
    IotaDocument,
    IotaIdentityClient,
    JptCredentialValidationOptions,
    JptCredentialValidator,
    JptCredentialValidatorUtils,
    JptPresentationValidationOptions,
    JptPresentationValidator,
    JptPresentationValidatorUtils,
    JwkMemStore,
    JwpCredentialOptions,
    JwpPresentationOptions,
    KeyIdMemStore,
    MethodScope,
    ProofAlgorithm,
    RevocationBitmap,
    RevocationTimeframeStatus,
    SelectiveDisclosurePresentation,
    Status,
    StatusCheck,
    Storage,
    Timestamp,
} from "@iota/identity-wasm/node";
import {
    type Address,
    AliasOutput,
    Client,
    MnemonicSecretManager,
    SecretManager,
    SecretManagerType,
    Utils,
} from "@iota/sdk-wasm/node";
import { API_ENDPOINT, ensureAddressHasFunds } from "../util";

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

    const fragment = await document.generateMethodJwp(
        storage,
        ProofAlgorithm.BLS12381_SHA256,
        undefined,
        MethodScope.VerificationMethod(),
    );
    const revocationBitmap = new RevocationBitmap();
    const serviceId = document.id().toUrl().join("#my-revocation-service");
    const service = revocationBitmap.toService(serviceId);

    document.insertService(service);
    // Construct an Alias Output containing the DID document, with the wallet address
    // set as both the state controller and governor.
    const aliasOutput: AliasOutput = await didClient.newDidOutput(address, document);

    // Publish the Alias Output and get the published DID document.
    const published = await didClient.publishDidOutput(secretManager, aliasOutput);

    return { address, document: published, fragment };
}
export async function zkp_revocation() {
    // Create a new client to interact with the IOTA ledger.
    const client = new Client({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });

    // Creates a new wallet and identity (see "0_create_did" example).
    const issuerSecretManager: MnemonicSecretManager = {
        mnemonic: Utils.generateMnemonic(),
    };
    const issuerStorage: Storage = new Storage(
        new JwkMemStore(),
        new KeyIdMemStore(),
    );
    let { document: issuerDocument, fragment: issuerFragment } = await createDid(
        client,
        issuerSecretManager,
        issuerStorage,
    );
    const holderSecretManager: MnemonicSecretManager = {
        mnemonic: Utils.generateMnemonic(),
    };
    const holderStorage: Storage = new Storage(
        new JwkMemStore(),
        new KeyIdMemStore(),
    );
    let { document: holderDocument, fragment: holderFragment } = await createDid(
        client,
        holderSecretManager,
        holderStorage,
    );
    // =========================================================================================
    // Step 1: Create a new RevocationTimeframeStatus containing the current validityTimeframe
    // =======================================================================================

    const timeframeId = issuerDocument.id().toUrl().join("#my-revocation-service");
    let revocationTimeframeStatus = new RevocationTimeframeStatus(
        timeframeId.toString(),
        5,
        Duration.minutes(1),
        Timestamp.nowUTC(),
    );

    // Create a credential subject indicating the degree earned by Alice.
    const subject = {
        name: "Alice",
        mainCourses: ["Object-oriented Programming", "Mathematics"],
        degree: {
            type: "BachelorDegree",
            name: "Bachelor of Science and Arts",
        },
        GPA: 4.0,
    };

    // Build credential using the above subject and issuer.
    const credential = new Credential({
        id: "https:/example.edu/credentials/3732",
        issuer: issuerDocument.id(),
        type: "UniversityDegreeCredential",
        credentialSubject: subject,
        credentialStatus: revocationTimeframeStatus as any as Status,
    });
    const credentialJpt = await issuerDocument
        .createCredentialJpt(
            credential,
            issuerStorage,
            issuerFragment,
            new JwpCredentialOptions(),
        );
    // Validate the credential's proof using the issuer's DID Document, the credential's semantic structure,
    // that the issuance date is not in the future and that the expiration date is not in the past:
    const decodedJpt = JptCredentialValidator.validate(
        credentialJpt,
        issuerDocument,
        new JptCredentialValidationOptions(),
        FailFast.FirstError,
    );

    console.log("Sending credential (as JPT) to the holder: " + credentialJpt.toString());

    // Holder validates the credential and retrieve the JwpIssued, needed to construct the JwpPresented
    let decodedCredential = JptCredentialValidator.validate(
        credentialJpt,
        issuerDocument,
        new JptCredentialValidationOptions(),
        FailFast.FirstError,
    );

    // ===========================================================================
    // Credential's Status check
    // ===========================================================================
    JptCredentialValidatorUtils.checkTimeframesAndRevocationWithValidityTimeframe2024(
        decodedCredential.credential(),
        issuerDocument,
        undefined,
        StatusCheck.Strict,
    );

    // A unique random challenge generated by the requester per presentation can mitigate replay attacks.
    const challenge = "475a7984-1bb5-4c4c-a56f-822bccd46440";

    const methodId = decodedCredential
        .decodedJwp()
        .getIssuerProtectedHeader()
        .kid!;

    const selectiveDisclosurePresentation = new SelectiveDisclosurePresentation(decodedCredential.decodedJwp());
    selectiveDisclosurePresentation.concealInSubject("mainCourses[1]");
    selectiveDisclosurePresentation.concealInSubject("degree.name");

    // Construct a JPT(JWP in the Presentation form) representing the Selectively Disclosed Verifiable Credential
    const presentationOptions = new JwpPresentationOptions();
    presentationOptions.nonce = challenge;
    const presentationJpt = await issuerDocument
        .createPresentationJpt(
            selectiveDisclosurePresentation,
            methodId,
            presentationOptions,
        );

    console.log("Sending presentation (as JPT) to the verifier: " + presentationJpt.toString());

    // ===========================================================================
    // Step 2: Verifier receives the Presentation and verifies it.
    // ===========================================================================

    const presentationValidationOptions = new JptPresentationValidationOptions({ nonce: challenge });
    const decodedPresentedCredential = JptPresentationValidator.validate(
        presentationJpt,
        issuerDocument,
        presentationValidationOptions,
        FailFast.FirstError,
    );

    JptPresentationValidatorUtils.checkTimeframesWithValidityTimeframe2024(
        decodedPresentedCredential.credential(),
        undefined,
        StatusCheck.Strict,
    );

    console.log("Presented credential successfully validated: " + decodedPresentedCredential.credential());

    // ===========================================================================
    // Step 2b: Waiting for the next validityTimeframe, will result in the Credential timeframe interval NOT valid
    // ===========================================================================

    try {
        const now = new Date();
        const timeInTwoMinutes = new Date(now.setMinutes(now.getMinutes() + 2));
        JptPresentationValidatorUtils.checkTimeframesWithValidityTimeframe2024(
            decodedPresentedCredential.credential(),
            Timestamp.parse(timeInTwoMinutes.toISOString()),
            StatusCheck.Strict,
        );
    } catch (_) {
        console.log("successfully expired!");
    }

    // ===========================================================================
    // Issuer decides to Revoke Holder's Credential
    // ===========================================================================

    console.log("Issuer decides to revoke the Credential");

    const identityClient = new IotaIdentityClient(client);

    // Update the RevocationBitmap service in the issuer's DID Document.
    // This revokes the credential's unique index.
    issuerDocument.revokeCredentials("my-revocation-service", 5);
    let aliasOutput = await identityClient.updateDidOutput(issuerDocument);
    const rent = await identityClient.getRentStructure();
    aliasOutput = await client.buildAliasOutput({
        ...aliasOutput,
        amount: Utils.computeStorageDeposit(aliasOutput, rent),
        aliasId: aliasOutput.getAliasId(),
        unlockConditions: aliasOutput.getUnlockConditions(),
    });
    issuerDocument = await identityClient.publishDidOutput(issuerSecretManager, aliasOutput);

    // Holder checks if his credential has been revoked by the Issuer
    try {
        JptCredentialValidatorUtils.checkRevocationWithValidityTimeframe2024(
            decodedCredential.credential(),
            issuerDocument,
            StatusCheck.Strict,
        );
    } catch (_) {
        console.log("Credential revoked!");
    }
}
