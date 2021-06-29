use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Context {
    String(String),
    Array(Vec<ContextEntry>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ContextEntry {
    String(String),
    Map(HashMap<String, String>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TypeOrTypeArray<T> {
    Type(T),
    Array(Vec<T>),
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Thing {
    #[serde(rename = "@context")]
    pub context: Context,
    #[serde(rename = "@type")]
    pub r#type: Option<TypeOrTypeArray<String>>,
    pub id: Option<String>,
    pub title: String,
    pub titles: Option<HashMap<String, String>>, // TODO: Consider using a MultiLanguage struct instead
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, String>>,
    pub version: Option<VersionInfo>,
    pub created: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
    pub support: Option<String>,
    pub base: Option<String>,
    pub actions: Option<HashMap<String, ActionAffordance>>,
    pub properties: Option<HashMap<String, PropertyAffordance>>,
    pub events: Option<HashMap<String, EventAffordance>>,
    pub links: Option<Vec<Link>>,
    pub forms: Option<Vec<Form>>,
    pub security: TypeOrTypeArray<String>,
    pub security_definitions: HashMap<String, SecurityScheme>,
    pub profile: Option<TypeOrTypeArray<String>>,
    pub schema_definitions: Option<HashMap<String, DataSchema>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SchemeIn {
    Header,
    Query,
    Body,
    Cookie,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum SchemaQoP {
    Auth,
    AuthInit,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "scheme")]
pub enum SecurityScheme {
    Nosec {
        #[serde(flatten)]
        common: SecuritySchemeCommon,
    },
    Basic {
        #[serde(flatten)]
        common: SecuritySchemeCommon,
        r#in: Option<SchemeIn>,
        name: Option<String>,
    },
    Combo {
        #[serde(flatten)]
        common: SecuritySchemeCommon,
        one_of: TypeOrTypeArray<String>,
        all_of: TypeOrTypeArray<String>,
    },
    Digest {
        #[serde(flatten)]
        common: SecuritySchemeCommon,
        qop: Option<SchemaQoP>,
        r#in: Option<SchemeIn>,
        name: Option<String>,
    },
    Bearer {
        #[serde(flatten)]
        common: SecuritySchemeCommon,
        authorization: Option<String>,
        alg: Option<String>,
        format: Option<String>,
        r#in: Option<SchemeIn>,
        name: Option<String>,
    },
    PSK {
        #[serde(flatten)]
        common: SecuritySchemeCommon,
        identity: Option<String>,
    },
    Oauth2 {
        #[serde(flatten)]
        common: SecuritySchemeCommon,
        authorization: Option<String>,
        token: Option<String>,
        refresh: Option<String>,
        scrops: Option<TypeOrTypeArray<String>>,
        flow: String,
    },
    Apikey {
        #[serde(flatten)]
        common: SecuritySchemeCommon,
        r#in: Option<SchemeIn>,
        name: Option<String>,
    },
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecuritySchemeCommon {
    #[serde(rename = "@type")]
    pub r#type: Option<TypeOrTypeArray<String>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, String>>,
    pub proxy: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo {
    pub instance: String,
    pub model: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum JSONSchemaTypes {
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
pub struct DataSchema {
    #[serde(rename = "@type")]
    pub r#type: Option<TypeOrTypeArray<String>>,
    pub title: Option<String>,
    pub titles: Option<HashMap<String, String>>, // TODO: Consider using a MultiLanguage struct instead
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub data_type: Option<JSONSchemaTypes>,
    pub r#const: Option<serde_json::Value>,
    pub unit: Option<String>,
    pub one_of: Option<Vec<DataSchema>>,
    pub r#enum: Option<Vec<serde_json::Value>>,
    pub read_only: Option<bool>,
    pub write_only: Option<bool>,
    pub format: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionAffordance {
    #[serde(rename = "@type")]
    pub r#type: Option<TypeOrTypeArray<String>>,
    pub title: Option<String>,
    pub titles: Option<HashMap<String, String>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, String>>,
    pub forms: Vec<Form>,
    pub uri_variables: Option<HashMap<String, DataSchema>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyAffordance {
    #[serde(flatten)]
    pub interaction_affordance: InteractionAffordance,

    #[serde(flatten)]
    pub data_schema: DataSchema,
    pub observable: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionAffordance {
    #[serde(flatten)]
    pub interaction_affordance: InteractionAffordance,

    pub input: Option<DataSchema>,
    pub output: Option<DataSchema>,
    pub safe: Option<bool>,
    pub idempotent: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventAffordance {
    #[serde(flatten)]
    pub interaction_affordance: InteractionAffordance,

    pub subscription: Option<DataSchema>,
    pub data: Option<DataSchema>,
    pub cancellation: Option<DataSchema>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OperationType {
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
pub struct Form {
    // TODO: Define forms for different affordance types
    pub op: Option<TypeOrTypeArray<OperationType>>,
    pub href: String,
    pub content_type: Option<String>,
    pub content_coding: Option<String>,
    pub subprotocol: Option<String>,
    pub security: Option<TypeOrTypeArray<String>>,
    pub scopes: Option<TypeOrTypeArray<String>>,
    pub response: Option<ExpectedResponse>,
    pub additional_responses: Option<TypeOrTypeArray<AdditionalExpectedResponse>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalExpectedResponse {
    pub success: Option<bool>,
    pub schema: Option<String>,
    pub content_type: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: String,
    pub r#type: Option<String>,
    pub rel: Option<String>,
    pub anchor: Option<String>,
    pub sizes: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpectedResponse {
    pub content_type: String,
}
