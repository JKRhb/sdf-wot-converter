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

#[derive(Debug, Deserialize, Serialize)]
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
    instance: String,
    model: Option<String>,
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
    r#type: Option<TypeOrTypeArray<String>>,
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
pub struct InteractionAffordance {
    #[serde(rename = "@type")]
    r#type: Option<TypeOrTypeArray<String>>,
    title: Option<String>,
    titles: Option<HashMap<String, String>>,
    description: Option<String>,
    descriptions: Option<HashMap<String, String>>,
    forms: Vec<Form>,
    uri_variables: Option<HashMap<String, DataSchema>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyAffordance {
    #[serde(flatten)]
    interaction_affordance: InteractionAffordance,

    #[serde(flatten)]
    data_schema: DataSchema,
    observable: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionAffordance {
    #[serde(flatten)]
    interaction_affordance: InteractionAffordance,

    input: Option<DataSchema>,
    output: Option<DataSchema>,
    safe: Option<bool>,
    idempotent: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventAffordance {
    #[serde(flatten)]
    interaction_affordance: InteractionAffordance,

    subscription: Option<DataSchema>,
    data: Option<DataSchema>,
    cancellation: Option<DataSchema>,
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
    op: Option<TypeOrTypeArray<OperationType>>,
    href: String,
    content_type: Option<String>,
    content_coding: Option<String>,
    subprotocol: Option<String>,
    security: Option<TypeOrTypeArray<String>>,
    scopes: Option<TypeOrTypeArray<String>>,
    response: Option<ExpectedResponse>,
    additional_responses: Option<TypeOrTypeArray<AdditionalExpectedResponse>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalExpectedResponse {
    success: Option<bool>,
    schema: Option<String>,
    content_type: Option<String>,
}


#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    href: String,
    r#type: Option<String>,
    rel: Option<String>,
    anchor: Option<String>,
    sizes: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpectedResponse {
    content_type: String,
}

