use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Context {
  String(String),
  Array(Vec<ContextEntry>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum ContextEntry {
  String(String),
  Map(HashMap<String, String>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum StringOrArrayOfString {
  String(String),
  Array(Vec<String>),
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Thing {
    #[serde(rename = "@context")]
    context: Context,
    #[serde(rename = "@type")]
    r#type: Option<StringOrArrayOfString>,
    id: Option<String>,
    title: String,
    titles: Option<HashMap<String, String>>,
    description: Option<String>,
    descriptions: Option<HashMap<String, String>>,
    version: Option<VersionInfo>,
    created: Option<DateTime<Utc>>,
    modified: Option<DateTime<Utc>>,
    support: Option<String>,
    base: Option<String>,
    // actions: Option<HashMap<String, ActionAffordance>>,
    // properties: Option<HashMap<String, PropertyAffordance>>,
    // events: Option<HashMap<String, EventAffordance>>,
    // links: Option<Vec<Link>>,
    forms: Option<Vec<Form>>,
    // security: Vec<Security>,
    // security_definitions: HashMap<String, SecurityDefinition>,
    // profile: Option<Profile>,
    // schema_definitions: Option<HashMap<String, DataSchema>>
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct VersionInfo {
    instance: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum JSONSchemaTypes {
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DataSchema {
    #[serde(rename = "@type")]
    r#type: Option<StringOrArrayOfString>,
    title: Option<HashMap<String, String>>,
    titles: Option<HashMap<String, String>>, // TODO: Consider using a MultiLanguage struct instead
    description: Option<String>,
    descriptions: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    data_type: Option<JSONSchemaTypes>,
    r#const: Option<serde_json::Value>,
    unit: Option<String>,
    one_of: Option<Vec<DataSchema>>,
    r#enum: Option<Vec<serde_json::Value>>,
    read_only: Option<bool>,
    write_only: Option<bool>,
    format: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct InteractionAffordance {
    #[serde(rename = "@type")]
    r#type: Option<StringOrArrayOfString>,
    title: Option<String>,
    titles: Option<HashMap<String, String>>,
    description: Option<String>,
    descriptions: Option<HashMap<String, String>>,
    forms: Vec<Form>,
    uri_variables: Option<HashMap<String, DataSchema>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum OperationType {
    Readproperty,
    Writeproperty,
    Observeproperty,
    Unobserveproperty,
    Invokeaction,
    Subscribeevent,
    Unsubscribeevent,
    Readallproperties,
    Writeallproperties,
    Readmultipleproperties,
    Writemultipleproperties,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Form { // TODO: Define forms for different affordance types
    op: Option<OperationType>, // TODO: Can be array
    href: String,
    content_type: Option<String>,
    content_coding: Option<String>,
    subprotocol: Option<String>,
    security: Option<StringOrArrayOfString>,
    scopes: Option<StringOrArrayOfString>,
    // response: Option<ExpectedResponse>,
}

