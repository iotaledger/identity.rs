use identity_new_resolver::{CompoundResolver, Resolver, Result, Error};

struct DidKey;
struct DidJwk;
struct DidWeb;
struct DidIota {
  network: u32,
}

struct CoreDoc;
struct IotaDoc;

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

struct Client {
  network: u32,
}

impl Resolver<DidIota> for Client {
  type Target = IotaDoc;
  async fn resolve(&self, input: &DidIota) -> Result<Self::Target> {
    if input.network == self.network {
      Ok(IotaDoc {})
    } else {
      Err(Error::Generic(anyhow::anyhow!("Invalid network")))
    }
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

#[derive(CompoundResolver)]
struct IdentityClient {
  #[resolver(DidKey -> CoreDoc, DidJwk -> CoreDoc, DidWeb -> CoreDoc)]
  dids: SuperDidResolver,
  #[resolver(DidIota -> IotaDoc if input.network == 0)]
  iota: Client,
  #[resolver(DidIota -> IotaDoc)]
  shimmer: Client,
}

#[tokio::test]
async fn test_compound_resolver_simple() {
  let super_resolver = SuperDidResolver {
    did_key: DidKeyResolver {},
    did_jwk: DidJwkResolver {},
    did_web: DidWebResolver {},
  };

  assert!(super_resolver.resolve(&DidJwk {}).await.is_ok());
}

#[tokio::test]
async fn test_compound_resolver_conflicts() {
  let super_resolver = SuperDidResolver {
    did_key: DidKeyResolver {},
    did_jwk: DidJwkResolver {},
    did_web: DidWebResolver {},
  };
  let identity_client = IdentityClient {
    dids: super_resolver,
    iota: Client { network: 0},
    shimmer: Client {network: 1},
  };
  
  assert!(identity_client.resolve(&DidJwk {}).await.is_ok());
  assert!(identity_client.resolve(&DidIota { network: 1}).await.is_ok());
  assert!(identity_client.resolve(&DidIota { network: 0}).await.is_ok());
}
