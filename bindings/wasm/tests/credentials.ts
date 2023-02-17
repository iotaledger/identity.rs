import type { CoreDocument, ICoreDocument, IotaDocument } from "../node";

export {};

const assert = require("assert");
const {
    Credential,
    CredentialValidator,
    CredentialValidationOptions,
    FailFast,
    KeyType,
    MethodScope,
    Presentation,
    ProofOptions,
    RevocationBitmap,
    IotaDocument,
    Service,
    VerificationMethod,
    StatusCheck,
    SubjectHolderRelationship,
    Timestamp,
    KeyPair,
    VerifierOptions,
    PresentationValidator,
    PresentationValidationOptions,
    CoreDocument,
    ICoreDocument,
} = require("../node");

// Mocks for characterisation tests ensuring that toCoreDocument()
// only gets called when a document cannot be cast to either IotaDocument
// or CoreDocument.
class MockInheritedCoreDocument extends CoreDocument {
    toCoreDocumentCalled: boolean;

    constructor(values: ICoreDocument) {
        super(values);
        this.toCoreDocumentCalled = false;
    }

    toCoreDocument(): CoreDocument {
        this.toCoreDocumentCalled = true;
        return super.toCoreDocument();
    }
}

class MockIToCoreDocument {
    inner: CoreDocument;
    toCoreDocumentCalled: boolean;

    constructor(inner: CoreDocument) {
        this.inner = inner;
        this.toCoreDocumentCalled = false;
    }

    toCoreDocument(): CoreDocument {
        this.toCoreDocumentCalled = true;
        return this.inner;
    }
}

class MockInheritedIotaDocument extends IotaDocument {
    toCoreDocumentCalled: boolean;

    constructor(network: string) {
        super(network);
        this.toCoreDocumentCalled = false;
    }

    toCoreDocument(): CoreDocument {
        this.toCoreDocumentCalled = true;
        return super.toCoreDocument();
    }
}
const credentialFields = {
    context: "https://www.w3.org/2018/credentials/examples/v1",
    id: "https://example.edu/credentials/3732",
    type: "UniversityDegreeCredential",
    credentialSubject: {
        id: "did:example:ebfeb1f712ebc6f1c276e12ec21",
        degree: {
            type: "BachelorDegree",
            name: "Bachelor of Science and Arts",
        },
    },
    issuer: "https://example.edu/issuers/565049",
    issuanceDate: "2010-01-01T00:00:00Z",
    expirationDate: "2020-01-01T19:23:24Z",
    credentialStatus: {
        id: "https://example.edu/status/24",
        type: "CredentialStatusList2017",
    },
    credentialSchema: {
        id: "https://example.org/examples/degree.json",
        type: "JsonSchemaValidator2018",
    },
    refreshService: {
        id: "https://example.edu/refresh/3732",
        type: "ManualRefreshService2018",
    },
    termsOfUse: {
        type: "IssuerPolicy",
        id: "https://example.com/policies/credential/4",
        profile: "https://example.com/profiles/credential",
        prohibition: [{
            assigner: "https://example.edu/issuers/14",
            assignee: "AllVerifiers",
            target: "https://example.edu/credentials/3732",
            action: ["Archival"],
        }],
    },
    evidence: {
        id: "https://example.edu/evidence/f2aeec97-fc0d-42bf-8ca7-0548192d4231",
        type: ["DocumentVerification"],
        verifier: "https://example.edu/issuers/14",
        evidenceDocument: "DriversLicense",
        subjectPresence: "Physical",
        documentPresence: "Physical",
        licenseNumber: "123AB4567",
    },
    nonTransferable: true,
    custom1: "asdf",
    custom2: 1234,
};

