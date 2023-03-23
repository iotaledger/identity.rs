// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = JwsHeader)]
pub struct WasmJwsHeader(JwsHeader); 

#[wasm_bindgen(js_class = JwsHeader)]
impl WasmJwsHeader {
    /// Create a new empty `JwsHeader`.
    #[wasm_bindgen(constructor)]
    pub const fn new() -> Self {
      Self {
        common: JwtHeader::new(),
        alg: None,
        b64: None,
        ppt: None,
      }
    }
  
    /// Returns the value for the algorithm claim (alg).
    #[wasm_bindgen]
    pub fn alg(&self) -> Option<JwsAlgorithm> {
      self.0.alg().map(|alg| alg.name().to_owned())
    }
    
    
    /// Sets a value for the algorithm claim (alg).
    #[wasm_bindgen(js_name = setAlg)]
    pub fn set_alg(&mut self, value: impl Into<JwsAlgorithm>) {
      self.0.set_alg(value);
    }
  
    /// Returns the value of the base64url-encode payload claim (b64).
    pub fn b64(&self) -> Option<bool> {
      self.0.b64()
    }
  
    /// Sets a value for the base64url-encode payload claim (b64).
    #[wasm_bindgen(js_name = setB64)]
    pub fn set_b64(&mut self, value: bool) {
      self.0.set_b64(value);
    }
  
    /// Returns the value of the passport extension claim (ppt).
    pub fn ppt(&self) -> Option<String> {
      self.0.ppt().map(ToOwned::to_owned)
    }
  
    /// Sets a value for the passport extension claim (ppt).
    #[wasm_bindgen(js_name = setPpt)]
    pub fn set_ppt(&mut self, value: String) {
      self.0.set_ppt(value);
    }
  
    // ===========================================================================
    // ===========================================================================
    
    #[wasm_bindgen]
    pub fn has(&self, claim: &str) -> bool {
        self.0.has(claim)
    }
  
    /// Returns `true` if none of the fields are set in both `self` and `other`.
    #[wasm_bindgen(js_name = isDisjoint)]
    pub fn is_disjoint(&self, other: &WasmJwsHeader) -> bool {
        self.0.is_disjoint(&other.0)
    }

// ===========================================================================
// Common JWT parameters 
// ===========================================================================

/// Returns the value of the JWK Set URL claim (jku).
#[wasm_bindgen]
  pub fn jku(&self) -> Option<String> {
    self.0.deref().jku().map(|url|url.to_string())
  }

  /// Sets a value for the JWK Set URL claim (jku).
  #[wasm_bindgen(js_name = setJku)]
  pub fn set_jku(&mut self, value: String) -> Result<()> {
    let url = Url::parse(value).wasm_result()?;
    self.0.deref_mut().set_jku(url);
  }

  /// Returns the value of the JWK claim (jwk).
  #[wasm_bindgen]
  pub fn jwk(&self) -> Option<&Jwk> {
    self.0.deref().jwk()
  }

  /// Sets a value for the JWK claim (jwk).
  #[wasm_bindgen(js_name = setJwk)]
  pub fn set_jwk(&mut self, value: &WasmJwk) {
    self.0.deref_mut().set_jwk(value.0.clone())
  }

  /// Returns the value of the key ID claim (kid).
  #[wasm_bindgen]
  pub fn kid(&self) -> Option<&str> {
    self.0.deref()
  }

  /// Sets a value for the key ID claim (kid).
  #[wasm_bindgen(js_name = setKid)]
  pub fn set_kid(&mut self, value: String) {
    self.0.deref_mut(value);
  }

  /// Returns the value of the X.509 URL claim (x5u).
  #[wasm_bindgen]
  pub fn x5u(&self) -> Option<String> {
    self.0.deref().x5u().map(|url| url.to_string())
  }

