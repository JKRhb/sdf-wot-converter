use super::definitions as sdf;
use crate::wot::definitions as wot;
use std::collections::HashMap;

/// Creates an info block from a Thing Model. I am a bit unsure how to map a
/// TM that has not been an SDF model before therefore this function only
/// returns `None` at the moment.
///
/// TODO: Investigate how to map this.
fn create_info_block(thing_model: &wot::ThingModel) -> Option<sdf::InfoBlock> {
    let title = thing_model.title.clone();
    let version = None;
    let copyright = None;
    let license = None;

    match (title, copyright, license, version) {
        (Some(title), Some(copyright), Some(license), Some(version)) => Some(sdf::InfoBlock {
            title,
            version,
            copyright,
            license,
        }),
        _ => None,
    }
}

/// Converts the @context of a Thing Model to SDF namespaces. Context entries that
/// only consist of a single URI are ignored for now as it is unclear how to map them.
fn convert_namespaces(context: &wot::Context) -> Option<HashMap<String, String>> {
    let mut namespaces: HashMap<String, String> = HashMap::new();
    if let wot::Context::Array(array_context) = context {
        for context_entry in array_context {
            if let wot::ContextEntry::Map(map_entry) = context_entry {
                for (key, value) in map_entry {
                    namespaces.insert(key.clone(), value.clone());
                }
            }
        }
    }

    if !namespaces.is_empty() {
        Some(namespaces)
    } else {
        None
    }
}

/// Converts a WoT Thing Model into an SDF model.
impl From<wot::ThingModel> for sdf::SDFModel {
    fn from(thing_model: wot::ThingModel) -> Self {
        let info = create_info_block(&thing_model);
        let namespace = convert_namespaces(&thing_model.base_thing.context);
        let default_namespace = None;
        let sdf_thing = None;
        let sdf_product = None;
        let sdf_object = None;
        let sdf_property = create_qualities::<wot::TMPropertyAffordance, sdf::PropertyQualities>(
            thing_model.properties,
        );
        let sdf_action =
            create_qualities::<wot::TMActionAffordance, sdf::ActionQualities>(thing_model.actions);
        let sdf_event =
            create_qualities::<wot::TMEventAffordance, sdf::EventQualities>(thing_model.events);
        let sdf_data = None;

        sdf::SDFModel {
            info,
            namespace,
            default_namespace,
            sdf_thing,
            sdf_product,
            sdf_object,
            sdf_property,
            sdf_action,
            sdf_event,
            sdf_data,
        }
    }
}

fn create_qualities<T, U: From<T>>(
    wot_definitions: Option<HashMap<String, T>>,
) -> Option<HashMap<String, U>> {
    let mut sdf_definitions: HashMap<String, U> = HashMap::new();

    if let Some(wot_affordances) = wot_definitions {
        for (key, wot_affordance) in wot_affordances {
            sdf_definitions.insert(key.clone(), U::from(wot_affordance));
        }
    }

    if !sdf_definitions.is_empty() {
        Some(sdf_definitions)
    } else {
        None
    }
}

impl From<wot::TMInteractionAffordance> for sdf::CommonQualities {
    fn from(interaction_affordance: wot::TMInteractionAffordance) -> Self {
        // TODO: Map missing fields
        let interaction_affordance_fields = interaction_affordance.interaction_affordance_fields;
        let description = interaction_affordance_fields.description.clone();
        let label = interaction_affordance_fields.title;

        sdf::CommonQualities {
            comment: None,
            description,
            label,
            sdf_ref: None,
            sdf_required: None,
        }
    }
}

impl From<&wot::DataSchema> for sdf::CommonQualities {
    fn from(data_schema: &wot::DataSchema) -> Self {
        // TODO: Map missing fields
        let description = data_schema.description.clone();
        let label = data_schema.title.clone();

        sdf::CommonQualities {
            comment: None,
            description,
            label,
            sdf_ref: None,
            sdf_required: None,
        }
    }
}

