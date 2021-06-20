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
    description: Option<String>, // long text (no constraints)
    label: Option<String>, //short text (no constraints); default to key
    comment: Option<String>, // source code comments only, no semantics
    // sdfRef: sdf-pointer
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum JSONSchemaTypes {
    Number,
    String,
    Boolean,
    Integer,
    Array,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct JSONSchema {
    r#type: JSONSchemaTypes,
    // TODO: Add more members
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
    jsonschema: JSONSchema,

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
