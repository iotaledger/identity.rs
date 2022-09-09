// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This module contains JSON strings of a valid presentation together with the DID Documents of the holder and
// credential issuers.

pub(super) const PRESENTATION_JSON: &str =
  include_str!("../../../../identity_credential/tests/fixtures/signed_presentation/presentation.json");
pub(super) const HOLDER_FOO_DOC_JSON: &str =
  include_str!("../../../../identity_credential/tests/fixtures/signed_presentation/subject_foo_doc.json");
pub(super) const ISSUER_IOTA_DOC_JSON: &str =
  include_str!("../../../../identity_credential/tests/fixtures/signed_presentation/issuer_iota_doc.json");
pub(super) const ISSUER_BAR_DOC_JSON: &str =
  include_str!("../../../../identity_credential/tests/fixtures/signed_presentation/issuer_bar_doc.json");