fn map_data_type(data_schema: &wot::DataSchema) -> Option<sdf::Types> {
    if let Some(json_schema_type) = &data_schema.data_type {
        match json_schema_type {
            wot::JSONSchemaTypes::Null => None,
            wot::JSONSchemaTypes::Boolean => {
                let typed_qualities = sdf::TypedQualities::<bool> {
                    r#enum: data_schema
                        .r#enum
                        .as_ref()
                        .and_then(|x| x.iter().map(serde_json::value::Value::as_bool).collect()),
                    r#const: data_schema
                        .r#const
                        .as_ref()
                        .and_then(serde_json::value::Value::as_bool),
                    default: data_schema
                        .default
                        .as_ref()
                        .and_then(serde_json::value::Value::as_bool),
                };

                let boolean_type = sdf::RegularTypes::Boolean(typed_qualities);
                Some(sdf::Types::Type(boolean_type))
            }
            wot::JSONSchemaTypes::Integer(integer_schema) => {
                let common_qualities = sdf::TypedQualities::<i64> {
                    r#enum: data_schema
                        .r#enum
                        .as_ref()
                        .and_then(|x| x.iter().map(serde_json::value::Value::as_i64).collect()),
                    r#const: data_schema
                        .r#const
                        .as_ref()
                        .and_then(serde_json::value::Value::as_i64),
                    default: data_schema
                        .default
                        .as_ref()
                        .and_then(serde_json::value::Value::as_i64),
                };

                let typed_qualities = sdf::NumberTypeQualities::<i64> {
                    minimum: integer_schema.minimum,
                    maximum: integer_schema.maximum,
                    exclusive_maximum: integer_schema.exclusive_maximum,
                    exclusive_minimum: integer_schema.exclusive_minimum,
                    multiple_of: integer_schema.multiple_of,
                    common_qualities,
                };

                let integer_type = sdf::RegularTypes::Integer(typed_qualities);
                Some(sdf::Types::Type(integer_type))
            }
            wot::JSONSchemaTypes::Number(number_schema) => {
                let common_qualities = sdf::TypedQualities::<f64> {
                    r#enum: data_schema
                        .r#enum
                        .as_ref()
                        .and_then(|x| x.iter().map(serde_json::value::Value::as_f64).collect()),
                    r#const: data_schema
                        .r#const
                        .as_ref()
                        .and_then(serde_json::value::Value::as_f64),
                    default: data_schema
                        .default
                        .as_ref()
                        .and_then(serde_json::value::Value::as_f64),
                };

                let typed_qualities = sdf::NumberTypeQualities::<f64> {
                    minimum: number_schema.minimum,
                    maximum: number_schema.maximum,
                    exclusive_maximum: number_schema.exclusive_maximum,
                    exclusive_minimum: number_schema.exclusive_minimum,
                    multiple_of: number_schema.multiple_of,
                    common_qualities,
                };

                let number_type = sdf::RegularTypes::Number(typed_qualities);
                Some(sdf::Types::Type(number_type))
            }
            wot::JSONSchemaTypes::String(string_schema) => {
                let common_qualities = sdf::TypedQualities::<String> {
                    r#enum: data_schema
                        .r#enum
                        .as_ref()
                        .and_then(|x| x.iter().map(|y| y.as_str().map(String::from)).collect()),
                    r#const: data_schema
                        .r#const
                        .as_ref()
                        .and_then(|x| serde_json::value::Value::as_str(x).map(String::from)),
                    default: data_schema
                        .default
                        .as_ref()
                        .and_then(|x| serde_json::value::Value::as_str(x).map(String::from)),
                };

                let typed_qualities = sdf::StringTypeQualities {
                    common_qualities,
                    min_length: string_schema.min_length,
                    max_length: string_schema.max_length,
                    pattern: string_schema.pattern.clone(),
                    format: None, // TODO: How to map to FormatQualities?,
                };

                let string_type = sdf::RegularTypes::String(typed_qualities);
                Some(sdf::Types::Type(string_type))
            }
            wot::JSONSchemaTypes::Array(array_schema) => {
                let items;
                let mut array_items: Vec<sdf::DataQualities> = Vec::new();

                if let Some(wot_items) = &array_schema.items {
                    match &**wot_items {
                        wot::TypeOrTypeArray::Type::<wot::DataSchema>(array_type) => {
                            array_items.push(sdf::DataQualities::from(array_type));
                        }
                        wot::TypeOrTypeArray::Array::<wot::DataSchema>(array) => {
                            for item in array {
                                array_items.push(sdf::DataQualities::from(item));
                            }
                        }
                    }
                };

                if array_items.is_empty() {
                    items = None;
                } else {
                    items = Some(array_items);
                }

                let typed_qualities = sdf::ArrayTypeQualities {
                    min_items: array_schema.min_items,
                    max_items: array_schema.max_items,
                    unique_items: None, // TODO: Is there an equivalent in WoT?
                    items,              // TODO: Should this be an array or a map?
                };

                let array_type = sdf::RegularTypes::Array(typed_qualities);
                Some(sdf::Types::Type(array_type))
            }
            wot::JSONSchemaTypes::Object(object_schema) => {
                let common_qualities = sdf::TypedQualities::<HashMap<String, serde_json::Value>> {
                    // TODO: How do you map this?
                    r#const: None,
                    default: None,
                    r#enum: None,
                };
                let properties;
                let mut properties_map: HashMap<String, sdf::DataQualities> = HashMap::new();

                if let Some(wot_properties) = &object_schema.properties {
                    for (key, data_schema) in wot_properties {
                        properties_map.insert(key.clone(), sdf::DataQualities::from(data_schema));
                    }
                }

                if !properties_map.is_empty() {
                    properties = Some(properties_map);
                } else {
                    properties = None;
                }

                let typed_qualities = sdf::ObjectTypeQualities {
                    common_qualities,
                    required: object_schema.required.clone(),
                    properties,
                };

                let object_type = sdf::RegularTypes::Object(typed_qualities);
                Some(sdf::Types::Type(object_type))
            }
        }
    } else {
        None
    }
}

