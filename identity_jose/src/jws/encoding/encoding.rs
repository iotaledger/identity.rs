// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0


/*

pub struct SigningInputItem<'builder_borrow, 'payload> {
  builder: &'builder_borrow mut Builder<'payload>,
  protected: Option<String>,
  signing_input: Box<[u8]>,
}

pub struct Builder<'payload> {
    /// The payload
    payload: Cow<'payload, [u8]>,
    /// The output format of the encoded token.
    format: JwsFormat,
    /// Content validation rules for unencoded content using the compact format.
    charset: CharSet,
    /// Encode the token with detached content.
    ///
    /// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
    detached: bool,
    /// Per-recipient configuration.
    signatures: Vec<JwsSignature>,

    // Must be set on the first signature push and not changed.
    // Consider using something like OnceCell to make this clear at the type level.
    b64: Option<bool>,
  }

  impl<'a> Builder<'a> {
    pub fn new(payload: &'a [u8], format: JwsFormat) -> Self {
      Self {
        payload,
        format,
        charset: CharSet::Default,
        detached: false,
        signatures: Vec::new(),
      }
    }

    pub fn charset(mut self, value: CharSet) -> Self {
      self.charset = value;
      self
    }

    pub fn detached(mut self, value: bool) -> Self {
      self.detached = value;
      self
    }

    pub fn start_signature_push<'builder_borrow>(&'builder_borrow mut self, protected_header: Option<JwsHeader>, unprotected_header: Option<JwsHeader>) -> Result<SigningInputItem<'builder_borrow>> {
      // Validate the headers
      {
      match (protected_header, unprotected_header) {
        (_,Some(unprotected)) if self.format == JwsFromat::Compact => {
          return Err(Error::UnprotectedHeaderInCompactJwsSerialization);
        },
        (None, None) => {
          return Err(Error::MissingHeader("cannot create JWS without a header"))
        },
        _ => {}
      }
      jwu::validate_jws_headers(
        protected_header,
        unprotected_header,
        protected_header.and_then(|header| header.crit()),
      )?;

      };

      // Extract the "b64" header parameter
      // See: https://tools.ietf.org/html/rfc7797#section-3

      let b64: bool = jwu::extract_b64(protected_header);
      {
      // Ensure that b64 is the same for all signatures.
      if let Some(value) = self.b64 {
        if value != b64 {
          return Err(Error::InvalidParam("b64"))
        };
      } else {
        self.b64 = Some(b64);
      }
    }

    // Encode payload as required for signing input
      let tmp: String;
      let payload: &[u8] = if b64 {
        tmp = jwu::encode_b64(builder.payload);
        tmp.as_bytes()
      } else if self.detached {
        claims
      } else {
        self.builder.charset.validate(claims)?;
        claims
      };

      let protected: Option<String> = recipient.protected.map(jwu::encode_b64_json).transpose()?;
      let header_bytes: &[u8] = protected.as_deref().map(str::as_bytes).unwrap_or_default();
      let signing_input: Box<[u8]> = jwu::create_message(header_bytes, payload).into();

      Ok(
        SigningInputItem{
          builder: &mut self,
          protected,
          signing_input
        }
      )
    }
}

impl<'builder_borrow> SigningInputItem<'builder_borrow> {

  pub fn signing_input(&self) -> &[u8] {
    &self.signing_input
  }

  pub fn complete_signature_push(self, signature: &[u8]) {

  }
}

*/


