import {
    Credential,
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
    SelectiveDisclosurePresentation,
    Storage,
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
        ProofAlgorithm.BBS,
        undefined,
        MethodScope.VerificationMethod(),
    );
    // Construct an Alias Output containing the DID document, with the wallet address
    // set as both the state controller and governor.
    const aliasOutput: AliasOutput = await didClient.newDidOutput(address, document);

    // Publish the Alias Output and get the published DID document.
    const published = await didClient.publishDidOutput(secretManager, aliasOutput);

    return { address, document: published, fragment };
}
export async function zkp() {
    // ===========================================================================
    // Step 1: Create identity for the issuer.
    // ===========================================================================

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

    // ===========================================================================
    // Step 2: Issuer creates and signs a Verifiable Credential with BBS algorithm.
    // ===========================================================================

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

    // ===========================================================================
    // Step 3: Issuer sends the Verifiable Credential to the holder.
    // ===========================================================================
    console.log("Sending credential (as JPT) to the holder: " + credentialJpt.toString());

    // ============================================================================================
    // Step 4: Holder resolve Issuer's DID, retrieve Issuer's document and validate the Credential
    // ============================================================================================
    const identityClient = new IotaIdentityClient(client);

    // Holder resolves issuer's DID.
    let issuerDid = IotaDID.parse(JptCredentialValidatorUtils.extractIssuerFromIssuedJpt(credentialJpt).toString());
    let issuerDoc = await identityClient.resolveDid(issuerDid);

    // Holder validates the credential and retrieve the JwpIssued, needed to construct the JwpPresented
    let decodedCredential = JptCredentialValidator.validate(
        credentialJpt,
        issuerDoc,
        new JptCredentialValidationOptions(),
        FailFast.FirstError,
    );

    // ===========================================================================
    // Step 5: Verifier sends the holder a challenge and requests a Presentation.
    //
    // Please be aware that when we mention "Presentation," we are not alluding to the Verifiable Presentation standard as defined by W3C (https://www.w3.org/TR/vc-data-model/#presentations).
    // Instead, our reference is to a JWP Presentation (https://datatracker.ietf.org/doc/html/draft-ietf-jose-json-web-proof#name-presented-form), which differs from the W3C standard.
    // ===========================================================================

    // A unique random challenge generated by the requester per presentation can mitigate replay attacks.
    const challenge = "475a7984-1bb5-4c4c-a56f-822bccd46440";

    // =========================================================================================================
    // Step 6: Holder engages in the Selective Disclosure of credential's attributes.
    // =========================================================================================================
    const methodId = decodedCredential
        .decodedJwp()
        .getIssuerProtectedHeader()
        .kid!;
    const selectiveDisclosurePresentation = new SelectiveDisclosurePresentation(decodedCredential.decodedJwp());
    selectiveDisclosurePresentation.concealInSubject("mainCourses[1]");
    selectiveDisclosurePresentation.concealInSubject("degree.name");

    // =======================================================================================================================================
    // Step 7: Holder needs Issuer's Public Key to compute the Signature Proof of Knowledge and construct the Presentation
    // JPT.
    // =======================================================================================================================================

    // Construct a JPT(JWP in the Presentation form) representing the Selectively Disclosed Verifiable Credential
    const presentationOptions = new JwpPresentationOptions();
    presentationOptions.nonce = challenge;
    const presentationJpt = await issuerDoc
        .createPresentationJpt(
            selectiveDisclosurePresentation,
            methodId,
            presentationOptions,
        );

    // ===========================================================================
    // Step 8: Holder sends a Presentation JPT to the Verifier.
    // ===========================================================================

    console.log("Sending presentation (as JPT) to the verifier: " + presentationJpt.toString());

    // ===========================================================================
    // Step 9: Verifier receives the Presentation and verifies it.
    // ===========================================================================

    // Verifier resolve Issuer DID
    const issuerDidV = IotaDID.parse(
        JptPresentationValidatorUtils.extractIssuerFromPresentedJpt(presentationJpt).toString(),
    );
    const issuerDocV = await identityClient.resolveDid(issuerDidV);

    const presentationValidationOptions = new JptPresentationValidationOptions({ nonce: challenge });
    const decodedPresentedCredential = JptPresentationValidator.validate(
        presentationJpt,
        issuerDocV,
        presentationValidationOptions,
        FailFast.FirstError,
    );

    console.log("Presented credential successfully validated: " + decodedPresentedCredential.credential());
}
