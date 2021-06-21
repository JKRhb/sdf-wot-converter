use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SDFModel {
    info: Option<InfoBlock>,
    namespace: Option<HashMap<String, String>>,
    default_namespace: Option<String>,
    sdf_thing: Option<HashMap<String, ThingQualities>>,
    sdf_product: Option<HashMap<String, ProductQualities>>,
    sdf_object: Option<HashMap<String, ObjectQualities>>,
    sdf_property: Option<HashMap<String, PropertyQualities>>,
    sdf_action: Option<HashMap<String, ActionQualities>>,
    sdf_event: Option<HashMap<String, EventQualities>>,
    sdf_data: Option<HashMap<String, DataQualities>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
struct CommonQualities {
    description: Option<String>,
    label: Option<String>,
    comment: Option<String>,
    // TODO: Add sdfRef and sdfRequired
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ThingQualities {
    #[serde(flatten)]
    common_qualities: CommonQualities,
    sdf_object: Option<HashMap<String, ObjectQualities>>,
    sdf_thing: Option<HashMap<String, ThingQualities>>,
}

use ThingQualities as ProductQualities;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ObjectQualities {
    #[serde(flatten)]
    common_qualities: CommonQualities,
    sdf_property: Option<HashMap<String, PropertyQualities>>,
    sdf_action: Option<HashMap<String, ActionQualities>>,
    sdf_event: Option<HashMap<String, EventQualities>>,
    sdf_data: Option<HashMap<String, DataQualities>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct NumberTypeQualities<T> {
    minimum: Option<T>,
    maximum: Option<T>,
    exclusive_minimum: Option<T>,
    exclusive_maximum: Option<T>,
    multiple_of: Option<T>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
enum FormatQualities {
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
struct TypedQualities<T> {
    r#const: Option<T>,
    default: Option<T>,
    r#enum: Option<Vec<T>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct StringTypeQualities {
    #[serde(flatten)]
    common_qualities: TypedQualities<String>,
    min_length: Option<i32>,
    max_length: Option<i32>,
    pattern: Option<String>,
    format: Option<FormatQualities>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ObjectTypeQualities {
    #[serde(flatten)]
    common_qualities: TypedQualities<serde_json::Map<String, serde_json::Value>>,
    required: Option<Vec<String>>,
    properties: Option<HashMap<String, DataQualities>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ArrayTypeQualities {
    min_items: Option<i32>,
    max_items: Option<i32>,
    unique_items: Option<bool>,
    // items: Option<DataQualities>, // TODO: Not the actual schema entry yet
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
enum RegularTypes {
    Number(NumberTypeQualities<f32>),
    String(StringTypeQualities),
    Boolean(TypedQualities<bool>), // TODO: Does "enum" make sense here?
    Integer(NumberTypeQualities<i32>),
    Array(ArrayTypeQualities),
    Object(ObjectTypeQualities),
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SdfChoice {
    sdf_choice: HashMap<String, DataQualities>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
enum Types {
    Type(RegularTypes),
    SdfChoice(SdfChoice),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
enum SDFType {
    ByteString,
    UnixTime,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DataQualities {
    #[serde(flatten)]
    common_qualities: CommonQualities,

    #[serde(flatten)]
    jsonschema: Option<Types>,

    unit: Option<String>,
    observable: Option<bool>,
    readable: Option<bool>,
    writable: Option<bool>,
    nullable: Option<bool>,
    sdf_type: Option<SDFType>,
    content_format: Option<String>,
}
use DataQualities as PropertyQualities;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ActionQualities {
    #[serde(flatten)]
    common_qualities: CommonQualities,
    sdf_input_data: Option<DataQualities>,
    sdf_output_data: Option<DataQualities>,
    sdf_data: Option<HashMap<String, DataQualities>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct EventQualities {
    #[serde(flatten)]
    common_qualities: CommonQualities,
    sdf_output_data: Option<DataQualities>,
    sdf_data: Option<HashMap<String, DataQualities>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct InfoBlock {
    title: String,
    version: String,
    copyright: String,
    license: String,
}