  /// Sets a value for the X.509 URL claim (x5u).
  #[wasm_bindgen(js_name = setX5u)]
  pub fn set_x5u(&mut self, value: String) -> Result<()> {
    let url = Url::parse(value).wasm_result()?;
    self.0.deref_mut().set_x5u(url);
  }

  /// Returns the value of the X.509 certificate chain claim (x5c).
  #[wasm_bindgen]
  pub fn x5c(&self) -> Option<Vec<String>> {
    self.0.deref().x5c().map(ToOwned::to_owned)
  }

  /// Sets values for the X.509 certificate chain claim (x5c).
  #[wasm_bindgen(js_name = setX5c)]
  pub fn set_x5c(&mut self, value: Vec<String>) {
    self.0.deref_mut().set_x5c(value);
  }

  /// Returns the value of the X.509 certificate SHA-1 thumbprint claim (x5t).
  #[wasm_bindgen]
  pub fn x5t(&self) -> Option<String> {
    self.0.deref().x5t().map(ToOwned::to_owned)
  }

  /// Sets a value for the X.509 certificate SHA-1 thumbprint claim (x5t).
  #[wasm_bindgen(js_name = setX5t)]
  pub fn set_x5t(&mut self, value: String) {
    self.0.deref_mut().set_x5t(value);
  }

  /// Returns the value of the X.509 certificate SHA-256 thumbprint claim
  /// (x5t#S256).
  #[wasm_bindgen(js_name = x5tS256)]
  pub fn x5t_s256(&self) -> Option<String> {
    self.0.deref().x5t_s256().map(ToOwned::to_owned)
  }

  /// Sets a value for the X.509 certificate SHA-256 thumbprint claim
  /// (x5t#S256).
  #[wasm_bindgen(js_name = setX5tS256)]
  pub fn set_x5t_s256(&mut self, value: String) {
    self.0.deref_mut().set_x5t_s256(value);
  }

  /// Returns the value of the token type claim (typ).
  #[wasm_bindgen]
  pub fn typ(&self) -> Option<String> {
    self.0.deref().typ().map(ToOwned::to_owned)
  }

  /// Sets a value for the token type claim (typ).
  #[wasm_bindgen(js_name = setTyp)]
  pub fn set_typ(&mut self, value: String) {
    self.0.deref_mut().set_typ(value);
  }

  /// Returns the value of the content type claim (cty).
  #[wasm_bindgen]
  pub fn cty(&self) -> Option<String> {
    self.0.deref().cty().map(ToOwned::to_owned)
  }

  /// Sets a value for the content type claim (cty).
  #[wasm_bindgen(js_name = setCty)]
  pub fn set_cty(&mut self, value: String) {
    self.0.deref_mut().set_cty(value);
  }

  /// Returns the value of the critical claim (crit).
  #[wasm_bindgen]
  pub fn crit(&self) -> Option<Vec<String>> {
    self.0.deref().crit().map(ToOwned::to_owned)
  }

  /// Sets values for the critical claim (crit).
  #[wasm_bindgen(js_name = setCrit)]
  pub fn set_crit(&mut self, value: Vec<String>) {
    self.0.deref_mut().set_crit(value)
  }

  /// Returns the value of the url claim (url).
  #[wasm_bindgen]
  pub fn url(&self) -> Option<String> {
    self.0.deref().url().map(|url| url.to_string())
  }

  /// Sets a value for the url claim (url).
  #[wasm_bindgen(js_name = setUrl)]
  pub fn set_url(&mut self, value: String) -> Result<()> {
    let url = Url::parse(value).wasm_result()?;
    self.0.deref_mut().set_url(value);
  }

  /// Returns the value of the nonce claim (nonce).
  #[wasm_bindgen]
  pub fn nonce(&self) -> Option<String> {
    self.0.deref().nonce().map(ToOwned::to_owned)
  }

  /// Sets a value for the nonce claim (nonce).
  #[wasm_bindgen(js_name = setNonce)]
  pub fn set_nonce(&mut self, value: String) {
    self.0.deref_mut().set_nonce(value);
    }
    
  }