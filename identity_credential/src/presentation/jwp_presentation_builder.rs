use jsonprooftoken::jwp::header::PresentationProtectedHeader;
use jsonprooftoken::jwp::{presented::JwpPresentedBuilder, issued::JwpIssued};
use crate::error::Error;
use crate::error::Result;


//TODO: ZKP - This could be subject to changes with the introduction of the revocation mechanism.

/// Used to construct a JwpPresentedBuilder and handle the selective disclosure of attributes
/// - @context MUST NOT be blinded
/// - id MUST be blinded
/// - type MUST NOT be blinded
/// - issuer MUST NOT be blinded
/// - issuanceDate MUST be blinded (if Timeframe Revocation mechanism is used)
/// - expirationDate MUST be blinded (if Timeframe Revocation mechanism is used)
/// - credentialSubject (User have to choose which attribute must be blinded)
/// - credentialSchema MUST NOT be blinded
/// - credentialStatus Will be used for Revocation mechanism, some fields could be blinded
/// - refreshService MUST NOT be blinded (perhaps will be used for Timeframe Revocation mechanism)
/// - termsOfUse NO reason to use it in ZK VC (will be in any case blinded)
/// - evidence (User have to choose which attribute must be blinded)
pub struct SelectiveDisclosurePresentation {
    jwp_builder: JwpPresentedBuilder
}

impl SelectiveDisclosurePresentation {

    /// Inizialize a presentation starting from an Issued JWP
    pub fn new(issued_jwp: &JwpIssued) -> Self {
        let mut jwp_builder = JwpPresentedBuilder::new(issued_jwp);

        jwp_builder.set_undisclosed("jti").ok(); // contains the credential's id, provides linkability
        
        jwp_builder.set_undisclosed("nbf").ok();
        jwp_builder.set_undisclosed("issuanceDate").ok(); // Undisclosed using Timeframe Revocation mechanism

        jwp_builder.set_undisclosed("expirationDate").ok(); // Undisclosed using Timeframe Revocation mechanism

        jwp_builder.set_undisclosed("termsOfUse").ok(); // Provides linkability so, there is NO reason to use it in ZK VC

        Self{jwp_builder}
    }

    /// Selectively disclose "credentialSubject" attributes.
    /// # Example 
    /// ```
    /// {
    ///     "id": 1234,
    ///     "name": "Alice",
    ///     "mainCourses": ["Object-oriented Programming", "Mathematics"],
    ///     "degree": {
    ///         "type": "BachelorDegree",
    ///         "name": "Bachelor of Science and Arts",
    ///     },
    ///     "GPA": "4.0",
    /// }
    /// ```
    /// If you want to undisclose for example the Mathematics course and the name of the degree:
    /// ```
    /// undisclose_subject("mainCourses[1]");
    /// undisclose_subject("degree.name");
    /// ```
    pub fn undisclose_subject(&mut self, path: &str) -> Result<(), Error>{
        let _ = self.jwp_builder.set_undisclosed(&("vc.credentialSubject.".to_owned() + path)).map_err(|_| Error::SelectiveDiscosureError);
        Ok(())
    }
    
    /// Selectively disclose "evidence" attributes
    pub fn undisclose_evidence(&mut self, path: &str) -> Result<(), Error> {
        let _ = self.jwp_builder.set_undisclosed(&("vc.evidence.".to_owned() + path)).map_err(|_| Error::SelectiveDiscosureError);
        Ok(())
    } 


    // Other option

    /// Undisclose a generic attribute passing its path
    pub fn set_undisclosed(&mut self, path: &str) -> Result<()>{
        let _ = self.jwp_builder.set_undisclosed(path).map_err(|_| Error::SelectiveDiscosureError);
        Ok(())
    }


    /// Set Presenation Protected Header
    pub fn set_presentation_header(&mut self, ph: PresentationProtectedHeader) {
        self.jwp_builder.presentation_protected_header(ph);
    }


    /// Get the builder
    pub fn builder(&self) -> &JwpPresentedBuilder {
        &self.jwp_builder
    }

}