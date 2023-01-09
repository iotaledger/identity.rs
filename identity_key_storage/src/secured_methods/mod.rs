// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod cryptosuite;
mod document_ext;
mod signing_material_construction_error;
mod method_creation_error;
mod method_removal_error;
mod remote_key;
mod signing_material;
pub use document_ext::CoreDocumentExt;
pub use signing_material_construction_error::SigningMaterialConstructionError;
mod eddsa_2020;
mod signable;

pub use method_creation_error::MethodCreationError;
pub use method_removal_error::MethodRemovalError;
pub use remote_key::RemoteKey;
pub use signable::*;
