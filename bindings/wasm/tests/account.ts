export {};

const assert = require('assert');
const {
    AccountBuilder,
    AutoSave,
    MethodScope,
    MethodContent,
    KeyType,
    MethodType,
    KeyPair,
} = require("../node");

function setupAccountBuilder() {
    return new AccountBuilder({
        autosave: AutoSave.never(),
        autopublish: false,
        clientConfig: {
            nodeSyncDisabled: true,
        }
    });
}

async function setupAccount() {
    return await setupAccountBuilder().createIdentity();
}

const privateKeyBytes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];
const ed25519PublicKeyBytes = [121, 181, 86, 46, 143, 230, 84, 249, 64, 120, 177, 18, 232, 169, 139, 167, 144, 31, 133, 58, 230, 149, 190, 215, 224, 227, 145, 11, 173, 4, 150, 100];

describe('AccountBuilder', function () {
    describe('#createIdentity()', function () {
        it('should deserialize privateKey Uint8Array correctly', async () => {
            const builder = setupAccountBuilder();
            const privateKey = new Uint8Array(privateKeyBytes);
            const account = await builder.createIdentity({
                privateKey: privateKey,
            });
            assert.equal(account.did().toString(), "did:iota:6Cm9iXWnB4RBrw7ty5u4eBbB5fHzbtjV58VLXWJ2GG8H");
        });
    });
});

describe('Account', function () {
    describe('#createMethod()', function () {
        it('should deserialize MethodContent privateKey Uint8Array correctly', async () => {
            const account = await setupAccount();

            // Test hard-coded private key.
            const fragment1 = "new-key-1";
            await account.createMethod({
                fragment: fragment1,
                content: MethodContent.PrivateEd25519(new Uint8Array(privateKeyBytes)),
            });
            const method1 = account.document().resolveMethod(fragment1, MethodScope.VerificationMethod());
            assert.equal(method1.id().fragment(), fragment1);
            assert.equal(method1.type().toString(), MethodType.Ed25519VerificationKey2018().toString());
            assert.equal(method1.data().tryDecode().toString(), ed25519PublicKeyBytes.toString());

            // Test KeyPair.
            const keypair = new KeyPair(KeyType.X25519);
            const fragment2 = "new-key-2";
            await account.createMethod({
                fragment: fragment2,
                scope: MethodScope.KeyAgreement(),
                content: MethodContent.PrivateX25519(keypair.private()),
            });
            const method2 = account.document().resolveMethod(fragment2, MethodScope.KeyAgreement());
            assert.equal(method2.id().fragment(), fragment2);
            assert.equal(method2.type().toString(), MethodType.X25519KeyAgreementKey2019().toString());
            assert.equal(method2.data().tryDecode().toString(), keypair.public().toString());
        });
        it('should deserialize MethodContent publicKey Uint8Array correctly', async () => {
            const account = await setupAccount();

            // Test hard-coded public key.
            const fragment1 = "new-key-1";
            await account.createMethod({
                fragment: fragment1,
                content: MethodContent.PublicEd25519(new Uint8Array(ed25519PublicKeyBytes)),
            });
            const method1 = account.document().resolveMethod(fragment1, MethodScope.VerificationMethod());
            assert.equal(method1.id().fragment(), fragment1);
            assert.equal(method1.type().toString(), MethodType.Ed25519VerificationKey2018().toString());
            assert.equal(method1.data().tryDecode().toString(), ed25519PublicKeyBytes.toString());

            // Test KeyPair.
            const keypair = new KeyPair(KeyType.X25519);
            const fragment2 = "new-key-2";
            await account.createMethod({
                fragment: fragment2,
                scope: MethodScope.KeyAgreement(),
                content: MethodContent.PublicX25519(keypair.public()),
            });
            const method2 = account.document().resolveMethod(fragment2, MethodScope.KeyAgreement());
            assert.equal(method2.id().fragment(), fragment2);
            assert.equal(method2.type().toString(), MethodType.X25519KeyAgreementKey2019().toString());
            assert.equal(method2.data().tryDecode().toString(), keypair.public().toString());
        });
    });
});

const credentialFields = {
    context: "https://www.w3.org/2018/credentials/examples/v1",
    id: "https://example.edu/credentials/3732",
    type: "UniversityDegreeCredential",
    credentialSubject: {
        id: "did:example:ebfeb1f712ebc6f1c276e12ec21",
        degree: {
            type: "BachelorDegree",
            name: "Bachelor of Science and Arts"
        }
    },
    issuer: "https://example.edu/issuers/565049",
    issuanceDate: "2010-01-01T00:00:00Z",
    expirationDate: "2020-01-01T19:23:24Z",
    credentialStatus: {
        id: "https://example.edu/status/24",
        type: "CredentialStatusList2017"
    },
    credentialSchema: {
        id: "https://example.org/examples/degree.json",
        type: "JsonSchemaValidator2018"
    },
    refreshService: {
        id: "https://example.edu/refresh/3732",
        type: "ManualRefreshService2018"
    },
    termsOfUse: {
        type: "IssuerPolicy",
        id: "https://example.com/policies/credential/4",
        profile: "https://example.com/profiles/credential",
        prohibition: [{
            assigner: "https://example.edu/issuers/14",
            assignee: "AllVerifiers",
            target: "https://example.edu/credentials/3732",
            action: ["Archival"]
        }]
    },
    evidence: {
        id: "https://example.edu/evidence/f2aeec97-fc0d-42bf-8ca7-0548192d4231",
        type: ["DocumentVerification"],
        verifier: "https://example.edu/issuers/14",
        evidenceDocument: "DriversLicense",
        subjectPresence: "Physical",
        documentPresence: "Physical",
        licenseNumber: "123AB4567"
    },
    nonTransferable: true,
    custom1: "asdf",
    custom2: 1234
};
