// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::string::String;
use std::fmt::{self, Display, Formatter, Write};
use std::cmp::Ordering;

use serde::Deserialize;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};
use serde_json::Value;

use fastcrypto::encoding::{Base64};

use anyhow::anyhow;

use crate::iota_sdk_abstraction::move_core_types::{
    identifier::Identifier,
    language_storage::StructTag
};
use crate::iota_sdk_abstraction::types::{
    base_types::{ObjectID, SequenceNumber, ObjectType, ObjectRef, ObjectInfo, IotaAddress},
    move_package::{TypeOrigin, UpgradeInfo, MovePackage},
    iota_serde::{IotaStructTag, BigInt, SequenceNumber as AsSequenceNumber},
    digests::{ObjectDigest,TransactionDigest},
    object::Owner,
    error::{IotaObjectResponseError, UserInputResult, UserInputError},
    gas_coin::GasCoin,
};

use super::{
    Page,
    iota_move::{IotaMoveStruct, IotaMoveValue},
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct IotaObjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<IotaObjectData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<IotaObjectResponseError>,
}

impl IotaObjectResponse {
    pub fn new(data: Option<IotaObjectData>, error: Option<IotaObjectResponseError>) -> Self {
        Self { data, error }
    }

    pub fn new_with_data(data: IotaObjectData) -> Self {
        Self {
            data: Some(data),
            error: None,
        }
    }

    pub fn new_with_error(error: IotaObjectResponseError) -> Self {
        Self {
            data: None,
            error: Some(error),
        }
    }
}