impl From<&wot::DataSchema> for sdf::DataQualities {
    fn from(data_schema: &wot::DataSchema) -> Self {
        let unit = data_schema.unit.clone();
        let writable = data_schema.read_only.map(|x| !x);
        let readable = data_schema.write_only.map(|x| !x);
        let jsonschema = map_data_type(data_schema);

        // TODO: Unmapped fields: @type, titles, descriptions, one_of, format
        // TODO: Check how type of enum, const, and default should be handled

        sdf::DataQualities {
            common_qualities: sdf::CommonQualities::from(data_schema),
            jsonschema,
            unit,
            observable: None,
            readable,
            writable,
            nullable: None,
            sdf_type: None,
            content_format: None,
        }
    }
}

impl From<wot::TMActionAffordance> for sdf::ActionQualities {
    fn from(action_affordance: wot::TMActionAffordance) -> Self {
        let common_qualities = sdf::CommonQualities::from(action_affordance.interaction_affordance);
        let action_affordance_fields = action_affordance.action_affordance_fields;
        let sdf_input_data = action_affordance_fields
            .input
            .as_ref()
            .map(sdf::DataQualities::from);
        let sdf_output_data = action_affordance_fields
            .output
            .as_ref()
            .map(sdf::DataQualities::from);

        sdf::ActionQualities {
            common_qualities,
            sdf_input_data,
            sdf_output_data,
            sdf_data: None,
        }
    }
}

impl From<wot::TMPropertyAffordance> for sdf::PropertyQualities {
    fn from(property_affordance: wot::TMPropertyAffordance) -> Self {
        let property_affordance_fields = property_affordance.property_affordance_fields;
        let mut property_qualities =
            sdf::DataQualities::from(&property_affordance_fields.data_schema);
        property_qualities.observable = property_affordance_fields.observable;
        property_qualities
    }
}

impl From<wot::TMEventAffordance> for sdf::EventQualities {
    fn from(event_affordance: wot::TMEventAffordance) -> Self {
        // TODO: What to do with subscription and cancellation?
        let common_qualities = sdf::CommonQualities::from(event_affordance.interaction_affordance);
        let event_affordance_fields = event_affordance.event_affordance_fields;
        let sdf_output_data = event_affordance_fields
            .data
            .as_ref()
            .map(sdf::DataQualities::from);

        sdf::EventQualities {
            common_qualities,
            sdf_output_data,
            sdf_data: None, // TODO: How should this be mapped?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_namespaces_string_test() {
        let string_context = wot::Context::String("foobar".to_string());
        let string_array_context =
            wot::Context::Array(vec![wot::ContextEntry::String("foobar".to_string())]);

        assert!(convert_namespaces(&string_context).is_none());
        assert!(convert_namespaces(&string_array_context).is_none());
    }

    #[test]
    fn convert_namespaces_map_test() {
        let context_map: HashMap<String, String> = vec![("foo".to_string(), "bar".to_string())]
            .into_iter()
            .collect();
        let map_array_context = wot::Context::Array(vec![wot::ContextEntry::Map(context_map)]);
        let expected_namespace: Option<HashMap<String, String>> = Some(
            vec![("foo".to_string(), "bar".to_string())]
                .into_iter()
                .collect(),
        );

        let converted_namespace = convert_namespaces(&map_array_context);
        assert!(converted_namespace.is_some());
        assert_eq!(converted_namespace, expected_namespace);
    }
}
