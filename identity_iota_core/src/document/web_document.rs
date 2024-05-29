use core::fmt;
use core::fmt::Debug;
use core::fmt::Display;
use identity_credential::credential::Jws;
use identity_did::CoreDID;
use identity_did::DIDUrl;
use identity_document::verifiable::JwsVerificationOptions;
use identity_verification::jose::jws::DecodedJws;
use identity_verification::jose::jws::JwsVerifier;
use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Object;
use identity_core::common::OneOrSet;
use identity_core::common::OrderedSet;
use identity_core::common::Url;
use identity_core::convert::FmtJson;
use identity_document::document::CoreDocument;
use identity_document::service::Service;
use identity_document::utils::DIDUrlQuery;
use identity_verification::MethodRelationship;
use identity_verification::MethodScope;
use identity_verification::VerificationMethod;

use crate::error::Result;
use crate::Error;
use crate::IotaDID;
use crate::IotaDocumentMetadata;
use crate::NetworkName;
use crate::StateMetadataDocument;
use crate::StateMetadataEncoding;
use crate::WebDID;





/// A DID Document adhering to the Web DID method specification.
///
/// This extends [`CoreDocument`].
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct WebDocument(CoreDocument);

impl WebDocument {
  // ===========================================================================
  // Constructors
  // ===========================================================================

  /// Constructs an empty DID Document with a [`WebDID`] identifier.
  pub fn new(url: &str) -> Result<Self, Error> {
    Ok(Self::new_with_id(WebDID::new(url).map_err(|e| Error::DIDSyntaxError(e))?))
  }

  /// Constructs an empty DID Document with the given identifier.
  pub fn new_with_id(id: WebDID) -> Self {
    // PANIC: constructing an empty DID Document is infallible, caught by tests otherwise.
    let document: CoreDocument = CoreDocument::builder(Object::default())
      .id(id.into())
      .build()
      .expect("empty IotaDocument constructor failed");
    let metadata: IotaDocumentMetadata = IotaDocumentMetadata::new();
    Self(document)
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns the DID document identifier.
  pub fn id(&self) -> &WebDID {
    // CORRECTNESS: This cast is OK because the public API does not expose methods
    // enabling unchecked mutation of the `id` field.
    WebDID::from_inner_ref_unchecked(self.0.id())
  }

  /// Returns an iterator yielding the DID controllers.
  pub fn controller(&self) -> impl Iterator<Item = &WebDID> + '_ {
    let core_did_controller_iter = self
      .0
      .controller()
      .map(|controllers| controllers.iter())
      .into_iter()
      .flatten();

    // CORRECTNESS: These casts are OK because the public API only allows setting WebDIDs.
    core_did_controller_iter.map(WebDID::from_inner_ref_unchecked)
  }

  /// Sets the value of the document controller.
  ///
  /// Note:
  /// * Duplicates in `controller` will be ignored.
  /// * Use an empty collection to clear all controllers.
  pub fn set_controller<T>(&mut self, controller: T)
  where
    T: IntoIterator<Item = WebDID>,
  {
    let controller_core_dids: Option<OneOrSet<CoreDID>> = {
      let controller_set: OrderedSet<CoreDID> = controller.into_iter().map(CoreDID::from).collect();
      if controller_set.is_empty() {
        None
      } else {
        Some(OneOrSet::new_set(controller_set).expect("controller is checked to be not empty"))
      }
    };

    *self.0.controller_mut() = controller_core_dids;
  }

  /// Returns a reference to the `alsoKnownAs` set.
  pub fn also_known_as(&self) -> &OrderedSet<Url> {
    self.0.also_known_as()
  }

  /// Returns a mutable reference to the `alsoKnownAs` set.
  pub fn also_known_as_mut(&mut self) -> &mut OrderedSet<Url> {
    self.0.also_known_as_mut()
  }

  /// Returns a reference to the underlying [`CoreDocument`].
  pub fn core_document(&self) -> &CoreDocument {
    &self.0
  }

  /// Returns a mutable reference to the underlying [`CoreDocument`].
  ///
  /// WARNING: Mutating the inner document directly bypasses checks and
  /// may have undesired consequences.
  pub(crate) fn core_document_mut(&mut self) -> &mut CoreDocument {
    &mut self.0
  }

  /// Returns a reference to the custom DID Document properties.
  pub fn properties(&self) -> &Object {
    self.0.properties()
  }

  /// Returns a mutable reference to the custom DID Document properties.
  ///
  /// # Warning
  ///
  /// The properties returned are not checked against the standard fields in a [`CoreDocument`]. Incautious use can have
  /// undesired consequences such as key collision when attempting to serialize the document or distinct resources (such
  /// as services and methods) being identified by the same DID URL.  
  pub fn properties_mut_unchecked(&mut self) -> &mut Object {
    self.0.properties_mut_unchecked()
  }

  // ===========================================================================
  // Services
  // ===========================================================================

  /// Return a set of all [`Service`]s in the document.
  pub fn service(&self) -> &OrderedSet<Service> {
    self.0.service()
  }

  /// Add a new [`Service`] to the document.
  ///
  /// # Errors
  /// An error is returned if there already exists a service or (verification) method with
  /// the same identifier in the document.  
  pub fn insert_service(&mut self, service: Service) -> Result<()> {
    self
      .core_document_mut()
      .insert_service(service)
      .map_err(Error::InvalidDoc)
  }

  /// Remove and return the [`Service`] identified by the given [`DIDUrl`] from the document.
  ///
  /// `None` is returned if the service does not exist in the document.
  pub fn remove_service(&mut self, did_url: &DIDUrl) -> Option<Service> {
    self.core_document_mut().remove_service(did_url)
  }

