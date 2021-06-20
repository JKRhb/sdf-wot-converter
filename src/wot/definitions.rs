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
enum Type {
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
    r#type: Option<Type>,
    id: Option<String>,
    title: String,
    titles: Option<HashMap<String, String>>,
    description: Option<String>,
    descriptions: Option<HashMap<String, String>>,
    version: Option<String>,
    created: Option<DateTime<Utc>>,
    modified: Option<DateTime<Utc>>,
    support: Option<String>,
    base: Option<String>,
    // actions: Option<HashMap<String, ActionAffordance>>,
    // properties: Option<HashMap<String, PropertyAffordance>>,
    // events: Option<HashMap<String, EventAffordance>>,
    // links: Option<Vec<Link>>,
    // forms: Option<Vec<Form>>,
    // security: Vec<Security>,
    // security_definitions: HashMap<String, SecurityDefinition>,
    // profile: Option<Profile>,
    // schema_definitions: Option<HashMap<String, DataSchema>>
}