impl Ord for IotaObjectResponse {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.data, &other.data) {
            (Some(data), Some(data_2)) => {
                if data.object_id.cmp(&data_2.object_id).eq(&Ordering::Greater) {
                    return Ordering::Greater;
                } else if data.object_id.cmp(&data_2.object_id).eq(&Ordering::Less) {
                    return Ordering::Less;
                }
                Ordering::Equal
            }
            // In this ordering those with data will come before IotaObjectResponses that are
            // errors.
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            // IotaObjectResponses that are errors are just considered equal.
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for IotaObjectResponse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl IotaObjectResponse {
    pub fn move_object_bcs(&self) -> Option<&Vec<u8>> {
        match &self.data {
            Some(IotaObjectData {
                     bcs: Some(IotaRawData::MoveObject(obj)),
                     ..
                 }) => Some(&obj.bcs_bytes),
            _ => None,
        }
    }

    pub fn owner(&self) -> Option<Owner> {
        if let Some(data) = &self.data {
            return data.owner;
        }
        None
    }

    pub fn object_id(&self) -> Result<ObjectID, anyhow::Error> {
        match (&self.data, &self.error) {
            (Some(obj_data), None) => Ok(obj_data.object_id),
            (None, Some(IotaObjectResponseError::NotExists { object_id })) => Ok(*object_id),
            (
                None,
                Some(IotaObjectResponseError::Deleted {
                         object_id,
                         version: _,
                         digest: _,
                     }),
            ) => Ok(*object_id),
            _ => Err(anyhow!(
                "Could not get object_id, something went wrong with IotaObjectResponse construction."
            )),
        }
    }

    pub fn object_ref_if_exists(&self) -> Option<ObjectRef> {
        match (&self.data, &self.error) {
            (Some(obj_data), None) => Some(obj_data.object_ref()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct DisplayFieldsResponse {
    pub data: Option<BTreeMap<String, String>>,
    pub error: Option<IotaObjectResponseError>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", rename = "ObjectData")]
pub struct IotaObjectData {
    pub object_id: ObjectID,
    /// Object version.
    #[serde_as(as = "AsSequenceNumber")]
    pub version: SequenceNumber,
    /// Base64 string representing the object digest
    pub digest: ObjectDigest,
    /// The type of the object. Default to be None unless
    /// IotaObjectDataOptions.showType is set to true
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<ObjectType>,
    // Default to be None because otherwise it will be repeated for the getOwnedObjects endpoint
    /// The owner of this object. Default to be None unless
    /// IotaObjectDataOptions.showOwner is set to true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
    /// The digest of the transaction that created or last mutated this object.
    /// Default to be None unless IotaObjectDataOptions.
    /// showPreviousTransaction is set to true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_transaction: Option<TransactionDigest>,
    /// The amount of IOTA we would rebate if this object gets deleted.
    /// This number is re-calculated each time the object is mutated based on
    /// the present storage gas price.
    #[serde_as(as = "Option<BigInt<u64>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_rebate: Option<u64>,
    /// The Display metadata for frontend UI rendering, default to be None
    /// unless IotaObjectDataOptions.showContent is set to true This can also
    /// be None if the struct type does not have Display defined See more details in <https://forums.iota.io/t/nft-object-display-proposal/4872>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<DisplayFieldsResponse>,
    /// Move object content or package content, default to be None unless
    /// IotaObjectDataOptions.showContent is set to true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<IotaParsedData>,
    /// Move object content or package content in BCS, default to be None unless
    /// IotaObjectDataOptions.showBcs is set to true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcs: Option<IotaRawData>,
}

impl IotaObjectData {
    pub fn object_ref(&self) -> ObjectRef {
        (self.object_id, self.version, self.digest)
    }

    pub fn object_type(&self) -> anyhow::Result<ObjectType> {
        self.type_
            .as_ref()
            .ok_or_else(|| anyhow!("type is missing for object {:?}", self.object_id))
            .cloned()
    }

    pub fn is_gas_coin(&self) -> bool {
        match self.type_.as_ref() {
            Some(ObjectType::Struct(ty)) if ty.is_gas_coin() => true,
            Some(_) => false,
            None => false,
        }
    }
}

impl Display for IotaObjectData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let type_ = if let Some(type_) = &self.type_ {
            type_.to_string()
        } else {
            "Unknown Type".into()
        };
        let mut writer = String::new();
        writeln!(
            writer,
            "{}",
            format!("----- {type_} ({}[{}]) -----", self.object_id, self.version)
        )?;
        if let Some(owner) = self.owner {
            writeln!(writer, "{}: {owner}", "Owner")?;
        }

        writeln!(
            writer,
            "{}: {}",
            "Version",
            self.version
        )?;
        if let Some(storage_rebate) = self.storage_rebate {
            writeln!(
                writer,
                "{}: {storage_rebate}",
                "Storage Rebate",
            )?;
        }

        if let Some(previous_transaction) = self.previous_transaction {
            writeln!(
                writer,
                "{}: {previous_transaction:?}",
                "Previous Transaction",
            )?;
        }

        write!(f, "{writer}")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase", rename = "ObjectDataOptions", default)]
pub struct IotaObjectDataOptions {
    /// Whether to show the type of the object. Default to be False
    pub show_type: bool,
    /// Whether to show the owner of the object. Default to be False
    pub show_owner: bool,
    /// Whether to show the previous transaction digest of the object. Default
    /// to be False
    pub show_previous_transaction: bool,
    /// Whether to show the Display metadata of the object for frontend
    /// rendering. Default to be False
    pub show_display: bool,
    /// Whether to show the content(i.e., package content or Move struct
    /// content) of the object. Default to be False
    pub show_content: bool,
    /// Whether to show the content in BCS format. Default to be False
    pub show_bcs: bool,
    /// Whether to show the storage rebate of the object. Default to be False
    pub show_storage_rebate: bool,
}

impl IotaObjectDataOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// return BCS data and all other metadata such as storage rebate
    pub fn bcs_lossless() -> Self {
        Self {
            show_bcs: true,
            show_type: true,
            show_owner: true,
            show_previous_transaction: true,
            show_display: false,
            show_content: false,
            show_storage_rebate: true,
        }
    }

    /// return full content except bcs
    pub fn full_content() -> Self {
        Self {
            show_bcs: false,
            show_type: true,
            show_owner: true,
            show_previous_transaction: true,
            show_display: false,
            show_content: true,
            show_storage_rebate: true,
        }
    }

    pub fn with_content(mut self) -> Self {
        self.show_content = true;
        self
    }

    pub fn with_owner(mut self) -> Self {
        self.show_owner = true;
        self
    }

    pub fn with_type(mut self) -> Self {
        self.show_type = true;
        self
    }

    pub fn with_display(mut self) -> Self {
        self.show_display = true;
        self
    }

    pub fn with_bcs(mut self) -> Self {
        self.show_bcs = true;
        self
    }

    pub fn with_previous_transaction(mut self) -> Self {
        self.show_previous_transaction = true;
        self
    }

    pub fn is_not_in_object_info(&self) -> bool {
        self.show_bcs || self.show_content || self.show_display || self.show_storage_rebate
    }
}

impl IotaObjectResponse {
    /// Returns a reference to the object if there is any, otherwise an Err if
    /// the object does not exist or is deleted.
    pub fn object(&self) -> Result<&IotaObjectData, IotaObjectResponseError> {
        if let Some(data) = &self.data {
            Ok(data)
        } else if let Some(error) = &self.error {
            Err(error.clone())
        } else {
            // We really shouldn't reach this code block since either data, or error field
            // should always be filled.
            Err(IotaObjectResponseError::Unknown)
        }
    }

    /// Returns the object value if there is any, otherwise an Err if
    /// the object does not exist or is deleted.
    pub fn into_object(self) -> Result<IotaObjectData, IotaObjectResponseError> {
        match self.object() {
            Ok(data) => Ok(data.clone()),
            Err(error) => Err(error),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "camelCase", rename = "ObjectRef")]
pub struct IotaObjectRef {
    /// Hex code as string representing the object id
    pub object_id: ObjectID,
    /// Object version.
    pub version: SequenceNumber,
    /// Base64 string representing the object digest
    pub digest: ObjectDigest,
}

impl IotaObjectRef {
    pub fn to_object_ref(&self) -> ObjectRef {
        (self.object_id, self.version, self.digest)
    }
}

impl Display for IotaObjectRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Object ID: {}, version: {}, digest: {}",
            self.object_id, self.version, self.digest
        )
    }
}

impl From<ObjectRef> for IotaObjectRef {
    fn from(oref: ObjectRef) -> Self {
        Self {
            object_id: oref.0,
            version: oref.1,
            digest: oref.2,
        }
    }
}

pub trait IotaData: Sized {
    type ObjectType;
    type PackageType;
    // Code is commented out because MoveObject and MoveStructLayout
    // introduce too many depencies
    // fn try_from_object(object: MoveObject, layout: MoveStructLayout)
    // -> Result<Self, anyhow::Error>;
    // fn try_from_package(package: MovePackage) -> Result<Self, anyhow::Error>;
    fn try_as_move(&self) -> Option<&Self::ObjectType>;
    fn try_into_move(self) -> Option<Self::ObjectType>;
    fn try_as_package(&self) -> Option<&Self::PackageType>;
    fn type_(&self) -> Option<&StructTag>;
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(tag = "dataType", rename_all = "camelCase", rename = "RawData")]
pub enum IotaRawData {
    // Manually handle generic schema generation
    MoveObject(IotaRawMoveObject),
    Package(IotaRawMovePackage),
}

impl IotaData for IotaRawData {
    type ObjectType = IotaRawMoveObject;
    type PackageType = IotaRawMovePackage;

    fn try_as_move(&self) -> Option<&Self::ObjectType> {
        match self {
            Self::MoveObject(o) => Some(o),
            Self::Package(_) => None,
        }
    }

    fn try_into_move(self) -> Option<Self::ObjectType> {
        match self {
            Self::MoveObject(o) => Some(o),
            Self::Package(_) => None,
        }
    }

    fn try_as_package(&self) -> Option<&Self::PackageType> {
        match self {
            Self::MoveObject(_) => None,
            Self::Package(p) => Some(p),
        }
    }

    fn type_(&self) -> Option<&StructTag> {
        match self {
            Self::MoveObject(o) => Some(&o.type_),
            Self::Package(_) => None,
        }
    }
}


#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(tag = "dataType", rename_all = "camelCase", rename = "Data")]
pub enum IotaParsedData {
    // Manually handle generic schema generation
    MoveObject(IotaParsedMoveObject),
    Package(IotaMovePackage),
}

impl IotaData for IotaParsedData {
    type ObjectType = IotaParsedMoveObject;
    type PackageType = IotaMovePackage;

    fn try_as_move(&self) -> Option<&Self::ObjectType> {
        match self {
            Self::MoveObject(o) => Some(o),
            Self::Package(_) => None,
        }
    }

    fn try_into_move(self) -> Option<Self::ObjectType> {
        match self {
            Self::MoveObject(o) => Some(o),
            Self::Package(_) => None,
        }
    }

    fn try_as_package(&self) -> Option<&Self::PackageType> {
        match self {
            Self::MoveObject(_) => None,
            Self::Package(p) => Some(p),
        }
    }

    fn type_(&self) -> Option<&StructTag> {
        match self {
            Self::MoveObject(o) => Some(&o.type_),
            Self::Package(_) => None,
        }
    }
}

impl Display for IotaParsedData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            IotaParsedData::MoveObject(o) => {
                writeln!(writer, "{}: {}", "type", o.type_)?;
                write!(writer, "{}", &o.fields)?;
            }
            IotaParsedData::Package(p) => {
                write!(
                    writer,
                    "{}: {:?}",
                    "Modules",
                    p.disassembled.keys()
                )?;
            }
        }
        write!(f, "{writer}")
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(rename = "MoveObject", rename_all = "camelCase")]
pub struct IotaParsedMoveObject {
    #[serde(rename = "type")]
    #[serde_as(as = "IotaStructTag")]
    pub type_: StructTag,
    pub has_public_transfer: bool,
    pub fields: IotaMoveStruct,
}

impl IotaParsedMoveObject {
    pub fn read_dynamic_field_value(&self, field_name: &str) -> Option<IotaMoveValue> {
        match &self.fields {
            IotaMoveStruct::WithFields(fields) => fields.get(field_name).cloned(),
            IotaMoveStruct::WithTypes { fields, .. } => fields.get(field_name).cloned(),
            _ => None,
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(rename = "RawMoveObject", rename_all = "camelCase")]
pub struct IotaRawMoveObject {
    #[serde(rename = "type")]
    #[serde_as(as = "IotaStructTag")]
    pub type_: StructTag,
    pub has_public_transfer: bool,
    pub version: SequenceNumber,
    #[serde_as(as = "Base64")]
    pub bcs_bytes: Vec<u8>,
}

impl IotaRawMoveObject {
    pub fn deserialize<'a, T: Deserialize<'a>>(&'a self) -> Result<T, anyhow::Error> {
        Ok(bcs::from_bytes(self.bcs_bytes.as_slice())?)
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(rename = "RawMovePackage", rename_all = "camelCase")]
pub struct IotaRawMovePackage {
    pub id: ObjectID,
    pub version: SequenceNumber,
    #[serde_as(as = "BTreeMap<_, Base64>")]
    pub module_map: BTreeMap<String, Vec<u8>>,
    pub type_origin_table: Vec<TypeOrigin>,
    pub linkage_table: BTreeMap<ObjectID, UpgradeInfo>,
}

impl From<MovePackage> for IotaRawMovePackage {
    fn from(p: MovePackage) -> Self {
        Self {
            id: p.id(),
            version: p.version(),
            module_map: p.serialized_module_map().clone(),
            type_origin_table: p.type_origin_table().clone(),
            linkage_table: p.linkage_table().clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "status", content = "details", rename = "ObjectRead")]
pub enum IotaPastObjectResponse {
    /// The object exists and is found with this version
    VersionFound(IotaObjectData),
    /// The object does not exist
    ObjectNotExists(ObjectID),
    /// The object is found to be deleted with this version
    ObjectDeleted(IotaObjectRef),
    /// The object exists but not found with this version
    VersionNotFound(ObjectID, SequenceNumber),
    /// The asked object version is higher than the latest
    VersionTooHigh {
        object_id: ObjectID,
        asked_version: SequenceNumber,
        latest_version: SequenceNumber,
    },
}

impl IotaPastObjectResponse {
    /// Returns a reference to the object if there is any, otherwise an Err
    pub fn object(&self) -> UserInputResult<&IotaObjectData> {
        match &self {
            Self::ObjectDeleted(oref) => Err(UserInputError::ObjectDeleted {
                object_ref: oref.to_object_ref(),
            }),
            Self::ObjectNotExists(id) => Err(UserInputError::ObjectNotFound {
                object_id: *id,
                version: None,
            }),
            Self::VersionFound(o) => Ok(o),
            Self::VersionNotFound(id, seq_num) => Err(UserInputError::ObjectNotFound {
                object_id: *id,
                version: Some(*seq_num),
            }),
            Self::VersionTooHigh {
                object_id,
                asked_version,
                latest_version,
            } => Err(UserInputError::ObjectSequenceNumberTooHigh {
                object_id: *object_id,
                asked_version: *asked_version,
                latest_version: *latest_version,
            }),
        }
    }

    /// Returns the object value if there is any, otherwise an Err
    pub fn into_object(self) -> UserInputResult<IotaObjectData> {
        match self {
            Self::ObjectDeleted(oref) => Err(UserInputError::ObjectDeleted {
                object_ref: oref.to_object_ref(),
            }),
            Self::ObjectNotExists(id) => Err(UserInputError::ObjectNotFound {
                object_id: id,
                version: None,
            }),
            Self::VersionFound(o) => Ok(o),
            Self::VersionNotFound(object_id, version) => Err(UserInputError::ObjectNotFound {
                object_id,
                version: Some(version),
            }),
            Self::VersionTooHigh {
                object_id,
                asked_version,
                latest_version,
            } => Err(UserInputError::ObjectSequenceNumberTooHigh {
                object_id,
                asked_version,
                latest_version,
            }),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(rename = "MovePackage", rename_all = "camelCase")]
pub struct IotaMovePackage {
    pub disassembled: BTreeMap<String, Value>,
}

pub type ObjectsPage = Page<IotaObjectResponse, ObjectID>;

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(rename = "GetPastObjectRequest", rename_all = "camelCase")]
pub struct IotaGetPastObjectRequest {
    /// the ID of the queried object
    pub object_id: ObjectID,
    /// the version of the queried object.
    #[serde_as(as = "AsSequenceNumber")]
    pub version: SequenceNumber,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IotaObjectDataFilter {
    MatchAll(Vec<IotaObjectDataFilter>),
    MatchAny(Vec<IotaObjectDataFilter>),
    MatchNone(Vec<IotaObjectDataFilter>),
    /// Query by type a specified Package.
    Package(ObjectID),
    /// Query by type a specified Move module.
    MoveModule {
        /// the Move package ID
        package: ObjectID,
        /// the module name
        #[serde_as(as = "DisplayFromStr")]
        module: Identifier,
    },
    /// Query by type
    StructType(
        #[serde_as(as = "IotaStructTag")]
        StructTag,
    ),
    AddressOwner(IotaAddress),
    ObjectOwner(ObjectID),
    ObjectId(ObjectID),
    // allow querying for multiple object ids
    ObjectIds(Vec<ObjectID>),
    Version(
        #[serde_as(as = "BigInt<u64>")]
        u64,
    ),
}

impl IotaObjectDataFilter {
    pub fn gas_coin() -> Self {
        Self::StructType(GasCoin::type_())
    }

    pub fn and(self, other: Self) -> Self {
        Self::MatchAll(vec![self, other])
    }
    pub fn or(self, other: Self) -> Self {
        Self::MatchAny(vec![self, other])
    }
    pub fn not(self, other: Self) -> Self {
        Self::MatchNone(vec![self, other])
    }

    pub fn matches(&self, object: &ObjectInfo) -> bool {
        match self {
            IotaObjectDataFilter::MatchAll(filters) => !filters.iter().any(|f| !f.matches(object)),
            IotaObjectDataFilter::MatchAny(filters) => filters.iter().any(|f| f.matches(object)),
            IotaObjectDataFilter::MatchNone(filters) => !filters.iter().any(|f| f.matches(object)),
            IotaObjectDataFilter::StructType(s) => {
                let obj_tag: StructTag = match &object.type_ {
                    ObjectType::Package => return false,
                    ObjectType::Struct(s) => s.clone().into(),
                };
                // If people do not provide type_params, we will match all type_params
                // e.g. `0x2::coin::Coin` can match `0x2::coin::Coin<0x2::iota::IOTA>`
                if !s.type_params.is_empty() && s.type_params != obj_tag.type_params {
                    false
                } else {
                    obj_tag.address == s.address
                        && obj_tag.module == s.module
                        && obj_tag.name == s.name
                }
            }
            IotaObjectDataFilter::MoveModule { package, module } => {
                matches!(&object.type_, ObjectType::Struct(s) if &ObjectID::from(s.address()) == package
                        && s.module() == module.as_ident_str())
            }
            IotaObjectDataFilter::Package(p) => {
                matches!(&object.type_, ObjectType::Struct(s) if &ObjectID::from(s.address()) == p)
            }
            IotaObjectDataFilter::AddressOwner(a) => {
                matches!(object.owner, Owner::AddressOwner(addr) if &addr == a)
            }
            IotaObjectDataFilter::ObjectOwner(o) => {
                matches!(object.owner, Owner::ObjectOwner(addr) if addr == IotaAddress::from(*o))
            }
            IotaObjectDataFilter::ObjectId(id) => &object.object_id == id,
            IotaObjectDataFilter::ObjectIds(ids) => ids.contains(&object.object_id),
            IotaObjectDataFilter::Version(v) => object.version.value() == *v,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase", rename = "ObjectResponseQuery", default)]
pub struct IotaObjectResponseQuery {
    /// If None, no filter will be applied
    pub filter: Option<IotaObjectDataFilter>,
    /// config which fields to include in the response, by default only digest
    /// is included
    pub options: Option<IotaObjectDataOptions>,
}

impl IotaObjectResponseQuery {
    pub fn new(
        filter: Option<IotaObjectDataFilter>,
        options: Option<IotaObjectDataOptions>,
    ) -> Self {
        Self { filter, options }
    }

    pub fn new_with_filter(filter: IotaObjectDataFilter) -> Self {
        Self {
            filter: Some(filter),
            options: None,
        }
    }

    pub fn new_with_options(options: IotaObjectDataOptions) -> Self {
        Self {
            filter: None,
            options: Some(options),
        }
    }
}