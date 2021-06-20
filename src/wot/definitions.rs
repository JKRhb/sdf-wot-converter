use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

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
enum TypeOrTypeArray<T> {
    String(T),
    Array(Vec<T>),
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Thing {
    #[serde(rename = "@context")]
    context: Context,
    #[serde(rename = "@type")]
    r#type: Option<TypeOrTypeArray<String>>,
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
    actions: Option<HashMap<String, ActionAffordance>>,
    properties: Option<HashMap<String, PropertyAffordance>>,
    events: Option<HashMap<String, EventAffordance>>,
    links: Option<Vec<Link>>,
    forms: Option<Vec<Form>>,
    security: TypeOrTypeArray<String>,
    security_definitions: HashMap<String, SecurityScheme>,
    profile: Option<TypeOrTypeArray<String>>,
    schema_definitions: Option<HashMap<String, DataSchema>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum SchemeIn {
    Header,
    Query,
    Body,
    Cookie,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
enum SchemaQoP {
    Auth,
    AuthInit,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "scheme")]
enum SecurityScheme {
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
struct SecuritySchemeCommon {
    #[serde(rename = "@type")]
    r#type: Option<TypeOrTypeArray<String>>,
    description: Option<String>,
    descriptions: Option<HashMap<String, String>>,
    proxy: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum SecurityDefinition {
    NoSecurityScheme(String),
    Array(Vec<ContextEntry>),
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
struct InteractionAffordance {
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
struct PropertyAffordance {
    #[serde(flatten)]
    interaction_affordance: InteractionAffordance,

    #[serde(flatten)]
    data_schema: DataSchema,
    observable: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ActionAffordance {
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
struct EventAffordance {
    #[serde(flatten)]
    interaction_affordance: InteractionAffordance,

    subscription: Option<DataSchema>,
    data: Option<DataSchema>,
    cancellation: Option<DataSchema>,
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
    security: Option<TypeOrTypeArray<String>>,
    scopes: Option<TypeOrTypeArray<String>>,
    response: Option<ExpectedResponse>,
    additional_responses: Option<TypeOrTypeArray<AdditionalExpectedResponse>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AdditionalExpectedResponse {
    success: Option<bool>,
    schema: Option<String>,
    content_type: Option<String>,
}


#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Link {
    href: String,
    r#type: Option<String>,
    rel: Option<String>,
    anchor: Option<String>,
    sizes: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ExpectedResponse {
    content_type: String,
}
