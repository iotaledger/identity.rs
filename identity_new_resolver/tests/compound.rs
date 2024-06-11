use identity_new_resolver::{CompoundResolver, Resolver, Result};

struct DidKey;
struct DidJwk;
struct DidWeb;

struct CoreDoc;

struct DidKeyResolver;
impl Resolver<DidKey> for DidKeyResolver {
  type Target = CoreDoc;
  async fn resolve(&self, _input: &DidKey) -> Result<Self::Target> {
    Ok(CoreDoc {})
  }
}
struct DidJwkResolver;
impl Resolver<DidJwk> for DidJwkResolver {
  type Target = CoreDoc;
  async fn resolve(&self, _input: &DidJwk) -> Result<Self::Target> {
    Ok(CoreDoc {})
  }
}
struct DidWebResolver;
impl Resolver<DidWeb> for DidWebResolver {
  type Target = CoreDoc;
  async fn resolve(&self, _input: &DidWeb) -> Result<Self::Target> {
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
