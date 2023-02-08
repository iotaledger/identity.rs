// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, MnemonicSecretManager } from "@iota/client-wasm/node";
import { Bip39 } from "@iota/crypto.js";
import {
    Credential,
    CredentialValidationOptions,
    DIDUrl,
    DomainLinkageConfiguration,
    DomainLinkageValidator,
    Duration,
    IotaDID,
    IotaDocument,
    IotaIdentityClient,
    LinkedDomainService,
    ProofOptions,
    Timestamp,
} from "@iota/identity-wasm/node";
import { IAliasOutput, IRent, TransactionHelper } from "@iota/iota.js";
import { API_ENDPOINT, createDid } from "../util";

/**
 * Demonstrates how to link a domain and a DID and verify the linkage.
 */
export async function domainLinkage() {
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
    let { document, keypair } = await createDid(client, secretManager);
    const did: IotaDID = document.id();

    // =====================================================
    // Create Linked Domain service
    // ====================================================

    let domainFoo = "https://foo.example.com";
    let domainBar = "https://bar.example.com";

    // Create a Linked Domain Service to enable the discovery of the linked domains through the DID Document.
    // This is optional
    let serviceUrl: DIDUrl = did.clone().join("#domain_linkage");
    let linkedDomainService: LinkedDomainService = new LinkedDomainService({
        id: serviceUrl,
        domains: [domainFoo, domainBar],
    });
    document.insertService(linkedDomainService.toService());
    let updatedDidDocument = await publishDocument(didClient, secretManager, document);
    console.log("Updated DID document:", JSON.stringify(updatedDidDocument, null, 2));

    // =====================================================
    // Create DID Configuration resource
    // =====================================================

    // Now the DID Document contains a service that includes the domains.
    // To allow a bidirectional linkage, the domains must link to the DID. This is
    // done by creating a `DID Configuration Resource` that includes a `Domain Linkage Credential`
    // and can be made available on the domain.

    // Create the Domain Linkage Credential.
    let domainLinkageCredential: Credential = Credential.createDomainLinkageCredential({
        issuer: document.id(),
        origin: domainFoo,
        expirationDate: Timestamp.nowUTC().checkedAdd(Duration.weeks(10))!,
    });

    // Sign the credential.
    domainLinkageCredential = document.signCredential(
        domainLinkageCredential,
        keypair.private(),
        "#key-1",
        ProofOptions.default(),
    );

    // Create the DID Configuration Resource which wraps the Domain Linkage credential.
    let configurationResource: DomainLinkageConfiguration = new DomainLinkageConfiguration([domainLinkageCredential]);

    // The DID Configuration resource can be made available on `https://foo.example.com/.well-known/did-configuration.json`.
    let configurationResourceJson = configurationResource.toJSON();
    console.log("Configuration Resource:", JSON.stringify(configurationResource.toJSON(), null, 2));

    // Now the DID Document links to the Domains through the service, and the Foo domain links to the DID
    // through the DID Configuration resource. A bidirectional linkage is established.
    // Note however that bidirectionality is not a hard requirement. It is valid to have a Domain Linkage
    // credential point to a DID, without the DID having a service that points back.

    // =====================================================
    // Verification can start from two different places.
    // The first case answers the question "What DID is this domain linked to?"
    // while the second answers "What domain is this DID linked to?".
    // ====================================================

    // =====================================================
    // → Case 1: starting from domain
    // =====================================================

    // Fetch the DID Configuration resource (For example using the Fetch API).
    // Note that redirection must be disabled when fetching.
    const _configurationUrl = `${domainFoo}/.well-known/did-configuration.json")`;

    // But since the DID Configuration
    // resource isn't available online in this example, we will simply use the JSON.
    let fetchedConfigurationResource = DomainLinkageConfiguration.fromJSON(configurationResource);

    // Retrieve the issuers of the Domain Linkage Credentials which correspond to the possibly linked DIDs.
    let issuers: Array<string> = fetchedConfigurationResource.issuers();
    const issuerDocument: IotaDocument = await didClient.resolveDid(IotaDID.parse(issuers[0]));

    // Validate the linkage between the Domain Linkage Credential in the configuration and the provided issuer DID.
    // Validation is succeeds when no error is thrown.
    DomainLinkageValidator.validateLinkage({
        domain: domainFoo,
        configuration: fetchedConfigurationResource,
        issuer: issuerDocument,
        validationOptions: CredentialValidationOptions.default(),
    });

    // =====================================================
    // → Case 2: starting from a DID
    // =====================================================

    const didDocument: IotaDocument = await didClient.resolveDid(did);

    // Get the Linked Domain Services from the DID Document.
    let linkedDomainServices: LinkedDomainService[] = didDocument
        .service()
        .filter(service => LinkedDomainService.isValid(service))
        .map(service => LinkedDomainService.fromService(service));

    // Get the domains included in the Linked Domain Service.
    let domains: string[] = linkedDomainServices[0].domains();

    // Fetch the DID Configuration resource (For example using the Fetch API).
    // Note that redirection must be disabled when fetching.
    const __configurationUrl = `${domains[0]}/.well-known/did-configuration.json")`;

    // But since the DID Configuration
    // resource isn't available online in this example, we will simply use the JSON.
    fetchedConfigurationResource = DomainLinkageConfiguration.fromJSON(configurationResource);

    // Validate the linkage between the Domain Linkage Credential in the configuration and the provided issuer DID.
    // Validation is succeeds when no error is thrown.
    DomainLinkageValidator.validateLinkage({
        domain: domains[0],
        configuration: fetchedConfigurationResource,
        issuer: didDocument,
        validationOptions: CredentialValidationOptions.default(),
    });
}

async function publishDocument(
    client: IotaIdentityClient,
    secretManager: MnemonicSecretManager,
    document: IotaDocument,
): Promise<IotaDocument> {
    // Resolve the latest output and update it with the given document.
    const aliasOutput: IAliasOutput = await client.updateDidOutput(document);

    // Because the size of the DID document increased, we have to increase the allocated storage deposit.
    // This increases the deposit amount to the new minimum.
    const rentStructure: IRent = await client.getRentStructure();
    aliasOutput.amount = TransactionHelper.getStorageDeposit(aliasOutput, rentStructure).toString();

    // Publish the output.
    const updated: IotaDocument = await client.publishDidOutput(secretManager, aliasOutput);
    return updated;
}
