// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Instant;

use identity_core::common::KeyComparable;
use identity_core::common::OrderedSet;
use identity_core::common::Url;

use crate::did::CoreDID;
use crate::did::CoreDIDUrl;
use crate::document::CoreDocument;
use crate::error::Error;
use crate::error::Result;
use crate::resolution::Dereference;
use crate::resolution::DocumentMetadata;
use crate::resolution::ErrorKind;
use crate::resolution::InputMetadata;
use crate::resolution::MetaDocument;
use crate::resolution::PrimaryResource;
use crate::resolution::Resolution;
use crate::resolution::ResolverMethod;
use crate::resolution::Resource;
use crate::resolution::SecondaryResource;
use crate::service::ServiceEndpoint;

/// Resolves a DID into a DID Document by using the "Read" operation of the DID method.
///
/// See [DID Resolution][SPEC] for more information.
///
/// [SPEC]: https://www.w3.org/TR/did-core/#did-resolution
pub async fn resolve<R>(did: impl AsRef<str>, input: InputMetadata, method: R) -> Result<Resolution>
where
  R: ResolverMethod,
{
  let mut context: ResolveContext = ResolveContext::new();

  // 1. Validate that the input DID conforms to the did rule of the DID Syntax.
  let did: CoreDID = match did.as_ref().parse() {
    Ok(did) => did,
    Err(_) => return Ok(context.finish_error(ErrorKind::InvalidDID)),
  };

  // 2. Determine if the input DID method is supported by the DID resolver
  //    that implements this algorithm.
  if !method.is_supported(&did) {
    return Ok(context.finish_error(ErrorKind::NotSupported));
  }

  // 3. Obtain the DID document for the input DID by executing the Read
  //    operation against the input DID's verifiable data registry.
  let doc: MetaDocument = match method.read(&did, input).await? {
    Some(doc) => doc,
    None => return Ok(context.finish_error(ErrorKind::NotFound)),
  };

  // 4. Validate that the output DID document conforms to a conformant
  //    serialization of the DID document data model.
  // if did.method() != doc.data.id.method() || did.method_id() != doc.data.id.method_id() {
  //   return Ok(context.finish_error(ErrorKind::InvalidDID));
  // }

  // TODO: Handle deactivated DIDs
  // TODO: Handle signature verification

  context.set_document(doc.data);
  context.set_metadata(doc.meta);
  context.set_resolved(did);

  Ok(context.finish())
}

/// Dereferences a [`DIDUrl`] into a primary or secondary resource.
///
/// See [DID Url Dereferencing][SPEC] for more information.
///
/// [SPEC]: https://www.w3.org/TR/did-core/#did-url-dereferencing
pub async fn dereference<R>(did_url: impl AsRef<str>, input: InputMetadata, method: R) -> Result<Dereference>
where
  R: ResolverMethod,
{
  // Create the context immediately, for accurate durations.
  let mut context: DerefContext = DerefContext::new();

  // 1. Obtain the DID document for the input DID by executing the DID
  //    resolution algorithm.
  let did_url: CoreDIDUrl = match did_url.as_ref().parse() {
    Ok(did) => did,
    Err(_) => return Ok(context.finish_error(ErrorKind::InvalidDID)),
  };
  let resolution: Resolution = resolve(did_url.did(), input, method).await?;

  // If the resolution result contains an error, bail early.
  if let Some(error) = resolution.metadata.error {
    return Ok(context.finish_error(error));
  }

  // Extract the document and metadata - Both properties MUST exist as we
  // checked for resolution errors above.
  let (document, metadata): (CoreDocument, DocumentMetadata) = match (resolution.document, resolution.document_metadata)
  {
    (Some(document), Some(metadata)) => (document, metadata),
    (Some(_), None) => return Err(Error::MissingResolutionMetadata),
    (None, Some(_)) => return Err(Error::MissingResolutionDocument),
    (None, None) => return Err(Error::MissingResolutionData),
  };

  // Extract the parsed DID from the resolution output - It MUST exist as we
  // checked for resolution errors above.
  let _ = resolution.metadata.resolved.ok_or(Error::MissingResolutionDID)?;

  // Add the resolution document metadata to the response.
  context.set_metadata(metadata);

  // 2. Execute the algorithm for Dereferencing the Primary Resource.
  let primary: PrimaryResource = match dereference_primary(document, did_url.clone())? {
    Some(primary) => primary,
    None => return Ok(context.finish_error(ErrorKind::NotFound)),
  };

  // 3. If the original input DID URL contained a fragment, execute the
  //    algorithm for Dereferencing the Secondary Resource.
  if let Some(fragment) = did_url.fragment() {
    // Dereferencing the Secondary Resource
    //
    match primary {
      // 1. If the result is a resolved DID document.
      PrimaryResource::Document(inner) => {
        // 1.1 From the resolved DID document, select the JSON object whose id
        //     property matches the input DID URL.
        if let Some(resource) = dereference_document(inner, fragment)? {
          // 1.2. Return the output resource.
          context.set_content(resource);
        }
      }
      // 2. Otherwise, if the result is an output service endpoint URL.
      PrimaryResource::Service(mut inner) => {
        // 2.1. Append the DID fragment to the output service endpoint URL.
        inner.set_fragment(Some(fragment));

        // 2.2. Return the output service endpoint URL.
        context.set_content(PrimaryResource::Service(inner));
      }
    }
  } else {
    context.set_content(primary);
  }

  Ok(context.finish())
}