describe("Credential", function() {
    describe("#new and field getters", function() {
        it("should work", async () => {
            const credential = new Credential(credentialFields);
            assert.deepStrictEqual(credential.context(), [Credential.BaseContext(), credentialFields.context]);
            assert.deepStrictEqual(credential.id(), credentialFields.id);
            assert.deepStrictEqual(credential.type(), [Credential.BaseType(), credentialFields.type]);
            assert.deepStrictEqual(credential.credentialSubject(), [credentialFields.credentialSubject]);
            assert.deepStrictEqual(credential.issuer(), credentialFields.issuer);
            assert.deepStrictEqual(credential.issuanceDate().toRFC3339(), credentialFields.issuanceDate);
            assert.deepStrictEqual(credential.expirationDate().toRFC3339(), credentialFields.expirationDate);
            assert.deepStrictEqual(credential.credentialStatus(), [credentialFields.credentialStatus]);
            assert.deepStrictEqual(credential.credentialSchema(), [credentialFields.credentialSchema]);
            assert.deepStrictEqual(credential.refreshService(), [credentialFields.refreshService]);
            assert.deepStrictEqual(credential.termsOfUse(), [credentialFields.termsOfUse]);
            assert.deepStrictEqual(credential.evidence(), [credentialFields.evidence]);
            assert.deepStrictEqual(credential.nonTransferable(), credentialFields.nonTransferable);
            const properties = new Map();
            properties.set("custom1", "asdf");
            properties.set("custom2", 1234);
            assert.deepStrictEqual(credential.properties(), properties);
            assert.deepStrictEqual(credential.proof(), undefined);
        });
    });
});

const presentationFields = {
    context: "https://www.w3.org/2018/credentials/examples/v1",
    id: "urn:uuid:3978344f-8596-4c3a-a978-8fcaba3903c5",
    type: "CredentialManagerPresentation",
    verifiableCredential: Credential.fromJSON({
        "@context": ["https://www.w3.org/2018/credentials/v1", "https://www.w3.org/2018/credentials/examples/v1"],
        id: "https://example.edu/credentials/3732",
        type: ["VerifiableCredential", "UniversityDegreeCredential"],
        credentialSubject: {
            id: "did:example:ebfeb1f712ebc6f1c276e12ec21",
            degree: {
                type: "BachelorDegree",
                name: "Bachelor of Science and Arts",
            },
        },
        issuer: "https://example.edu/issuers/565049",
        issuanceDate: "2010-01-01T00:00:00Z",
    }),
    holder: "did:example:1234",
    refreshService: {
        id: "https://example.edu/refresh/3732",
        type: "ManualRefreshService2018",
    },
    termsOfUse: {
        type: "IssuerPolicy",
        id: "https://example.com/policies/credential/4",
        profile: "https://example.com/profiles/credential",
        prohibition: [{
            assigner: "https://example.edu/issuers/14",
            assignee: "AllVerifiers",
            target: "https://example.edu/credentials/3732",
            action: ["Archival"],
        }],
    },
    custom1: "asdf",
    custom2: 1234,
};

describe("Presentation", function() {
    describe("#new and field getters", function() {
        it("should work", async () => {
            const presentation = new Presentation(presentationFields);
            assert.deepStrictEqual(presentation.context(), [Presentation.BaseContext(), presentationFields.context]);
            assert.deepStrictEqual(presentation.id(), presentationFields.id);
            assert.deepStrictEqual(presentation.type(), [Presentation.BaseType(), presentationFields.type]);
            assert.deepStrictEqual(
                presentation.verifiableCredential()[0].toJSON(),
                presentationFields.verifiableCredential.toJSON(),
            );
            assert.deepStrictEqual(presentation.holder(), presentationFields.holder);
            assert.deepStrictEqual(presentation.refreshService(), [presentationFields.refreshService]);
            assert.deepStrictEqual(presentation.termsOfUse(), [presentationFields.termsOfUse]);
            const properties = new Map();
            properties.set("custom1", "asdf");
            properties.set("custom2", 1234);
            assert.deepStrictEqual(presentation.properties(), properties);
            assert.deepStrictEqual(presentation.proof(), undefined);
        });
    });
});

