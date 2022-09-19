// TODO: Remove or reuse depending on what we do with the account.
// Note that this test is not executed as long as it sits in the legacy directory.
export {};

const assert = require("assert");
const {
    AccountBuilder,
    AutoSave,
    KeyType,
    KeyPair,
    MethodContent,
    MethodScope,
    MethodType,
} = require("../node");

function setupAccountBuilder() {
    return new AccountBuilder({
        autosave: AutoSave.never(),
        autopublish: false,
        clientConfig: {
            nodeSyncDisabled: true,
        },
    });
}

async function setupAccount() {
    return await setupAccountBuilder().createIdentity();
}

const privateKeyBytes = [
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    30,
    31,
    32,
];
const ed25519PublicKeyBytes = [
    121,
    181,
    86,
    46,
    143,
    230,
    84,
    249,
    64,
    120,
    177,
    18,
    232,
    169,
    139,
    167,
    144,
    31,
    133,
    58,
    230,
    149,
    190,
    215,
    224,
    227,
    145,
    11,
    173,
    4,
    150,
    100,
];

describe("AccountBuilder", function() {
    describe("#createIdentity()", function() {
        it("should deserialize privateKey Uint8Array correctly", async () => {
            const builder = setupAccountBuilder();
            const privateKey = new Uint8Array(privateKeyBytes);
            const account = await builder.createIdentity({
                privateKey: privateKey,
            });
            assert.equal(account.did().toString(), "did:iota:6Cm9iXWnB4RBrw7ty5u4eBbB5fHzbtjV58VLXWJ2GG8H");
        });
    });
});

describe("Account", function() {
    describe("#createMethod()", function() {
        it("should deserialize MethodContent privateKey Uint8Array correctly", async () => {
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
        it("should deserialize MethodContent publicKey Uint8Array correctly", async () => {
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
    describe("#createService()", function() {
        it("should take one type and endpoint", async () => {
            const account = await setupAccount();

            // Test single type & endpoint.
            const fragment1 = "new-service-1";
            await account.createService({
                fragment: fragment1,
                type: "LinkedDomains",
                endpoint: "https://example.com/",
            });
            const service = account.document().resolveService(fragment1);
            assert.deepStrictEqual(service.id().fragment(), fragment1);
            assert.deepStrictEqual(service.type(), ["LinkedDomains"]);
            assert.deepStrictEqual(service.serviceEndpoint(), "https://example.com/");
        });
        it("should take multiple types and endpoints", async () => {
            const account = await setupAccount();

            // Test multiple types & endpoints.
            const fragment1 = "new-service-1";
            await account.createService({
                fragment: fragment1,
                type: ["LinkedDomains", "ExampleType"],
                endpoint: ["https://example.com/", "https://iota.org/"],
            });
            const service = account.document().resolveService(fragment1);
            assert.deepStrictEqual(service.id().fragment(), fragment1);
            assert.deepStrictEqual(service.type(), ["LinkedDomains", "ExampleType"]);
            assert.deepStrictEqual(service.serviceEndpoint(), ["https://example.com/", "https://iota.org/"]);
        });
    });
});
