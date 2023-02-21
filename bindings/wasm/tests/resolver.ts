export {};

import {
    CoreDID,
    CoreDocument,
    FailFast,
    IotaDID,
    IotaDocument,
    IToCoreDocument,
    Presentation,
    PresentationValidationOptions,
    Resolver,
} from "../node";
import assert = require("assert");

const presentationJSON = require("../../../identity_credential/tests/fixtures/signed_presentation/presentation.json");
const issuerIotaDocJSON = require(
    "../../../identity_credential/tests/fixtures/signed_presentation/issuer_iota_doc.json",
);
const issuerBarDocJSON = require("../../../identity_credential/tests/fixtures/signed_presentation/issuer_bar_doc.json");
const holderFooDocJSON = require(
    "../../../identity_credential/tests/fixtures/signed_presentation/subject_foo_doc.json",
);
const presentation = Presentation.fromJSON(presentationJSON);
const holderFooDoc = CoreDocument.fromJSON(holderFooDocJSON);
const issuerIotaDoc: IotaDocument = IotaDocument.fromJSON(issuerIotaDocJSON);
const issuerBarDoc: CoreDocument = CoreDocument.fromJSON(issuerBarDocJSON);

class MockFooDocument {
    inner: CoreDocument;
    constructor(inner: CoreDocument) {
        this.inner = inner;
    }

    toCoreDocument() : CoreDocument {
        return this.inner;
    }
}

describe("Resolver", function() {
    describe("#verifyPresentation", function() {
        it("should accept a correct presentation when configured correctly", async () => {
            // mock method handlers
            const resolveDidIota = async function(did_input: string) {
                const parsedDid: IotaDID = IotaDID.parse(did_input);
                if (issuerIotaDoc.id().toString() == parsedDid.toString()) {
                    return issuerIotaDoc;
                } else {
                    throw new Error(`could not resolve did ${did_input}`);
                }
            };

            const resolveDidFoo = async function(did_input: string) {
                const parsedDid: CoreDID = CoreDID.parse(did_input);
                if (holderFooDoc.id().toString() == parsedDid.toString()) {
                    let doc = new MockFooDocument(holderFooDoc);
                    return doc;
                } else {
                    throw new Error(`could not resolve did ${did_input}`);
                }
            };

            const resolveDidBar = async function(did_input: string) {
                const parsedDid: CoreDID = CoreDID.parse(did_input);
                if (issuerBarDoc.id().toString() == parsedDid.toString()) {
                    return issuerBarDoc;
                } else {
                    throw new Error(`could not resolve did ${did_input}`);
                }
            };

            let handlerMap: Map<string, (did: string) => Promise<CoreDocument | IToCoreDocument>> = new Map();
            handlerMap.set("iota", resolveDidIota);
            handlerMap.set("foo", resolveDidFoo);
            handlerMap.set("bar", resolveDidBar);

            const resolver = new Resolver({
                handlers: handlerMap,
            });

            const resolvedHolderDoc = await resolver.resolvePresentationHolder(presentation);
            assert(resolvedHolderDoc instanceof MockFooDocument);
            assert(!(resolvedHolderDoc instanceof CoreDocument));
            // Check that we are not leaking memory. 
            assert.deepStrictEqual(resolvedHolderDoc.inner._strongCountInternal() as number, 1);

            // Also check with Promise.any and Promise.all 
            let promise0 = resolver.resolvePresentationHolder(presentation);
            let promise1 = resolver.resolvePresentationHolder(presentation);
            let holderDocFromPromiseAny = await Promise.any([promise0, promise1]); 
            assert.deepStrictEqual(holderDocFromPromiseAny.inner._strongCountInternal() as number, 1);

            let promise2 = resolver.resolvePresentationHolder(presentation);
            let promise3 = resolver.resolvePresentationHolder(presentation);
            let [out2, out3] = await Promise.all([promise2, promise3]);
            assert.deepStrictEqual(out2.inner._strongCountInternal() as number, 1);
            assert.deepStrictEqual(out3.inner._strongCountInternal() as number, 1); 


            const resolvedIssuerDocuments = await resolver.resolvePresentationIssuers(presentation);

            assert(resolvedIssuerDocuments instanceof Array);
            // Check that we are not leaking memory
            for (let document of resolvedIssuerDocuments) {
                assert.deepStrictEqual(document._strongCountInternal() as number, 1);
            }

            let verificationResultPassingHolderDoc = await resolver.verifyPresentation(
                presentation,
                PresentationValidationOptions.default(),
                FailFast.FirstError,
                resolvedHolderDoc,
                undefined,
            );
            assert.equal(verificationResultPassingHolderDoc, undefined);

            let verificationResultPassingHolderAndIssuerDocuments = await resolver.verifyPresentation(
                presentation,
                PresentationValidationOptions.default(),
                FailFast.FirstError,
                resolvedHolderDoc,
                resolvedIssuerDocuments,
            );

            // Check that we are not leaking memory when calling verifyPresentation. 
            assert.deepStrictEqual(
                resolvedHolderDoc.inner._strongCountInternal() as number, 
                1
            );

            for (let doc of resolvedIssuerDocuments) {
                assert.deepStrictEqual(
                    doc._strongCountInternal() as number, 
                    1
                );
            }

            assert.equal(verificationResultPassingHolderAndIssuerDocuments, undefined);

            let verificationResultPassingIssuerDocuments = await resolver.verifyPresentation(
                presentation,
                PresentationValidationOptions.default(),
                FailFast.FirstError,
                undefined,
                resolvedIssuerDocuments,
            );
            assert.equal(verificationResultPassingIssuerDocuments, undefined);

            let verificationResultPassingNoDocuments = await resolver.verifyPresentation(
                presentation,
                PresentationValidationOptions.default(),
                FailFast.FirstError,
                undefined,
                undefined,
            );
            assert.equal(verificationResultPassingNoDocuments, undefined);

            // passing the wrong document should throw an error
            assert.notEqual(resolvedHolderDoc, resolvedIssuerDocuments[0]);

            try {
                let result = await resolver.verifyPresentation(
                    presentation,
                    PresentationValidationOptions.default(),
                    FailFast.FirstError,
                    resolvedIssuerDocuments[0],
                    undefined,
                );
            } catch (e) {
                return;
            }
            throw new Error("no error thrown when passing incorrect holder");
        });

        it("should fail presentation validation when configured incorrectly", async () => {
            // setup mock handlers returning DID documents from other methods
            const resolveDidIotaMisconfigured = async function(_did_input: string) {
                return holderFooDoc;
            };

            const resolveDidFooMisconfigured = async function(_did_input: string) {
                return issuerBarDoc;
            };

            const resolveDidBarMisconfigured = async function(did_input: string) {
                return issuerIotaDoc;
            };

            let handlerMap: Map<string, (did: string) => Promise<IotaDocument | CoreDocument>> = new Map();
            handlerMap.set("iota", resolveDidIotaMisconfigured);
            handlerMap.set("foo", resolveDidFooMisconfigured);
            handlerMap.set("bar", resolveDidBarMisconfigured);

            const resolver = new Resolver({
                handlers: handlerMap,
            });

            try {
                await resolver.verifyPresentation(
                    presentation,
                    PresentationValidationOptions.default(),
                    FailFast.FirstError,
                    undefined,
                    undefined,
                );
            } catch (e) {
                if (e instanceof Error) {
                    assert.equal("CompoundPresentationValidationError", e.name);
                    return;
                }
            }

            throw new Error(
                "the incorrectly configured resolver did not throw the expected error when validating the presentation",
            );
        });
    });
});
