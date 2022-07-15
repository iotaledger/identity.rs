export {};

const assert = require('assert');
const {
    Document,
    KeyType,
    KeyPair,
    Service,
} = require("../node");

describe('Document', function () {
    describe('#insertService()', function () {
        it('should take one type', async () => {
            const keypair = new KeyPair(KeyType.Ed25519);
            const doc = new Document(keypair);

            // Test single type.
            const fragment1 = "new-service-1";
            doc.insertService(new Service({
                id: doc.id().toUrl().join('#' + fragment1),
                type: "LinkedDomains",
                serviceEndpoint: {
                    "origins": ["https://iota.org/", "https://example.com/"]
                },
            }));
            const service = doc.resolveService(fragment1);
            assert.deepStrictEqual(service.id().fragment(), fragment1);
            assert.deepStrictEqual(service.type(), ["LinkedDomains"]);
            assert.deepStrictEqual(service.serviceEndpoint(), new Map<string, string[]>([
                ["origins", ["https://iota.org/", "https://example.com/"]],
            ]));
        });
        it('should take multiple types', async () => {
            const keypair = new KeyPair(KeyType.Ed25519);
            const doc = new Document(keypair);

            // Test multiple types.
            const fragment1 = "new-service-1";
            doc.insertService(new Service({
                id: doc.id().toUrl().join('#' + fragment1),
                type: ["LinkedDomains", "ExampleType"],
                serviceEndpoint: ["https://example.com/", "https://iota.org/"],
            }));
            const service = doc.resolveService(fragment1);
            assert.deepStrictEqual(service.id().fragment(), fragment1);
            assert.deepStrictEqual(service.type(), ["LinkedDomains", "ExampleType"]);
            assert.deepStrictEqual(service.serviceEndpoint(), ["https://example.com/", "https://iota.org/"]);
        });
    });
});
