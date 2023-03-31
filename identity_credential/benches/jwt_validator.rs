use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use identity_core::{convert::FromJson};
use identity_credential::{validator::{vc_jwt_validation::{CredentialValidator, CredentialValidationOptions}, FailFast}};
use identity_document::document::CoreDocument;


struct Input {
    jws: String,
    issuer: CoreDocument,
    fail_fast: FailFast,
    options: CredentialValidationOptions
}
fn setup() -> Input {

    let doc: &str = r#"
    {
        "id": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
        "verificationMethod": [
          {
            "id": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr#root",
            "controller": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
            "type": "Ed25519VerificationKey2018",
            "publicKeyMultibase": "zHyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr"
          },
          {
            "id": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr#9bIsJINU23yv6MA43fWPr3NUIRLbCOgm",
            "controller": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
            "type": "JWK",
            "publicKeyJwk": {
              "kty": "OKP",
              "alg": "EdDSA",
              "kid": "9bIsJINU23yv6MA43fWPr3NUIRLbCOgm",
              "crv": "Ed25519",
              "x": "W-GEwsuWJahZQjLMxs3ibp7tBxHe0nQn-6tUKJ7F8L8"
            }
          }
        ]
      }
      
    "#;

    let jws: &str = "eyJraWQiOiJkaWQ6YmFyOkh5eDYyd1BRR3l2WENvaWhacTFCcmJVakJSaDJMdU54V2lpcU1rZkF1U1pyIzliSXNKSU5VMjN5djZNQTQzZldQcjNOVUlSTGJDT2dtIiwiYWxnIjoiRWREU0EifQ.eyJpc3MiOiJkaWQ6YmFyOkh5eDYyd1BRR3l2WENvaWhacTFCcmJVakJSaDJMdU54V2lpcU1rZkF1U1pyIiwibmJmIjoxMjYyMzczODA0LCJqdGkiOiJodHRwOi8vZXhhbXBsZS5lZHUvY3JlZGVudGlhbHMvMzczMiIsInN1YiI6ImRpZDpleGFtcGxlOmViZmViMWY3MTJlYmM2ZjFjMjc2ZTEyZWMyMSIsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIlVuaXZlcnNpdHlEZWdyZWVDcmVkZW50aWFsIl0sImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImRlZ3JlZSI6eyJuYW1lIjoiQmFjaGVsb3Igb2YgU2NpZW5jZSBpbiBNZWNoYW5pY2FsIEVuZ2luZWVyaW5nIiwidHlwZSI6IkJhY2hlbG9yRGVncmVlIn19fX0.2W-pFWOFPQap3vVpQbBcSUkjtDmNgjAhlvq7IChhG2ugeFndzLwjPgxZd7uyk4ZLkzFN3gwrNVGgRfEDZ5SJAw";

    let issuer = CoreDocument::from_json(doc).unwrap();
    Input { jws: jws.to_owned(), issuer , fail_fast: FailFast::AllErrors, options: CredentialValidationOptions::default() }   
}




pub fn criterion_benchmark(c: &mut Criterion) {
    let input = setup();
    c.bench_with_input(BenchmarkId::new("credential validator bench", 1), &input, |bencher, input| {
        bencher.iter(||  {
            let validator = CredentialValidator::default();
            validator.validate(&input.jws, &input.issuer, &input.options, input.fail_fast)
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);