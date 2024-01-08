use identity_document::document::CoreDocument;

use crate::{credential::Jpt, validator::FailFast};

use super::JptPresentationValidationOptions;



// /// A type for decoding and validating [`Credential`]s in JPT format representing a JWP in Presented form. //TODO: validator
// #[non_exhaustive]
// pub struct JptPresentationValidator;

// impl JptPresentationValidator {


//     /// Decodes and validates a [`Credential`] issued as a JPT. A [`DecodedJptCredential`] is returned upon success.
//     ///
//     /// The following properties are validated according to `options`:
//     /// - the issuer's proof on the JWP,
//     /// - the expiration date,
//     /// - the issuance date,
//     /// - the semantic structure.
//     pub fn validate<DOC, T>(
//         credential_jpt: &Jpt, //TODO: the validation process could be handled both for JWT and JPT by the same function, the function could recognise if the token in input is a JWT or JPT based on the typ field
//         issuer: &DOC,
//         options: &JptPresentationValidationOptions,
//         fail_fast: FailFast,
//       ) -> Result<DecodedJptCredential<T>, CompoundCredentialValidationError>
//       where
//         T: ToOwned<Owned = T> + serde::Serialize + serde::de::DeserializeOwned,
//         DOC: AsRef<CoreDocument>,
//       {
//         Self::validate_extended::<CoreDocument, T>(
//           credential_jpt,
//           std::slice::from_ref(issuer.as_ref()),
//           options,
//           fail_fast,
//         )
//       }
// }