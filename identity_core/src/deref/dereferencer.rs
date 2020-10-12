use anyhow::anyhow;
use percent_encoding::percent_decode_str;

use crate::{
    common::Url,
    deref::{Dereference, DereferenceContext, DereferenceInput, PrimaryResource, SecondaryResource},
    did::{DIDDocument, Param, DID},
    error::{Error, Result},
    resolver::{ErrorKind, IdentityResolver, Resolution},
    utils::HasId as _,
};

pub struct Dereferencer<R> {
    resolver: R,
}

impl<R> Dereferencer<R>
where
    R: IdentityResolver,
{
    pub fn new(resolver: R) -> Self {
        Self { resolver }
    }

    pub async fn deref_str(&self, did: &str, input: DereferenceInput) -> Result<Dereference> where R: Sync {
        self.deref_did(&DID::parse_from_str(did)?, input).await
    }

    pub async fn deref_did(&self, did: &DID, input: DereferenceInput) -> Result<Dereference> where R: Sync {
        let mut context: DereferenceContext = DereferenceContext::new();

        self.deref_did_(did, input, &mut context).await?;

        Ok(context.finish())
    }

    async fn deref_did_(&self, did: &DID, input: DereferenceInput, context: &mut DereferenceContext) -> Result<()> where R: Sync {
        let mut resolution: Resolution = self.resolver.resolve_did(did, input.resolution).await?;

        if resolution.metadata.error.is_some() {
            context.set_resolution(resolution);
        } else if let Some(document) = resolution.did_document.take() {
            context.set_resolution(resolution);

            let primary: PrimaryResource = dereference_primary(did, document)?;

            if let Some(fragment) = did.fragment.as_deref() {
                match primary {
                    PrimaryResource::Document(inner) => {
                        if let Some(resource) = dereference_secondary(inner, fragment)? {
                            context.set_resource(resource);
                        }
                    }
                    PrimaryResource::Service(mut inner) => {
                        inner.set_fragment(Some(fragment));
                        context.set_resource(PrimaryResource::Service(inner));
                    }
                }
            } else {
                context.set_resource(primary);
            }
        } else {
            context.set_error(ErrorKind::NotFound);
            context.set_resolution(resolution);
        }

        Ok(())
    }
}

fn dereference_primary(did: &DID, document: DIDDocument) -> Result<PrimaryResource> {
    let cloned: DID = DID {
        fragment: None,
        ..did.clone()
    };

    let params: &[Param] = cloned.query.as_deref().unwrap_or_default();
    let path: &[String] = cloned.path_segments.as_deref().unwrap_or_default();

    let service: Option<&str> = params
        .iter()
        .find(|param| param.key == "service")
        .and_then(|param| param.value.as_deref());

    if let Some(param) = service {
        // Extract the service endpoint URL from the resolved DID document.
        let input: &str = document
            .services
            .iter()
            .find(|service| matches!(service.id.fragment.as_deref(), Some(fragment) if fragment == param))
            .map(|service| service.endpoint.context.as_str())
            .unwrap_or_default();

        // Perform the "Service Endpoint Construction" algorithm
        Url::parse(input)
            .map_err(Into::into)
            .and_then(|url| service_endpoint_ctor(did, params, url))
            .map(PrimaryResource::Service)
    } else if params.is_empty() && path.is_empty() {
        Ok(PrimaryResource::Document(document))
    } else {
        todo!("Method-Specific Deref")
    }
}

fn dereference_secondary(document: DIDDocument, target: &str) -> Result<Option<SecondaryResource>> {
    macro_rules! extract {
        ($base:expr, $target:expr, $iter:expr, $resource:ident) => {
            for object in $iter {
                let did: DID = DID::join_relative(&$base, object.id())?;
                let fragment: &str = did.fragment.as_deref().unwrap_or_default();

                if fragment == $target {
                    return Ok(Some(SecondaryResource::$resource(object)));
                }
            }
        };
    }

    let base: DID = document.derive_did().clone();

    extract!(base, target, document.public_keys, PublicKey);
    extract!(base, target, document.auth, Authentication);
    extract!(base, target, document.assert, Authentication);
    extract!(base, target, document.verification, Authentication);
    extract!(base, target, document.delegation, Authentication);
    extract!(base, target, document.invocation, Authentication);
    extract!(base, target, document.agreement, Authentication);
    extract!(base, target, document.services, Service);

    Ok(None)
}

// Service Endpoint Construction
//
// [Ref](https://w3c-ccg.github.io/did-resolution/#service-endpoint-construction)
fn service_endpoint_ctor(did: &DID, params: &[Param], service: Url) -> Result<Url> {
    // The input DID URL and input service endpoint URL MUST NOT both have a
    // query component.
    if did.query.is_some() && service.query().is_some() {
        return Err(Error::DereferenceError(anyhow!("Multiple DID Queries")));
    }

    // The input DID URL and input service endpoint URL MUST NOT both have a
    // fragment component.
    if did.fragment.is_some() && service.fragment().is_some() {
        return Err(Error::DereferenceError(anyhow!("Multiple DID Fragments")));
    }

    // The input service endpoint URL MUST be an HTTP(S) URL.
    if service.scheme() != "https" {
        return Err(Error::DereferenceError(anyhow!("Invalid Service Protocol")));
    }

    let segments: &[String] = did.path_segments.as_deref().unwrap_or_default();

    let relative: Option<&str> = params
        .iter()
        .find(|param| param.key == "relative-ref")
        .and_then(|param| param.value.as_deref())
        .filter(|value| !value.is_empty());

    // 1. Initialize a string output service endpoint URL to the value of
    //    the input service endpoint URL
    let mut output: Url = service.clone();

    // 2. If the output service endpoint URL has a query component, remove
    //    it.
    output.set_query(None);

    // 3. If the output service endpoint URL has a fragment component,
    //    remove it.
    output.set_fragment(None);

    if let Some(relative) = relative {
        output = output.join(&percent_decode_str(relative).decode_utf8()?)?;
    }

    // 4. Append the path component of the input DID URL to the output
    //    service endpoint URL.
    output.path_segments_mut().unwrap().extend(segments);

    // 5. If the input service endpoint URL has a query component, append ?
    //    plus the query to the output service endpoint URL.
    // 6. If the input DID URL has a query component, append ? plus the
    //    query to the output service endpoint URL.
    match (did.query.as_deref(), service.query().map(|_| service.query_pairs())) {
        (Some(params), None) => {
            let iter = strip_params(params.iter().map(Param::pair));
            output.query_pairs_mut().extend_pairs(iter);
        }
        (None, Some(query)) => {
            output.query_pairs_mut().extend_pairs(strip_params(query));
        }
        (Some(_), Some(_)) => unreachable!(),
        (None, None) => {}
    }

    // 7. If the input service endpoint URL has a fragment component, append
    //    # plus the fragment to the output service endpoint URL.
    // 8. If the input DID URL has a fragment component, append # plus the
    //    fragment to the output service endpoint URL.
    match (did.fragment.as_deref(), service.fragment()) {
        (fragment @ Some(_), None) | (None, fragment @ Some(_)) => output.set_fragment(fragment),
        (Some(_), Some(_)) => unreachable!(),
        (None, None) => {}
    }

    // 9. Return the output service endpoint URL.
    Ok(output)
}

fn strip_params<T, U>(iter: impl Iterator<Item = (T, U)>) -> impl Iterator<Item = (T, U)>
where
    T: AsRef<str>,
{
    iter.filter(|(key, _)| !DID::PARAMS.contains(&key.as_ref()))
}
