use crate::error::Result;

use identity_core::common::Object;
use identity_core::common::OrderedSet;
use identity_core::common::Url;

use crate::Error;
use identity_did::did::CoreDID;
use identity_did::did::DIDUrl;
use identity_did::did::DID;
use identity_did::service::Service;
use identity_did::service::ServiceEndpoint;
use indexmap::map::IndexMap;

use crate::Error::DomainLinkageError;

pub struct LinkedDomainService<D = CoreDID, T = Object>
where
  D: DID,
{
  service: Service<D, T>,
}

impl<D, T> TryFrom<Service<D, T>> for LinkedDomainService<D, T>
where
  D: DID,
{
  type Error = Error;

  fn try_from(service: Service<D, T>) -> std::result::Result<Self, Self::Error> {
    LinkedDomainService::check_structure(&service)?;
    Ok(LinkedDomainService { service })
  }
}

impl<D, T> From<LinkedDomainService<D, T>> for Service<D, T>
where
  D: DID,
{
  fn from(service: LinkedDomainService<D, T>) -> Self {
    service.service
  }
}

impl<D, T> LinkedDomainService<D, T>
where
  D: DID,
{
  pub(crate) fn domain_linkage_service_type() -> &'static str {
    "LinkedDomains"
  }

  /// Convenient function to create a spec compliant [Linked Domain Service Endpoint](https://identity.foundation/.well-known/resources/did-configuration/#linked-domain-service-endpoint)
  /// Domain URLs must include the `https` scheme in order to pass the domain linkage validation.
  pub fn new(did_url: DIDUrl<D>, domains: impl Into<OrderedSet<Url>>, properties: T) -> Result<Self> {
    let domains: OrderedSet<Url> = domains.into();
    for domain in domains.iter() {
      if domain.scheme() != "https" {
        return Err(DomainLinkageError("domain does not include `https` scheme".into()));
      }
    }
    if domains.len() == 1 {
      return Ok(Self {
        service: Service::builder(properties)
          .id(did_url)
          .type_(Self::domain_linkage_service_type())
          .service_endpoint(ServiceEndpoint::One(domains.head().unwrap().clone()))
          .build()
          .map_err(|err| DomainLinkageError(Box::new(err)))?,
      });
    }
    let mut map: IndexMap<String, OrderedSet<Url>> = IndexMap::new();
    map.insert("origins".to_owned(), domains);
    let service = Service::builder(properties)
      .id(did_url)
      .type_(Self::domain_linkage_service_type())
      .service_endpoint(ServiceEndpoint::Map(map))
      .build()
      .map_err(|err| DomainLinkageError(Box::new(err)))?;
    Ok(Self { service })
  }

  pub fn into_service(self) -> Service<D, T>
  where
    D: DID,
  {
    self.service
  }

  /// Checks the semantic structure of a Linked Domain Service.
  ///
  /// Note: `{"type": ["LinkedDomains"]}` might be serialized the same way as  `{"type": "LinkedDomains"}`
  /// which passes the semantic check.
  pub fn check_structure(service: &Service<D, T>) -> Result<()> {
    if service.type_().len() != 1 {
      return Err(DomainLinkageError("invalid service type".into()));
    }

    let service_type = service
      .type_()
      .get(0)
      .ok_or(DomainLinkageError("invalid service type".into()))?;

    if !service_type.eq(Self::domain_linkage_service_type()) {
      return Err(DomainLinkageError("invalid service type".into()));
    }

    match service.service_endpoint() {
      ServiceEndpoint::One(endpoint) => {
        if endpoint.scheme() != "https" {
          Err(DomainLinkageError("domain does not include `https` scheme".into()))?;
        }
        Ok(())
      }
      ServiceEndpoint::Set(_) => Err(DomainLinkageError(
        "service endpoints must be either a string or an object containing an `origins` property".into(),
      )),
      ServiceEndpoint::Map(endpoint) => {
        if endpoint.is_empty() {
          return Err(DomainLinkageError("invalid service endpoint object".into()));
        }
        let origins: &OrderedSet<Url> = endpoint
          .get(&"origins".to_owned())
          .ok_or(DomainLinkageError("invalid service endpoint object".into()))?;

        for origin in origins.iter() {
          if origin.scheme() != "https" {
            return Err(DomainLinkageError("domain does not include `https` scheme".into()));
          }
        }
        Ok(())
      }
    }
  }

  /// Returns the domains contained in the Linked Domain Service.
  pub fn domains(&self) -> Vec<Url> {
    match self.service.service_endpoint() {
      ServiceEndpoint::One(endpoint) => {
        vec![endpoint.clone()]
      }
      ServiceEndpoint::Set(_) => Vec::new(),
      ServiceEndpoint::Map(endpoint) => {
        if let Some(origins) = endpoint.get(&"origins".to_owned()) {
          return origins.clone().to_vec();
        }
        Vec::new()
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::credential::linked_domain_service::LinkedDomainService;
  use identity_core::common::Object;
  use identity_core::common::OrderedSet;
  use identity_core::common::Url;
  use identity_core::convert::FromJson;
  use identity_did::did::CoreDID;
  use identity_did::did::CoreDIDUrl;
  use identity_did::service::Service;
  use serde_json::json;

  #[test]
  fn create_service_multiple_origins() {
    let domain_1 = "https://foo.example-1.com";
    let domain_2 = "https://bar.example-2.com";
    let mut domains = OrderedSet::new();
    domains.append(Url::parse(domain_1).unwrap());
    domains.append(Url::parse(domain_2).unwrap());

    let service: LinkedDomainService = LinkedDomainService::new(
      CoreDIDUrl::parse("did:example:123#foo").unwrap(),
      domains,
      Object::new(),
    )
    .unwrap();

    let service_from_json: Service<CoreDID, Object> = Service::from_json_value(json!({
        "id":"did:example:123#foo",
        "type": "LinkedDomains",
        "serviceEndpoint": {
          "origins": [domain_1, domain_2]
        }
    }))
    .unwrap();
    assert_eq!(service.into_service(), service_from_json);
  }
  //
  // #[test]
  // fn create_service_single_origin() {
  //   let mut domains = OrderedSet::new();
  //   domains.append(Url::parse("https://foo.example-1.com").unwrap());
  //
  //   let service: Service<CoreDID, Object> =
  //     LinkedDomainService::create_linked_domain_service(CoreDIDUrl::parse("did:example:123#foo").unwrap(), domains)
  //       .unwrap();
  //
  //   // Add a new Service.
  //   let service_from_json: Service<CoreDID, Object> = Service::from_json_value(json!({
  //       "id":"did:example:123#foo",
  //       "type": "LinkedDomains",
  //       "serviceEndpoint": "https://foo.example-1.com"
  //   }))
  //     .unwrap();
  //   assert_eq!(service, service_from_json);
  // }
  //
  // #[test]
  // fn extract_domains() {
  //   let service_1: Service<CoreDID, Object> = Service::from_json_value(json!({
  //       "id":"did:example:123#foo",
  //       "type": "LinkedDomains",
  //       "serviceEndpoint": "https://foo.example-1.com"
  //   }))
  //     .unwrap();
  //   let domain: Vec<Url> = vec![Url::parse("https://foo.example-1.com").unwrap()];
  //   assert_eq!(LinkedDomainService::domains(&service_1).unwrap(), domain);
  //
  //   let service_2: Service<CoreDID, Object> = Service::from_json_value(json!({
  //       "id":"did:example:123#foo",
  //       "type": "LinkedDomains",
  //       "serviceEndpoint": { "origins" : ["https://foo.example-1.com", "https://foo.example-2.com"]}
  //   }))
  //     .unwrap();
  //   let domains: Vec<Url> = vec![
  //     Url::parse("https://foo.example-1.com").unwrap(),
  //     Url::parse("https://foo.example-2.com").unwrap(),
  //   ];
  //   assert_eq!(LinkedDomainService::domains(&service_2).unwrap(), domains);
  // }
  //
  // #[test]
  // fn extract_domains_invalid_scheme() {
  //   // http scheme instead of https.
  //   let service_1: Service<CoreDID, Object> = Service::from_json_value(json!({
  //       "id":"did:example:123#foo",
  //       "type": "LinkedDomains",
  //       "serviceEndpoint": "http://foo.example-1.com"
  //   }))
  //     .unwrap();
  //   assert!(LinkedDomainService::domains(&service_1).is_err());
  //
  //   let service_2: Service<CoreDID, Object> = Service::from_json_value(json!({
  //       "id":"did:example:123#foo",
  //       "type": "LinkedDomains",
  //       "serviceEndpoint": { "origins" : ["https://foo.example-1.com", "http://foo.example-2.com"]}
  //   }))
  //     .unwrap();
  //   assert!(LinkedDomainService::domains(&service_2).is_err());
  // }
  //
  // #[test]
  // fn extract_domain_type_check() {
  //   // Valid type.
  //   let service_1: Service<CoreDID, Object> = Service::from_json_value(json!({
  //       "id":"did:example:123#foo",
  //       "type": "LinkedDomains",
  //       "serviceEndpoint": "https://foo.example-1.com"
  //   }))
  //     .unwrap();
  //   assert!(LinkedDomainService::domains(&service_1).is_ok());
  //
  //   // Invalid type.
  //   let service_2: Service<CoreDID, Object> = Service::from_json_value(json!({
  //       "id":"did:example:123#foo",
  //       "type": "LinkedDomain",
  //       "serviceEndpoint": "https://foo.example-1.com"
  //   }))
  //     .unwrap();
  //   assert!(LinkedDomainService::domains(&service_2).is_err());
  // }
}