#[derive(Debug)]
struct ResolveContext(Resolution, Instant);

impl ResolveContext {
  fn new() -> Self {
    Self(Resolution::new(), Instant::now())
  }

  fn set_document(&mut self, value: CoreDocument) {
    self.0.document = Some(value);
  }

  fn set_metadata(&mut self, value: DocumentMetadata) {
    self.0.document_metadata = Some(value);
  }

  fn set_resolved(&mut self, value: CoreDID) {
    self.0.metadata.resolved = Some(value);
  }

  fn set_error(&mut self, value: ErrorKind) {
    self.0.metadata.error = Some(value);
  }

  fn finish_error(mut self, value: ErrorKind) -> Resolution {
    self.set_error(value);
    self.finish()
  }

  fn finish(mut self) -> Resolution {
    self.0.metadata.duration = self.1.elapsed();
    self.0
  }
}

#[derive(Debug)]
struct DerefContext(Dereference, Instant);

impl DerefContext {
  fn new() -> Self {
    Self(Dereference::new(), Instant::now())
  }

  fn set_content(&mut self, value: impl Into<Resource>) {
    self.0.content = Some(value.into());
  }

  fn set_metadata(&mut self, value: DocumentMetadata) {
    self.0.content_metadata = Some(value);
  }

  fn set_error(&mut self, value: ErrorKind) {
    self.0.metadata.error = Some(value);
  }

  fn finish_error(mut self, value: ErrorKind) -> Dereference {
    self.set_error(value);
    self.finish()
  }

  fn finish(mut self) -> Dereference {
    self.0.metadata.duration = self.1.elapsed();
    self.0
  }
}

fn dereference_primary(document: CoreDocument, mut did_url: CoreDIDUrl) -> Result<Option<PrimaryResource>> {
  // Remove the DID fragment from the input DID URL.
  let _ = did_url.set_fragment(None).expect("clearing the fragment is infallible");

  // 1. If the input DID URL contains the DID parameter service...
  if let Some((_, target)) = did_url.query_pairs().find(|(key, _)| key == "service") {
    // 1.1. From the resolved DID document, select the service endpoint whose
    //      id property contains a fragment which matches the value of the
    //      service DID parameter of the input DID URL.
    document
      .service()
      .iter()
      .find(|service| matches!(service.id().fragment(), Some(fragment) if fragment == target))
      .map(|service| service.service_endpoint())
      // 1.2. Execute the Service Endpoint Construction algorithm.
      .map(|endpoint| match endpoint {
        ServiceEndpoint::One(url) => service_endpoint_ctor(did_url, url),
        // TODO: support service endpoint sets and map? Dereferencing spec does not address them.
        ServiceEndpoint::Set(_) => Err(Error::InvalidResolutionService),
        ServiceEndpoint::Map(_) => Err(Error::InvalidResolutionService),
      })
      .transpose()?
      // 1.3. Return the output service endpoint URL.
      .map(Into::into)
      .map(Ok)
      .transpose()
    // 3. Otherwise, if the input DID URL contains no DID path and no DID query.
  } else if did_url.path().unwrap_or_default().is_empty() && did_url.query().is_none() {
    // 3.1. Return the resolved DID document.
    Ok(Some(document.into()))
  } else {
    todo!("Handle Method-Specific Dereference")
  }
}

