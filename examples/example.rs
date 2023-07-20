use std::{collections::HashMap, convert::TryFrom, fmt::Debug};

use jsonptr::Pointer;
use serde::Serialize;
use serde_json::{json, Value};

type SdfModel = serde_json::Map<String, Value>;
type ThingDescription = serde_json::Map<String, Value>;
type ThingModel = serde_json::Map<String, Value>;

type SdfDataqualities = serde_json::Map<String, Value>;
type WotDataSchema = serde_json::Map<String, Value>;

macro_rules! map_schema {
    ($key: expr, $input: expr, $output: expr) => {
        if let Some(data_type) = $input.get($key) {
            $output.insert($key.to_owned(), data_type.clone());
        }
    };
}

pub trait ToDataSchema {
    fn to_wot_data_schema(&self) -> WotDataSchema;
}

impl ToDataSchema for SdfDataqualities {
    fn to_wot_data_schema(&self) -> WotDataSchema {
        let output = map_common_schema_fields(self);

        output
    }
}

fn map_common_schema_fields(
    input: &serde_json::Map<String, Value>,
) -> serde_json::Map<String, Value> {
    let mut output = serde_json::Map::<String, Value>::new();

    map_schema!("type", input, output);
    map_schema!("unit", input, output);
    map_schema!("const", input, output);
    map_schema!("default", input, output);
    map_schema!("multipleOf", input, output);
    map_schema!("minimum", input, output);
    map_schema!("maximum", input, output);
    map_schema!("required", input, output);
    map_schema!("pattern", input, output);
    map_schema!("format", input, output);
    map_schema!("exclusiveMinimum", input, output);
    map_schema!("exclusiveMaximum", input, output);
    map_schema!("minItems", input, output);
    map_schema!("maxItems", input, output);
    map_schema!("minLength", input, output);
    map_schema!("maxLength", input, output);

    output.to_owned()
}

pub fn main() {
    let mut data = json!({"foo": { "bar": "baz" }});
    // let ptr = Pointer::new(["foo", "bar"]);
    let ptr = Pointer::try_from("#/foo/bar").unwrap();
    println!("{}", ptr.resolve(&data).unwrap());

    let data_qualities = json!({"type": "string", "exclusiveMaximum": 5});

    if let serde_json::Value::Object(object) = data_qualities {
        let data_schema = object.to_wot_data_schema();

        println!("{}", data_schema.get("type").unwrap());
        println!("{}", data_schema.get("exclusiveMaximum").unwrap());
    }
}
