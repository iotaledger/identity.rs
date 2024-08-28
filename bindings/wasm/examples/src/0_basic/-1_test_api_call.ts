// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ProposalAction, IotaDID, IotaDocument, KinesisIdentityClient, Multicontroller, ProposalBuilder, ControllerAndVotingPower } from "@iota/identity-wasm/node";

import { IotaClient as KinesisClient } from "@iota/iota.js/client";


async function testIdentityClient(identityClient: KinesisIdentityClient): Promise<void> {
    console.dir(await identityClient.getBalance());

    console.dir(identityClient.senderPublicKey());

    console.dir(identityClient.senderAddress());

    console.dir(identityClient.networkName());

    try {
        await identityClient.getIdentity("foobar");
    } catch(ex) {
        console.log((ex as Error).message);
    }

    const did4resolveDid = IotaDID.parse("did:iota:0x0101010101010101010101010101010101010101010101010101010101010101");
    try {
        await identityClient.resolveDid(did4resolveDid);
    } catch(ex) {
        console.log((ex as Error).message);
    }

    const document1 = new IotaDocument("foobar");
    try {
        await identityClient.publishDidDocument(document1, BigInt(12345), "dummy signer");
    } catch(ex) {
        console.log((ex as Error).message);
    }

    const document2 = new IotaDocument("foobar");
    try {
        await identityClient.publishDidDocumentUpdate(document2, BigInt(12345), "dummy signer");
    } catch(ex) {
        console.log((ex as Error).message);
    }

    const did4deactivateDidOutput = IotaDID.parse("did:iota:0x0101010101010101010101010101010101010101010101010101010101010101");
    try {
        await identityClient.deactivateDidOutput(did4deactivateDidOutput, BigInt(12345), "dummy signer");
    } catch(ex) {
        console.log((ex as Error).message);
    }
}

function testMultiController(): void {
    let multiController = new Multicontroller();

    const testCapId = "123";
    console.dir(multiController.controlledValue());
    console.dir(multiController.controllerVotingPower(testCapId));
    console.dir(multiController.hasMember(testCapId));
    console.dir(multiController.intoInner());
    console.dir(multiController.proposals());
    console.dir(multiController.threshold());
}

async function testProposals(identityClient: KinesisIdentityClient): Promise<void> {
    let action: ProposalAction = "Deactivate";
    console.dir(action);

    action = { UpdateDocument: new IotaDocument("foobar") };
    console.dir(action);
    console.dir(action.UpdateDocument);
    console.dir(action.UpdateDocument.id());
    console.dir(action.UpdateDocument.toJSON());

    let identity = await identityClient
        .createIdentity(Uint8Array.from([1, 2, 3]))
        .threshold(BigInt(1))
        .gasBudget(BigInt(1))
        .controllers([
            new ControllerAndVotingPower("one", BigInt(1)),
            new ControllerAndVotingPower("two", BigInt(2)),
        ])
        .finish(identityClient, "dummySigner")
        ;
    console.dir(identity);
    console.dir(identity.isShared());
    console.dir(identity.proposals());
    const deactivateProposal = await identity
        .deactivateDid()
        .expirationEpoch(BigInt(1))
        .gasBudget(BigInt(1))
        .key("key")
        .finish(identityClient, "dummySigner")
        ;
    console.dir(deactivateProposal);

    // proposals consume the identity instance, so we need a new one
    identity = await identityClient
        .createIdentity(Uint8Array.from([1, 2, 3]))
        .threshold(BigInt(1))
        .gasBudget(BigInt(1))
        .controllers([
            new ControllerAndVotingPower("one", BigInt(1)),
            new ControllerAndVotingPower("two", BigInt(2)),
        ])
        .finish(identityClient, "dummySigner")
        ;

    const updateProposal = await identity
        .updateDidDocument(new IotaDocument("foobar"))
        .expirationEpoch(BigInt(1))
        .gasBudget(BigInt(1))
        .key("key")
        .finish(identityClient, "dummySigner")
        ;
    console.dir(updateProposal);
}

/** Demonstrate how to create a DID Document and publish it in a new Alias Output. */
export async function testApiCall(): Promise<void> {
    const kinesis_client = new KinesisClient({ url: 'http://127.0.0.1:9000' });
    const balanceFromKinesisClient = await kinesis_client.getChainIdentifier();
    console.dir(balanceFromKinesisClient);

    // test builder
    let identityClient = KinesisIdentityClient
      .builder()
      .identityIotaPackageId('foo')
      .networkName('bar')
      .senderPublicKey(new Uint8Array([1, 2, 3, 4]))
      .iotaClient(kinesis_client)
      .build()
      ;

    await testIdentityClient(identityClient);

    testMultiController();

    await testProposals(identityClient);

    console.log("done");
}