fn dereference_document(document: CoreDocument, fragment: &str) -> Result<Option<SecondaryResource>> {
  #[inline]
  fn dereference<T>(
    base_did: &CoreDID,
    target_fragment: &str,
    resources: &OrderedSet<T>,
  ) -> Result<Option<SecondaryResource>>
  where
    T: Clone + AsRef<CoreDIDUrl> + KeyComparable + Into<SecondaryResource>,
  {
    for resource in resources.iter() {
      let resource_url: &CoreDIDUrl = resource.as_ref();

      // Skip objects with different base URLs
      if resource_url.did() != base_did {
        continue;
      }

      if matches!(resource_url.fragment(), Some(fragment) if fragment == target_fragment) {
        return Ok(Some(resource.clone().into()));
      }
    }

    Ok(None)
  }

  let base_did: CoreDID = document.id().clone();

  if let Some(resource) = dereference(&base_did, fragment, document.verification_method())? {
    return Ok(Some(resource));
  }

  if let Some(resource) = dereference(&base_did, fragment, document.authentication())? {
    return Ok(Some(resource));
  }

  if let Some(resource) = dereference(&base_did, fragment, document.assertion_method())? {
    return Ok(Some(resource));
  }

  if let Some(resource) = dereference(&base_did, fragment, document.key_agreement())? {
    return Ok(Some(resource));
  }

  if let Some(resource) = dereference(&base_did, fragment, document.capability_delegation())? {
    return Ok(Some(resource));
  }

  if let Some(resource) = dereference(&base_did, fragment, document.capability_invocation())? {
    return Ok(Some(resource));
  }

  if let Some(resource) = dereference(&base_did, fragment, document.service())? {
    return Ok(Some(resource));
  }

  Ok(None)
}

// Service Endpoint Construction
//
// [Ref](https://w3c-ccg.github.io/did-resolution/#service-endpoint-construction)
fn service_endpoint_ctor(did: CoreDIDUrl, url: &Url) -> Result<Url> {
  // The input DID URL and input service endpoint URL MUST NOT both have a
  // query component.
  if did.query().is_some() && url.query().is_some() {
    return Err(Error::InvalidDIDQuery);
  }

  // The input DID URL and input service endpoint URL MUST NOT both have a
  // fragment component.
  if did.fragment().is_some() && url.fragment().is_some() {
    return Err(Error::InvalidDIDFragment);
  }

  // The input service endpoint URL MUST be an HTTP(S) URL.
  if url.scheme() != "https" {
    return Err(Error::InvalidResolutionService);
  }

  // 1. Initialize a string output service endpoint URL to the value of
  //    the input service endpoint URL.
  let mut output: Url = url.clone();

  // 2. If the output service endpoint URL has a query component, remove it.
  output.set_query(None);

  // 3. If the output service endpoint URL has a fragment component, remove it.
  output.set_fragment(None);

  // Decode and join the `relative-ref` query param, if it exists.
  if let Some((_, relative)) = did.query_pairs().find(|(key, _)| key == "relative-ref") {
    output = output.join(&relative)?;
  }

  // 4. Append the path component of the input DID URL to the output
  //    service endpoint URL.
  output
    .path_segments_mut()
    .unwrap()
    .pop_if_empty()
    .extend(did.path().unwrap_or_default().split('/'));

  // 5. If the input service endpoint URL has a query component, append ?
  //    plus the query to the output service endpoint URL.
  // 6. If the input DID URL has a query component, append ? plus the
  //    query to the output service endpoint URL.
  match (did.query(), url.query()) {
    (Some(_), None) => {
      output.query_pairs_mut().extend_pairs(did.query_pairs());
    }
    (None, Some(_)) => {
      output.query_pairs_mut().extend_pairs(url.query_pairs());
    }
    (Some(_), Some(_)) => unreachable!(),
    (None, None) => {}
  }

  // 7. If the input service endpoint URL has a fragment component, append
  //    # plus the fragment to the output service endpoint URL.
  // 8. If the input DID URL has a fragment component, append # plus the
  //    fragment to the output service endpoint URL.
  match (did.fragment(), url.fragment()) {
    (fragment @ Some(_), None) | (None, fragment @ Some(_)) => output.set_fragment(fragment),
    (Some(_), Some(_)) => unreachable!(),
    (None, None) => {}
  }

  // 9. Return the output service endpoint URL.
  Ok(output)
}

#[cfg(test)]
mod test {
  use crate::did::DID;
  use crate::service::Service;
  use crate::utils::Queryable;
  use crate::verification::MethodData;
  use crate::verification::MethodType;
  use crate::verification::VerificationMethod;

  use super::*;

  fn did() -> CoreDIDUrl {
    "did:test:1234".parse().unwrap()
  }

  #[test]
  fn test_service_endpoint_valid() {
    let did = did();
    assert!(service_endpoint_ctor(did, &Url::parse("https://my-service.endpoint.net").unwrap()).is_ok());
  }

  #[test]
  fn test_service_endpoint_invalid_query() {
    let did_url = did();
    assert!(matches!(
      service_endpoint_ctor(
        did_url.clone().join("?query=this").unwrap(),
        &Url::parse("https://my-service.endpoint.net?query=this").unwrap()
      ),
      Err(Error::InvalidDIDQuery)
    ));

    assert!(service_endpoint_ctor(
      did_url.clone().join("?query=this").unwrap(),
      &Url::parse("https://my-service.endpoint.net").unwrap(),
    )
    .is_ok());
    assert!(service_endpoint_ctor(
      did_url,
      &Url::parse("https://my-service.endpoint.net?query=this").unwrap(),
    )
    .is_ok());
  }

