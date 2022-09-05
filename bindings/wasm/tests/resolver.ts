import type { CoreDocument } from "../node";

export {};

const assert = require('assert');
const {
    StardustDID,
    MixedResolver,
    CoreDocument,
    Presentation, 
    StardustDocument, 
    CoreDID, 
    FailFast,
    PresentationValidationOptions
} = require("../node");

describe('Resolver', function () {
    describe('#verifyPresentation', function () {
        it('should verify presentations correctly', async () => {
            const presentationJSON = {
                "@context": "https://www.w3.org/2018/credentials/v1",
                "id": "https://example.org/credentials/3732",
                "type": "VerifiablePresentation",
                "verifiableCredential": [
                  {
                    "@context": "https://www.w3.org/2018/credentials/v1",
                    "id": "https://example.edu/credentials/3732",
                    "type": [
                      "VerifiableCredential",
                      "UniversityDegreeCredential"
                    ],
                    "credentialSubject": {
                      "id": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5",
                      "GPA": "4.0",
                      "degree": {
                        "name": "Bachelor of Science and Arts",
                        "type": "BachelorDegree"
                      },
                      "name": "Alice"
                    },
                    "issuer": "did:iota:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                    "issuanceDate": "2022-08-31T08:35:44Z",
                    "expirationDate": "2050-09-01T08:35:44Z",
                    "proof": {
                      "type": "JcsEd25519Signature2020",
                      "verificationMethod": "did:iota:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA#issuerKey",
                      "signatureValue": "3d2aAPqjzaSQ2XbFtqLsauv2Ukdn4Hcevz2grNuJn4q4JbBmDHZpAvekVG12A3ZKRRTeKaBPguxXqcDaqujckWWz"
                    }
                  },
                  {
                    "@context": "https://www.w3.org/2018/credentials/v1",
                    "id": "https://example.edu/credentials/3732",
                    "type": [
                      "VerifiableCredential",
                      "UniversityDegreeCredential"
                    ],
                    "credentialSubject": {
                      "id": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5",
                      "GPA": "4.0",
                      "degree": {
                        "name": "Bachelor of Science and Arts",
                        "type": "BachelorDegree"
                      },
                      "name": "Alice"
                    },
                    "issuer": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
                    "issuanceDate": "2022-08-31T08:35:44Z",
                    "expirationDate": "2050-09-01T08:35:44Z",
                    "proof": {
                      "type": "JcsEd25519Signature2020",
                      "verificationMethod": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr#root",
                      "signatureValue": "2iAYujqHLXP5csZzabdkfurpHaKT3Q8dnJDA4TL7pSJ7gjXLCb2tN7CF4ztKkCKmvY6VYG3pTuN1PeLGEFiQvuQr"
                    }
                  }
                ],
                "holder": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5",
                "proof": {
                  "type": "JcsEd25519Signature2020",
                  "verificationMethod": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5#root",
                  "signatureValue": "3tVeoKjftrEAQvV3MgpKgiydRHU6i8mYVRnPc6C85upo1TDBEdN94gyW1RzbgPESaZCGeDa582BxAUHVE4rVjaAd",
                  "challenge": "475a7984-1bb5-4c4c-a56f-822bccd46441"
                }
              };

            const presentation = Presentation.fromJSON(presentationJSON); 


            const resolveDidIota = async function nameOne(did_input: string) {
                const parsedDid: StardustDID = StardustDID.parse(did_input);
                const resolvedJSON = {
                    "doc": {
                      "id": "did:iota:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                      "verificationMethod": [
                        {
                          "id": "did:iota:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA#issuerKey",
                          "controller": "did:iota:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                          "type": "Ed25519VerificationKey2018",
                          "publicKeyMultibase": "zFVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z"
                        }
                      ]
                    },
                    "meta": {
                      "created": "2022-08-31T09:33:31Z",
                      "updated": "2022-08-31T09:33:31Z"
                    }
                  };
                  if (parsedDid.toString() == did_input) {
                    return StardustDocument.fromJSON(resolvedJSON);
                  }
                  
            };

            const resolveDidFoo = async function nameTwo(did_input: string) {
                const parsedDid: CoreDID = CoreDID.parse(did_input);
                const resolvedJSON = {
                    "id": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5",
                    "verificationMethod": [
                      {
                        "id": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5#root",
                        "controller": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5",
                        "type": "Ed25519VerificationKey2018",
                        "publicKeyMultibase": "z586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5"
                      }
                    ]
                  };

                  if (parsedDid.toString() == did_input) {
                    return CoreDocument.fromJSON(resolvedJSON);
                  }
                  
            };

            const resolveDidBar = async function nameThree(did_input: string) {
                const parsedDid: CoreDID = CoreDID.parse(did_input);
                const resolvedJSON = {
                    "id": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
                    "verificationMethod": [
                      {
                        "id": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr#root",
                        "controller": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
                        "type": "Ed25519VerificationKey2018",
                        "publicKeyMultibase": "zHyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr"
                      }
                    ]
                  };

                  if (parsedDid.toString() == did_input) {
                    return CoreDocument.fromJSON(resolvedJSON);
                  }
                  
            };

            let handlerMap: Map<string, (did:string) => Promise<StardustDocument | CoreDocument>> = new Map(); 
            handlerMap.set("iota", resolveDidIota);
            handlerMap.set("foo", resolveDidFoo);
            handlerMap.set("bar", resolveDidBar); 

            const resolver = new MixedResolver({
                handlers: handlerMap
            }); 

            

            const holderDoc = await resolver.resolvePresentationHolder(presentation); 
            assert!(holderDoc instanceof CoreDocument);
            
            
            const issuerDocuments = await resolver.resolvePresentationIssuers(presentation); 
            

            const verificationResultPassingHolderDoc = await resolver.verifyPresentation(
                presentation, 
                new PresentationValidationOptions({}), 
                FailFast.FirstError, 
                holderDoc);
                assert.equal(verificationResultPassingHolderDoc, undefined); 
                

            const verificationResultPassingHolderAndIssuerDocuments = await resolver.verifyPresentation(
                presentation, 
                new PresentationValidationOptions({}), 
                FailFast.FirstError, 
                holderDoc, 
                issuerDocuments);
                assert.equal(verificationResultPassingHolderAndIssuerDocuments, undefined); 
               
            
            const verificationResultPassingIssuerDocuments = await resolver.verifyPresentation(
                presentation, 
                new PresentationValidationOptions({}), 
                FailFast.FirstError, 
                undefined, 
                issuerDocuments);
                assert.equal(verificationResultPassingIssuerDocuments, undefined); 
                
            
            const verificationResultPassingNoDocuments = await resolver.verifyPresentation(
                presentation, 
                new PresentationValidationOptions({}), 
                FailFast.FirstError
                );
                assert.equal(verificationResultPassingNoDocuments, undefined); 

            
            // check that verification fails when a wrong holder is passed in 

            let expectedErrorName = "";
            assert(issuerDocuments instanceof Array); 
            console.log("error name before:", expectedErrorName); 
            try {
                console.log("entering try block"); 
                resolver.verifyPresentation(
                    presentation, 
                    new PresentationValidationOptions({}), 
                    FailFast.FirstError, 
                    issuerDocuments.at(100), 
                    undefined);
            } catch (e) {
                console.log("catch"); 
                expectedErrorName = e.name; 
                console.log(`error ${e}`); 
            }
            console.log("error name after:", expectedErrorName); 
            //assert.equal(expectedErrorName, 'ResolverError::PresentationValidationError');
            
        });
    });
});