// Test the duck-typed interfaces for PresentationValidator and CredentialValidator.
describe("CredentialValidator, PresentationValidator", function() {
    describe("#validate()", function() {
        it("should work", async () => {
            // Set up issuer & subject DID documents.
            const issuerDoc = new IotaDocument("iota");
            const issuerKeys = new KeyPair(KeyType.Ed25519);
            issuerDoc.insertMethod(
                new VerificationMethod(issuerDoc.id(), KeyType.Ed25519, issuerKeys.public(), "#iss-0"),
                MethodScope.VerificationMethod(),
            );

            // Add RevocationBitmap service.
            const revocationBitmap = new RevocationBitmap();
            issuerDoc.insertService(
                new Service({
                    id: issuerDoc.id().join("#my-revocation-service"),
                    type: RevocationBitmap.type(),
                    serviceEndpoint: revocationBitmap.toEndpoint(),
                }),
            );

            const subjectDoc = new IotaDocument("iota");
            const subjectKeys = new KeyPair(KeyType.Ed25519);
            subjectDoc.insertMethod(
                new VerificationMethod(subjectDoc.id().toCoreDid(), KeyType.Ed25519, subjectKeys.public(), "#sub-0"),
                MethodScope.VerificationMethod(),
            );

            const subjectDID = subjectDoc.id();
            const issuerDID = issuerDoc.id();
            const subject = {
                id: subjectDID.toString(),
                name: "Alice",
                degreeName: "Bachelor of Science and Arts",
                degreeType: "BachelorDegree",
                GPA: "4.0",
            };
            const credential = new Credential({
                id: "https://example.edu/credentials/3732",
                type: "UniversityDegreeCredential",
                issuer: issuerDID.toString(),
                credentialStatus: {
                    id: issuerDoc.id() + "#my-revocation-service",
                    type: RevocationBitmap.type(),
                    revocationBitmapIndex: "5",
                },
                credentialSubject: subject,
            });

            // Sign the credential with the issuer's DID Document.
            const signedCredential = issuerDoc.signCredential(
                credential,
                issuerKeys.private(),
                "#iss-0",
                ProofOptions.default(),
            );

            // Validate the credential.
            assert.doesNotThrow(() => CredentialValidator.checkStructure(signedCredential));
            assert.doesNotThrow(() => CredentialValidator.checkExpiresOnOrAfter(signedCredential, Timestamp.nowUTC()));
            assert.doesNotThrow(() => CredentialValidator.checkIssuedOnOrBefore(signedCredential, Timestamp.nowUTC()));
            assert.doesNotThrow(() =>
                CredentialValidator.checkSubjectHolderRelationship(
                    signedCredential,
                    subjectDID.toString(),
                    SubjectHolderRelationship.AlwaysSubject,
                )
            );
            assert.doesNotThrow(() =>
                CredentialValidator.checkStatus(signedCredential, [issuerDoc], StatusCheck.Strict)
            );
            assert.doesNotThrow(() =>
                CredentialValidator.verifySignature(
                    signedCredential,
                    [issuerDoc, subjectDoc],
                    VerifierOptions.default(),
                )
            );
            // Check that we are not leaking memory in Rust. 
            assert.deepStrictEqual(issuerDoc._strongCountInternal() as number, 1);
            assert.deepStrictEqual(subjectDoc._strongCountInternal() as number, 1);

            assert.doesNotThrow(() =>
                CredentialValidator.validate(
                    signedCredential,
                    issuerDoc,
                    CredentialValidationOptions.default(),
                    FailFast.FirstError,
                )
            );
            assert.deepStrictEqual(
                CredentialValidator.extractIssuer(signedCredential).toString(),
                issuerDID.toString(),
            );

            // Characterisation test: Check that toCoreDocument does not get called
            // when passing an extension of `CoreDocument`.
            let mockInheritedDocument = new MockInheritedCoreDocument({
                id: issuerDoc.id(),
                verificationMethod: issuerDoc.methods(MethodScope.VerificationMethod()),
            });

            assert.doesNotThrow(() =>
                CredentialValidator.verifySignature(
                    signedCredential,
                    [mockInheritedDocument, subjectDoc],
                    VerifierOptions.default(),
                )
            );

            assert.deepStrictEqual(
                mockInheritedDocument.toCoreDocumentCalled,
                false,
            );
            // Also check that we don't leak memory. 
            assert.deepStrictEqual(mockInheritedDocument._strongCountInternal() as number, 1);

            // Characterisation test: Check that toCoreDocument DOES get called
            // when passing a mere implementer of IToCoreDocument (without inheriting from CoreDocument)
            // to CredentialValidator.verifySignature.
            let mockIToCoreDocument = new MockIToCoreDocument(issuerDoc.toCoreDocument());
            assert.doesNotThrow(() =>
                CredentialValidator.verifySignature(
                    signedCredential,
                    [mockIToCoreDocument, subjectDoc],
                    VerifierOptions.default(),
                )
            );

            assert.deepStrictEqual(
                mockIToCoreDocument.toCoreDocumentCalled,
                true,
            );

            // Also check that we don't leak memory.     
            assert.deepStrictEqual(mockIToCoreDocument.inner._strongCountInternal() as number, 1);

            // Characterisation test: Check that toCoreDocument does not get called
            // when passing `IotaDocument` (we use inheritance as a way of mocking a normal IotaDocument).
            let mockIotaDoc = new MockInheritedIotaDocument("iota");
            // Make sure that DIDs are the same.
            assert.deepStrictEqual(issuerDoc.id().toString(), mockIotaDoc.id().toString());
            // Insert the same verificationMethod used by `issuerDoc` to sign the credential.
            mockIotaDoc.insertMethod(
                new VerificationMethod(issuerDoc.id().toCoreDid(), KeyType.Ed25519, issuerKeys.public(), "#iss-0"),
                MethodScope.VerificationMethod(),
            );

            assert.doesNotThrow(() =>
                CredentialValidator.verifySignature(
                    signedCredential,
                    [mockIotaDoc, subjectDoc],
                    VerifierOptions.default(),
                )
            );

            assert.deepStrictEqual(
                mockIotaDoc.toCoreDocumentCalled,
                false,
            );

            // Construct a presentation.
            const presentation = new Presentation({
                id: "https://example.org/credentials/3732",
                holder: subjectDID.toString(),
                verifiableCredential: signedCredential,
            });
            const signedPresentation = subjectDoc.signPresentation(
                presentation,
                subjectKeys.private(),
                "#sub-0",
                ProofOptions.default(),
            );

            // Validate the presentation.
            assert.doesNotThrow(() => PresentationValidator.checkStructure(signedPresentation));
            assert.doesNotThrow(() =>
                PresentationValidator.verifyPresentationSignature(
                    signedPresentation,
                    subjectDoc,
                    VerifierOptions.default(),
                )
            );
            // Check that we don't leak memory. 
            assert.deepStrictEqual(subjectDoc._strongCountInternal() as number, 1);

            assert.doesNotThrow(() =>
                PresentationValidator.validate(
                    signedPresentation,
                    subjectDoc,
                    [issuerDoc],
                    PresentationValidationOptions.default(),
                    FailFast.FirstError,
                )
            );
            // Check that we don't leak memory. 
            assert.deepStrictEqual(subjectDoc._strongCountInternal() as number, 1);
            assert.deepStrictEqual(issuerDoc._strongCountInternal() as number, 1);

            assert.deepStrictEqual(
                PresentationValidator.extractHolder(signedPresentation).toString(),
                subjectDID.toString(),
            );
        });
    });
});