  #[test]
  fn test_service_endpoint_invalid_fragment() {
    let did_url = did();
    assert!(matches!(
      service_endpoint_ctor(
        did_url.clone().join("#fragment").unwrap(),
        &Url::parse("https://my-service.endpoint.net#fragment").unwrap()
      ),
      Err(Error::InvalidDIDFragment)
    ));

    assert!(service_endpoint_ctor(
      did_url.clone().join("#fragment").unwrap(),
      &Url::parse("https://my-service.endpoint.net").unwrap(),
    )
    .is_ok());
    assert!(service_endpoint_ctor(
      did_url,
      &Url::parse("https://my-service.endpoint.net#fragment").unwrap(),
    )
    .is_ok());
  }

  fn generate_method(did: &CoreDID, fragment: &str) -> VerificationMethod {
    VerificationMethod::builder(Default::default())
      .id(did.to_url().join(fragment).unwrap())
      .controller(did.clone())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::new_base58(fragment.as_bytes()))
      .build()
      .unwrap()
  }

  fn generate_service(did: &CoreDID, fragment: &str, url: &str) -> Service {
    Service::builder(Default::default())
      .id(did.to_url().join(fragment).unwrap())
      .service_endpoint(Url::parse(url).unwrap().into())
      .type_("LinkedDomains")
      .build()
      .unwrap()
  }

  fn generate_document() -> CoreDocument {
    let did = CoreDID::parse("did:example:1234").unwrap();
    CoreDocument::builder(Default::default())
      .id(did.clone())
      .authentication(generate_method(&did, "#auth-key"))
      .verification_method(generate_method(&did, "#key-1"))
      .verification_method(generate_method(&did, "#key-2"))
      .service(generate_service(&did, "#service-1", "https://127.0.0.1"))
      .service(generate_service(&did, "#service-2", "https://example.com"))
      .build()
      .unwrap()
  }

  #[rustfmt::skip]
  #[test]
  fn test_dereference_primary() {
    let document = generate_document();
    let did = document.id();

    // Dereference document
    match dereference_primary(document.clone(), did.to_url()).unwrap().unwrap() {
      PrimaryResource::Document(actual) => assert_eq!(actual, document),
      actual => panic!("Expected document got {:?}", actual),
    }

    // TODO: test dereference services
    // match dereference_primary(document.clone(), did.to_url().join("?service=service-1").unwrap()).unwrap().unwrap() {
    //   PrimaryResource::Service(actual) => assert_eq!(actual, document.service().query("#service-1").unwrap().service_endpoint().clone()),
    //   actual => panic!("Expected #service-1 got {:?}", actual),
    // }
    // match dereference_primary(document.clone(), did.to_url().join("?service=service-2").unwrap()).unwrap().unwrap() {
    //   PrimaryResource::Service(actual) => assert_eq!(actual, document.service().query("#service-2").unwrap().service_endpoint().clone()),
    //   actual => panic!("Expected #service-2 got {:?}", actual),
    // }
  }

  #[rustfmt::skip]
  #[test]
  fn test_dereference_document() {
    let document = generate_document();

    // Dereference methods
    match dereference_document(document.clone(), "auth-key").unwrap().unwrap() {
      SecondaryResource::VerificationKey(actual) => assert_eq!(actual, document.methods().find(|method| method.id().fragment().unwrap() == "auth-key").unwrap().clone()),
      actual => panic!("Expected #auth-key got {:?}", actual),
    }
    match dereference_document(document.clone(), "key-1").unwrap().unwrap() {
      SecondaryResource::VerificationKey(actual) => assert_eq!(actual, document.verification_method().query("#key-1").unwrap().clone()),
      actual => panic!("Expected #key-1 got {:?}", actual),
    }
    match dereference_document(document.clone(), "key-2").unwrap().unwrap() {
      SecondaryResource::VerificationKey(actual) => assert_eq!(actual, document.verification_method().query("#key-2").unwrap().clone()),
      actual => panic!("Expected #key-2 got {:?}", actual),
    }

    // Dereference services
    match dereference_document(document.clone(), "service-1").unwrap().unwrap() {
      SecondaryResource::Service(actual) => assert_eq!(actual, document.service().query("#service-1").unwrap().clone()),
      actual => panic!("Expected #service-1 got {:?}", actual),
    }
    match dereference_document(document.clone(), "service-2").unwrap().unwrap() {
      SecondaryResource::Service(actual) => assert_eq!(actual, document.service().query("#service-2").unwrap().clone()),
      actual => panic!("Expected #service-2 got {:?}", actual),
    }
  }
}
