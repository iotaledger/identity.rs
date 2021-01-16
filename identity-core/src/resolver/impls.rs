use anyhow::anyhow;
use did_doc::{url::Url, Document};
use did_url::DID;
use std::time::Instant;

use crate::{
    error::{Error, Result},
    resolver::{
        Dereference, DocumentMetadata, ErrorKind, InputMetadata, MetaDocument, PrimaryResource, Resolution,
        ResolverMethod, Resource, SecondaryResource,
    },
};

pub async fn resolve<R>(did: &str, input: InputMetadata, method: R) -> Result<Resolution>
where
    R: ResolverMethod,
{
    let mut context: ResolveContext = ResolveContext::new();

    // 1. Validate that the input DID conforms to the did rule of the DID Syntax.
    let did: DID = match did.parse() {
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

pub async fn dereference<R>(did: &str, input: InputMetadata, method: R) -> Result<Dereference>
where
    R: ResolverMethod,
{
    // Create the context immediately, for accurate durations.
    let mut context: DerefContext = DerefContext::new();

    // 1. Obtain the DID document for the input DID by executing the DID
    //    resolution algorithm.
    let resolution: Resolution = resolve(did, input, method).await?;

    // If the resolution result contains an error, bail early.
    if let Some(error) = resolution.metadata.error {
        return Ok(context.finish_error(error));
    }

    // Extract the document and metadata - Both properties MUST exist as we
    // checked for resolution errors above.
    let (document, metadata): (Document, DocumentMetadata) = resolution
        .document
        .zip(resolution.document_metadata)
        .ok_or_else(|| Error::DereferenceError(anyhow!("Missing Document/Metadata")))?;

    // Extract the parsed DID from the resolution output - It MUST exist as we
    // checked for resolution errors above.
    let did: DID = resolution
        .metadata
        .resolved
        .ok_or_else(|| Error::DereferenceError(anyhow!("Missing Resolved DID")))?;

    // Add the resolution document metadata to the response.
    context.set_metadata(metadata);

    // 2. Execute the algorithm for Dereferencing the Primary Resource.
    let primary: PrimaryResource = match dereference_primary(document, did.clone())? {
        Some(primary) => primary,
        None => return Ok(context.finish_error(ErrorKind::NotFound)),
    };

    // 3. If the original input DID URL contained a DID fragment, execute the
    //    algorithm for Dereferencing the Secondary Resource.
    if let Some(fragment) = did.fragment() {
        //
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

    fn set_document(&mut self, value: Document) {
        self.0.document = Some(value);
    }

    fn set_metadata(&mut self, value: DocumentMetadata) {
        self.0.document_metadata = Some(value);
    }

    fn set_resolved(&mut self, value: DID) {
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

fn dereference_primary(document: Document, mut did: DID) -> Result<Option<PrimaryResource>> {
    // Remove the DID fragment from the input DID URL.
    did.set_fragment(None);

    // 1. If the input DID URL contains the DID parameter service...
    if let Some((_, target)) = did.query_pairs().find(|(key, _)| key == "service") {
        // 1.1. From the resolved DID document, select the service endpoint whose
        //      id property contains a fragment which matches the value of the
        //      service DID parameter of the input DID URL.
        document
            .service()
            .iter()
            .find(|service| matches!(service.id().fragment(), Some(fragment) if fragment == target))
            .map(|service| service.service_endpoint())
            // 1.2. Execute the Service Endpoint Construction algorithm.
            .map(|url| service_endpoint_ctor(did, url))
            .transpose()?
            // 1.3. Return the output service endpoint URL.
            .map(Into::into)
            .map(Ok)
            .transpose()
    // 3. Otherwise, if the input DID URL contains no DID path and no DID query.
    } else if did.path().is_empty() && did.query().is_none() {
        // 3.1. Return the resolved DID document.
        Ok(Some(document.into()))
    } else {
        todo!("Handle Method-Specific Dereference")
    }
}

fn dereference_document(document: Document, fragment: &str) -> Result<Option<SecondaryResource>> {
    macro_rules! extract {
        ($base:expr, $target:expr, $iter:expr) => {
            for object in $iter.iter() {
                let did: DID = $base.join(object.id())?;

                if matches!(did.fragment(), Some(fragment) if fragment == $target) {
                    return Ok(Some(object.clone().into()));
                }
            }
        };
    }

    extract!(document.id(), fragment, document.verification_method());
    extract!(document.id(), fragment, document.authentication());
    extract!(document.id(), fragment, document.assertion_method());
    extract!(document.id(), fragment, document.key_agreement());
    extract!(document.id(), fragment, document.capability_delegation());
    extract!(document.id(), fragment, document.capability_invocation());
    extract!(document.id(), fragment, document.service());

    Ok(None)
}

// Service Endpoint Construction
//
// [Ref](https://w3c-ccg.github.io/did-resolution/#service-endpoint-construction)
fn service_endpoint_ctor(did: DID, url: &Url) -> Result<Url> {
    // The input DID URL and input service endpoint URL MUST NOT both have a
    // query component.
    if did.query().is_some() && url.query().is_some() {
        return Err(Error::DereferenceError(anyhow!("Multiple DID Queries")));
    }

    // The input DID URL and input service endpoint URL MUST NOT both have a
    // fragment component.
    if did.fragment().is_some() && url.fragment().is_some() {
        return Err(Error::DereferenceError(anyhow!("Multiple DID Fragments")));
    }

    // The input service endpoint URL MUST be an HTTP(S) URL.
    if url.scheme() != "https" {
        return Err(Error::DereferenceError(anyhow!("Invalid Service Protocol")));
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
        .extend(did.path().split('/'));

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
