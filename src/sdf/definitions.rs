use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SDFModel {
    pub info: Option<InfoBlock>,
    pub namespace: Option<HashMap<String, String>>,
    pub default_namespace: Option<String>,
    pub sdf_thing: Option<HashMap<String, ThingQualities>>,
    pub sdf_product: Option<HashMap<String, ProductQualities>>,
    pub sdf_object: Option<HashMap<String, ObjectQualities>>,
    pub sdf_property: Option<HashMap<String, PropertyQualities>>,
    pub sdf_action: Option<HashMap<String, ActionQualities>>,
    pub sdf_event: Option<HashMap<String, EventQualities>>,
    pub sdf_data: Option<HashMap<String, DataQualities>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonQualities {
    pub description: Option<String>,
    pub label: Option<String>,
    pub comment: Option<String>,
    pub sdf_ref: Option<String>,
    pub sdf_required: Option<Vec<String>>,
    // TODO: Add validation for sdfRef and sdfRequired
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThingQualities {
    #[serde(flatten)]
    pub common_qualities: CommonQualities,
    pub sdf_object: Option<HashMap<String, ObjectQualities>>,
    pub sdf_thing: Option<HashMap<String, ThingQualities>>,
}

use ThingQualities as ProductQualities;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectQualities {
    #[serde(flatten)]
    pub common_qualities: CommonQualities,
    pub sdf_property: Option<HashMap<String, PropertyQualities>>,
    pub sdf_action: Option<HashMap<String, ActionQualities>>,
    pub sdf_event: Option<HashMap<String, EventQualities>>,
    pub sdf_data: Option<HashMap<String, DataQualities>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberTypeQualities<T> {
    #[serde(flatten)]
    pub common_qualities: TypedQualities<T>,
    pub minimum: Option<T>,
    pub maximum: Option<T>,
    pub exclusive_minimum: Option<T>,
    pub exclusive_maximum: Option<T>,
    pub multiple_of: Option<T>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FormatQualities {
    DateTime,
    Date,
    Time,
    Uri,
    UriReference,
    Uuid,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TypedQualities<T> {
    pub r#const: Option<T>,
    pub default: Option<T>,
    pub r#enum: Option<Vec<T>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StringTypeQualities {
    #[serde(flatten)]
    pub common_qualities: TypedQualities<String>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub pattern: Option<String>,
    pub format: Option<FormatQualities>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectTypeQualities {
    #[serde(flatten)]
    pub common_qualities: TypedQualities<HashMap<String, serde_json::Value>>,
    pub required: Option<Vec<String>>,
    pub properties: Option<HashMap<String, DataQualities>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrayTypeQualities {
    pub min_items: Option<u32>,
    pub max_items: Option<u32>,
    pub unique_items: Option<bool>,
    pub items: Option<Vec<DataQualities>>, // TODO: Should this be an array/map?
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum RegularTypes {
    Number(NumberTypeQualities<f64>),
    String(StringTypeQualities),
    Boolean(TypedQualities<bool>), // TODO: Does "enum" make sense here?
    Integer(NumberTypeQualities<i64>),
    Array(ArrayTypeQualities),
    Object(ObjectTypeQualities),
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdfChoice {
    sdf_choice: HashMap<String, DataQualities>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Types {
    Type(RegularTypes),
    SdfChoice(SdfChoice),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SDFType {
    ByteString,
    UnixTime,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataQualities {
    #[serde(flatten)]
    pub common_qualities: CommonQualities,

    #[serde(flatten)]
    pub jsonschema: Option<Types>,

    pub unit: Option<String>,
    pub observable: Option<bool>,
    pub readable: Option<bool>,
    pub writable: Option<bool>,
    pub nullable: Option<bool>,
    pub sdf_type: Option<SDFType>,
    pub content_format: Option<String>,
}
pub type PropertyQualities = DataQualities;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionQualities {
    #[serde(flatten)]
    pub common_qualities: CommonQualities,
    pub sdf_input_data: Option<DataQualities>,
    pub sdf_output_data: Option<DataQualities>,
    pub sdf_data: Option<HashMap<String, DataQualities>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventQualities {
    #[serde(flatten)]
    pub common_qualities: CommonQualities,
    pub sdf_output_data: Option<DataQualities>,
    pub sdf_data: Option<HashMap<String, DataQualities>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InfoBlock {
    pub title: String,
    pub version: String,
    pub copyright: String,
    pub license: String,
}