  // ===========================================================================
  // Verification Methods
  // ===========================================================================

  /// Returns a `Vec` of verification method references whose verification relationship matches `scope`.
  ///
  /// If `scope` is `None`, all **embedded** methods are returned.
  pub fn methods(&self, scope: Option<MethodScope>) -> Vec<&VerificationMethod> {
    self.0.methods(scope)
  }

  /// Adds a new [`VerificationMethod`] to the document in the given [`MethodScope`].
  ///
  /// # Errors
  ///
  /// Returns an error if a method with the same fragment already exists.
  pub fn insert_method(&mut self, method: VerificationMethod, scope: MethodScope) -> Result<()> {
    self
      .core_document_mut()
      .insert_method(method, scope)
      .map_err(Error::InvalidDoc)
  }

  /// Removes and returns the [`VerificationMethod`] identified by `did_url` from the document.
  ///
  /// # Note
  ///
  /// All _references to the method_ found in the document will be removed.
  /// This includes cases where the reference is to a method contained in another DID document.
  pub fn remove_method(&mut self, did_url: &DIDUrl) -> Option<VerificationMethod> {
    self.core_document_mut().remove_method(did_url)
  }

  /// Removes and returns the [`VerificationMethod`] from the document. The [`MethodScope`] under which the method was
  /// found is appended to the second position of the returned tuple.
  ///
  /// # Note
  ///
  /// All _references to the method_ found in the document will be removed.
  /// This includes cases where the reference is to a method contained in another DID document.
  pub fn remove_method_and_scope(&mut self, did_url: &DIDUrl) -> Option<(VerificationMethod, MethodScope)> {
    self.core_document_mut().remove_method_and_scope(did_url)
  }

  /// Attaches the relationship to the method resolved by `method_query`.
  ///
  /// # Errors
  ///
  /// Returns an error if the method does not exist or if it is embedded.
  /// To convert an embedded method into a generic verification method, remove it first
  /// and insert it with [`MethodScope::VerificationMethod`].
  pub fn attach_method_relationship<'query, Q>(
    &mut self,
    method_query: Q,
    relationship: MethodRelationship,
  ) -> Result<bool>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self
      .core_document_mut()
      .attach_method_relationship(method_query, relationship)
      .map_err(Error::InvalidDoc)
  }

  /// Detaches the `relationship` from the method identified by `did_url`.
  /// Returns `true` if the relationship was found and removed, `false` otherwise.
  ///
  /// # Errors
  ///
  /// Returns an error if the method does not exist or is embedded.
  /// To remove an embedded method, use [`Self::remove_method`].
  ///
  /// # Note
  ///
  /// If the method is referenced in the given scope, but the document does not contain the referenced verification
  /// method, then the reference will persist in the document (i.e. it is not removed).
  pub fn detach_method_relationship<'query, Q>(
    &mut self,
    method_query: Q,
    relationship: MethodRelationship,
  ) -> Result<bool>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self
      .core_document_mut()
      .detach_method_relationship(method_query, relationship)
      .map_err(Error::InvalidDoc)
  }

  /// Returns the first [`VerificationMethod`] with an `id` property matching the
  /// provided `method_query` and the verification relationship specified by `scope` if present.
  ///
  /// # Warning
  ///
  /// Incorrect use of this method can lead to distinct document resources being identified by the same DID URL.
  pub fn resolve_method_mut<'query, Q>(
    &mut self,
    method_query: Q,
    scope: Option<MethodScope>,
  ) -> Option<&mut VerificationMethod>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self.0.resolve_method_mut(method_query, scope)
  }

  /// Returns the first [`Service`] with an `id` property matching the provided `service_query`, if present.
  // NOTE: This method demonstrates unexpected behaviour in the edge cases where the document contains
  // services whose ids are of the form <did different from this document's>#<fragment>.
  pub fn resolve_service<'query, 'me, Q>(&'me self, service_query: Q) -> Option<&Service>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self.0.resolve_service(service_query)
  }

  /// Returns the first [`VerificationMethod`] with an `id` property matching the
  /// provided `method_query` and the verification relationship specified by `scope` if present.
  // NOTE: This method demonstrates unexpected behaviour in the edge cases where the document contains methods
  // whose ids are of the form <did different from this document's>#<fragment>.
  pub fn resolve_method<'query, 'me, Q>(
    &'me self,
    method_query: Q,
    scope: Option<MethodScope>,
  ) -> Option<&VerificationMethod>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self.0.resolve_method(method_query, scope)
  }

  // ===========================================================================
  // Signatures
  // ===========================================================================

  /// Decodes and verifies the provided JWS according to the passed [`JwsVerificationOptions`] and
  /// [`JwsVerifier`].
  ///
  /// Regardless of which options are passed the following conditions must be met in order for a verification attempt to
  /// take place.
  /// - The JWS must be encoded according to the JWS compact serialization.
  /// - The `kid` value in the protected header must be an identifier of a verification method in this DID document.
  pub fn verify_jws<'jws, T: JwsVerifier>(
    &self,
    jws: &'jws Jws,
    detached_payload: Option<&'jws [u8]>,
    signature_verifier: &T,
    options: &JwsVerificationOptions,
  ) -> Result<DecodedJws<'jws>> {
    self
      .core_document()
      .verify_jws(jws.as_str(), detached_payload, signature_verifier, options)
      .map_err(Error::JwsVerificationError)
  }


}

impl AsRef<CoreDocument> for WebDocument {
    fn as_ref(&self) -> &CoreDocument {
      &self.0
    }
}

impl Display for WebDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      self.fmt_json(f)
    }
  }