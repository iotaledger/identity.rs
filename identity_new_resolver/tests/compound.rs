use identity_new_resolver::{CompoundResolver, Result, Resolver};

struct DidKey;
struct DidJwk;
struct DidWeb;

struct CoreDoc;

struct DidKeyResolver;
impl Resolver<CoreDoc, DidKey> for DidKeyResolver {
  async fn resolve(&self, _input: &DidKey) -> Result<CoreDoc> {
    Ok(CoreDoc {})
  }
}
struct DidJwkResolver;
impl Resolver<CoreDoc, DidJwk> for DidJwkResolver {
  async fn resolve(&self, _input: &DidJwk) -> Result<CoreDoc> {
    Ok(CoreDoc {})
  }
}
struct DidWebResolver;
impl Resolver<CoreDoc, DidWeb> for DidWebResolver {
  async fn resolve(&self, _input: &DidWeb) -> Result<CoreDoc> {
    Ok(CoreDoc {})
  }
}

#[derive(CompoundResolver)]
struct SuperDidResolver {
  #[resolver(DidKey -> CoreDoc)]
  did_key: DidKeyResolver,
  #[resolver(DidJwk -> CoreDoc)]
  did_jwk: DidJwkResolver,
  #[resolver(DidWeb -> CoreDoc)]
  did_web: DidWebResolver,
}

#[tokio::test]
async fn test_compound_resolver() {
    let super_resolver = SuperDidResolver {
        did_key: DidKeyResolver {},
        did_jwk: DidJwkResolver {},
        did_web: DidWebResolver {},
    };

    assert!(super_resolver.resolve(&DidJwk {}).await.is_ok());
}
