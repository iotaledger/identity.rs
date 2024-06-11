use identity_new_resolver::{CompoundResolver, Resolver, Result};

struct DidKey;
struct DidJwk;
struct DidWeb;

struct CoreDoc;

struct DidKeyResolver;
impl Resolver<DidKey, CoreDoc> for DidKeyResolver {
  async fn resolve(&self, _input: &DidKey) -> Result<CoreDoc> {
    Ok(CoreDoc {})
  }
}
struct DidJwkResolver;
impl Resolver<DidJwk, CoreDoc> for DidJwkResolver {
  async fn resolve(&self, _input: &DidJwk) -> Result<CoreDoc> {
    Ok(CoreDoc {})
  }
}
struct DidWebResolver;
impl Resolver<DidWeb, CoreDoc> for DidWebResolver {
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
