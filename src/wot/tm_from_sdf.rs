use super::definitions as wot;
use crate::sdf::definitions as sdf;
use std::collections::HashMap;

impl From<sdf::SDFModel> for wot::ThingModel {
    fn from(sdf_model: sdf::SDFModel) -> Self {
        let mut context_entries: Vec<wot::ContextEntry> = vec![wot::ContextEntry::String(
            "https://www.w3.org/2019/wot/td/v1".to_string(),
        )];
        let links;

        let infoblock: &Option<sdf::InfoBlock> = &sdf_model.info;
        let title: Option<String>;
        let description: Option<String>;
        let version: Option<wot::VersionInfo>;
        match infoblock {
            None => {
                title = None;
                version = None;
                description = None;
                links = None;
            }
            Some(infoblock) => {
                title = Some(infoblock.title.clone());
                let link_fields = wot::Link {
                    rel: Some("license".to_string()),
                    r#type: None,
                    anchor: None,
                    sizes: None,
                };
                version = Some(wot::VersionInfo {
                    // TODO: Revisit use of "instance" and "model"
                    instance: infoblock.version.clone(),
                    model: None,
                });
                description = Some(infoblock.copyright.clone());
                links = Some(vec![wot::TMLink {
                    link_fields,
                    href: Some(infoblock.license.clone()),
                }]);
            }
        };

        if let Some(x) = sdf_model.namespace.clone() {
            context_entries.push(wot::ContextEntry::Map(x));
        };

        let base_thing = wot::BaseThing {
            context: wot::Context::Array(context_entries),
            description,
            version,

            // Not covered by SDF yet:
            r#type: None,
            titles: None,
            descriptions: None,
            id: None,
            modified: None,
            profile: None,
            schema_definitions: None,
            base: None,
            created: None,
            support: None,
        };

        wot::ThingModel {
            base_thing,

            title,
            actions: convert_actions(&sdf_model),
            properties: convert_properties(&sdf_model),
            events: convert_events(&sdf_model),
            links,

            forms: None,
            security: None,
            security_definitions: None,
        }
    }
}

fn first_letter_to_uppper_case(s1: &str) -> String {
    let mut c = s1.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn get_prefixed_key(prefix: Option<String>, key: String) -> String {
    match prefix {
        Some(prefix) => {
            let capitalized_affordance_name = first_letter_to_uppper_case(&key);
            format!("{}{}", prefix, capitalized_affordance_name)
        }
        None => key,
    }
}

fn create_interaction_affordance(
    common_qualities: &sdf::CommonQualities,
) -> wot::TMInteractionAffordance {
    let title: Option<String> = common_qualities.label.clone();
    let description: Option<String> = common_qualities.description.clone();

    let interaction_affordance_fields = wot::InteractionAffordance {
        title,
        description,

        titles: None,
        descriptions: None,
        r#type: None,
        uri_variables: None,
    };

    wot::TMInteractionAffordance {
        interaction_affordance_fields,
        forms: None,
    }
}

fn convert_action(sdf_action: &sdf::ActionQualities) -> wot::TMActionAffordance {
    let input;
    match &sdf_action.sdf_input_data {
        None => input = None,
        Some(input_data) => {
            input = Some(convert_to_data_schema(&input_data));
        }
    };

    let output;
    match &sdf_action.sdf_output_data {
        None => output = None,
        Some(output_data) => {
            output = Some(convert_to_data_schema(&output_data));
        }
    };

    let action_affordance_fields = wot::ActionAffordance {
        input,
        output,
        safe: None,
        idempotent: None,
    };

    wot::TMActionAffordance {
        action_affordance_fields,
        interaction_affordance: create_interaction_affordance(&sdf_action.common_qualities),
    }
}

fn convert_actions(sdf_model: &sdf::SDFModel) -> Option<HashMap<String, wot::TMActionAffordance>> {
    let mut actions_map: HashMap<String, wot::TMActionAffordance> = HashMap::new();

    convert_sdf_actions(&sdf_model, &mut actions_map, &sdf_model.sdf_action, None);
    convert_sdf_object_actions(&sdf_model, &mut actions_map, &sdf_model.sdf_object, None);
    convert_sdf_thing_actions(&sdf_model, &mut actions_map, &sdf_model.sdf_thing, None);

    if !actions_map.is_empty() {
        Some(actions_map)
    } else {
        None
    }
}

fn map_regular_type(sdf_type: &sdf::RegularTypes) -> Option<wot::JSONSchemaTypes> {
    match sdf_type {
        sdf::RegularTypes::Number(sdf_schema) => {
            let mapping = wot::JSONSchemaTypes::Number(wot::NumberSchema::<f32> {
                minimum: sdf_schema.minimum,
                exclusive_minimum: sdf_schema.exclusive_minimum,
                maximum: sdf_schema.maximum,
                exclusive_maximum: sdf_schema.exclusive_maximum,
                multiple_of: sdf_schema.multiple_of,
            });
            Some(mapping)
        }
        sdf::RegularTypes::Integer(sdf_schema) => {
            let mapping = wot::JSONSchemaTypes::Integer(wot::NumberSchema::<i32> {
                minimum: sdf_schema.minimum,
                exclusive_minimum: sdf_schema.exclusive_minimum,
                maximum: sdf_schema.maximum,
                exclusive_maximum: sdf_schema.exclusive_maximum,
                multiple_of: sdf_schema.multiple_of,
            });
            Some(mapping)
        }
        sdf::RegularTypes::String(sdf_schema) => {
            let mapping = wot::JSONSchemaTypes::String(wot::StringSchema {
                // TODO: format is not mapped yet
                min_length: sdf_schema.min_length,
                max_length: sdf_schema.max_length,
                pattern: sdf_schema.pattern.clone(),
                content_encoding: None,
                content_media_type: None,
            });
            Some(mapping)
        }
        sdf::RegularTypes::Array(sdf_schema) => {
            // TODO: Can SDF arrays only specify one data quality?
            let items;
            if let Some(array_items) = &sdf_schema.items {
                let data_schema = convert_to_data_schema(&array_items);
                items = Some(Box::new(wot::TypeOrTypeArray::Type::<wot::DataSchema>(
                    data_schema,
                )));
            } else {
                items = None;
            }

            let mapping = wot::JSONSchemaTypes::Array(wot::ArraySchema {
                // TODO: Can unique_items be mapped?
                min_items: sdf_schema.min_items,
                max_items: sdf_schema.max_items,
                items,
            });
            Some(mapping)
        }
        sdf::RegularTypes::Object(sdf_schema) => {
            let properties;
            let mut wot_properties: HashMap<String, wot::DataSchema> = HashMap::new();
            if let Some(sdf_properties) = &sdf_schema.properties {
                for (key, data_quality) in sdf_properties {
                    let data_schema = convert_to_data_schema(&data_quality);
                    wot_properties.insert(key.clone(), data_schema);
                }
            }

            if !wot_properties.is_empty() {
                properties = Some(wot_properties);
            } else {
                properties = None;
            }

            let mapping = wot::JSONSchemaTypes::Object(wot::ObjectSchema {
                required: sdf_schema.required.clone(),
                properties,
            });
            Some(mapping)
        }
        sdf::RegularTypes::Boolean(_) => Some(wot::JSONSchemaTypes::Boolean),
    }
}

fn map_data_type(jsonschema: &Option<sdf::Types>) -> Option<wot::JSONSchemaTypes> {
    match jsonschema {
        None => None,
        Some(jsonschema_type) => match jsonschema_type {
            sdf::Types::Type(regular_type) => map_regular_type(regular_type),
            sdf::Types::SdfChoice(_) => {
                // TODO: How should sdfChoice be covered?
                None
            }
        },
    }
}

/// Maps SDF's `readable` and `writable` to WoT TD's `writeOnly` and `readOnly`.
///
/// # Return value
///
/// A boolean tuple. The first value represents `writeOnly`, the second one `readOnly`.
fn map_readable_writable(
    readable: Option<bool>,
    writable: Option<bool>,
) -> (Option<bool>, Option<bool>) {
    let write_only = readable.map(|x| !x);
    let read_only = writable.map(|x| !x);
    (write_only, read_only)
}

fn convert_to_data_schema(sdf_property: &sdf::DataQualities) -> wot::DataSchema {
    // TODO: How should nullable be mapped?
    let (write_only, read_only) =
        map_readable_writable(sdf_property.readable, sdf_property.writable);

    wot::DataSchema {
        write_only,
        read_only,

        r#enum: None,  // Still TODO
        r#const: None, // Still TODO
        data_type: map_data_type(&sdf_property.jsonschema),
        one_of: None, // TODO: Can this be mapped using sdfChoice?

        unit: sdf_property.unit.clone(), // TODO: Check if this kind of mapping is appropriate

        title: None,       // Set to None to avoid duplication
        description: None, // Set to None to avoid duplication
        titles: None,
        descriptions: None,
        format: None, // TODO: Can this be mapped?
        r#type: None,
    }
}

fn convert_property(sdf_property: &sdf::PropertyQualities) -> wot::TMPropertyAffordance {
    // TODO: How should contentFormat be mapped?

    // TODO: Refactor as sdfProperty is an alias for sdfData
    let property_affordance_fields = wot::PropertyAffordance {
        observable: sdf_property.observable,

        data_schema: convert_to_data_schema(sdf_property),
    };

    wot::TMPropertyAffordance {
        property_affordance_fields,
        interaction_affordance: create_interaction_affordance(&sdf_property.common_qualities),
    }
}

fn convert_properties(
    sdf_model: &sdf::SDFModel,
) -> Option<HashMap<String, wot::TMPropertyAffordance>> {
    let mut properties: HashMap<String, wot::TMPropertyAffordance> = HashMap::new();

    convert_sdf_properties(&sdf_model, &mut properties, &sdf_model.sdf_property, None);
    convert_sdf_object_properties(&sdf_model, &mut properties, &sdf_model.sdf_object, None);
    convert_sdf_thing_properties(&sdf_model, &mut properties, &sdf_model.sdf_thing, None);

    if !properties.is_empty() {
        Some(properties)
    } else {
        None
    }
}

macro_rules! create_object_conversion_function {
    ($wot_type:ty, $sdf_type:ty, $function_name:ident, $function_call:ident, $field_name:ident) => {
        fn $function_name(
            _sdf_model: &sdf::SDFModel, // Might be used later for resolving references
            wot_definitions: &mut HashMap<String, $wot_type>,
            sdf_definitions: &Option<HashMap<String, $sdf_type>>,
            prefix: Option<String>,
        ) -> () {
            match sdf_definitions {
                Some(sdf_definitions) => {
                    for (key, value) in sdf_definitions {
                        let prefixed_key = get_prefixed_key(prefix.clone(), key.to_string());

                        $function_call(
                            _sdf_model,
                            wot_definitions,
                            &value.$field_name,
                            Some(prefixed_key),
                        );
                    }
                }
                None => (),
            }
        }
    };
}

macro_rules! create_affordance_conversion_function {
    ($wot_type:ty, $sdf_type:ty, $function_name:ident, $function_call:ident) => {
        fn $function_name(
            _sdf_model: &sdf::SDFModel, // Might be used later for resolving references
            wot_definitions: &mut HashMap<String, $wot_type>,
            sdf_definitions: &Option<HashMap<String, $sdf_type>>,
            prefix: Option<String>,
        ) -> () {
            match sdf_definitions {
                Some(sdf_definitions) => {
                    for (key, value) in sdf_definitions {
                        let prefixed_key = get_prefixed_key(prefix.clone(), key.to_string());
                        wot_definitions.insert(prefixed_key, $function_call(&value));
                    }
                }
                None => (),
            }
        }
    };
}

create_affordance_conversion_function!(
    wot::TMPropertyAffordance,
    sdf::PropertyQualities,
    convert_sdf_properties,
    convert_property
);
create_affordance_conversion_function!(
    wot::TMActionAffordance,
    sdf::ActionQualities,
    convert_sdf_actions,
    convert_action
);
create_affordance_conversion_function!(
    wot::TMEventAffordance,
    sdf::EventQualities,
    convert_sdf_events,
    convert_event
);
create_object_conversion_function!(
    wot::TMPropertyAffordance,
    sdf::ObjectQualities,
    convert_sdf_object_properties,
    convert_sdf_properties,
    sdf_property
);
create_object_conversion_function!(
    wot::TMActionAffordance,
    sdf::ObjectQualities,
    convert_sdf_object_actions,
    convert_sdf_actions,
    sdf_action
);
create_object_conversion_function!(
    wot::TMEventAffordance,
    sdf::ObjectQualities,
    convert_sdf_object_events,
    convert_sdf_events,
    sdf_event
);

macro_rules! create_thing_conversion_function {
    ($wot_type:ty, $function_name:ident, $object_function:ident) => {
        fn $function_name(
            _sdf_model: &sdf::SDFModel, // Might be used later for resolving references
            wot_definitions: &mut HashMap<String, $wot_type>,
            sdf_definitions: &Option<HashMap<String, sdf::ThingQualities>>,
            prefix: Option<String>,
        ) -> () {
            match sdf_definitions {
                None => (),
                Some(sdf_definitions) => {
                    for (key, value) in sdf_definitions {
                        let prefixed_key = get_prefixed_key(prefix.clone(), key.to_string());

                        $function_name(
                            _sdf_model,
                            wot_definitions,
                            &value.sdf_thing,
                            Some(prefixed_key.clone()),
                        );
                        $object_function(
                            _sdf_model,
                            wot_definitions,
                            &value.sdf_object,
                            Some(prefixed_key),
                        );
                    }
                }
            }
        }
    };
}

create_thing_conversion_function!(
    wot::TMActionAffordance,
    convert_sdf_thing_actions,
    convert_sdf_object_actions
);
create_thing_conversion_function!(
    wot::TMPropertyAffordance,
    convert_sdf_thing_properties,
    convert_sdf_object_properties
);
create_thing_conversion_function!(
    wot::TMEventAffordance,
    convert_sdf_thing_events,
    convert_sdf_object_events
);

fn convert_event(sdf_event: &sdf::EventQualities) -> wot::TMEventAffordance {
    // TODO: How should sdf_data be mapped?
    let data = sdf_event
        .sdf_output_data
        .as_ref()
        .map(|output_data| convert_to_data_schema(&output_data));

    let event_affordance_fields = wot::EventAffordance {
        subscription: None, // Still TODO
        data,
        cancellation: None, // Still TODO
    };

    wot::TMEventAffordance {
        event_affordance_fields,
        interaction_affordance: create_interaction_affordance(&sdf_event.common_qualities),
    }
}

fn convert_events(sdf_model: &sdf::SDFModel) -> Option<HashMap<String, wot::TMEventAffordance>> {
    let mut events: HashMap<String, wot::TMEventAffordance> = HashMap::new();

    convert_sdf_events(&sdf_model, &mut events, &sdf_model.sdf_event, None);
    convert_sdf_object_events(&sdf_model, &mut events, &sdf_model.sdf_object, None);
    convert_sdf_thing_events(&sdf_model, &mut events, &sdf_model.sdf_thing, None);

    if !events.is_empty() {
        Some(events)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_letter_to_uppper_case_test() {
        assert_eq!(first_letter_to_uppper_case(""), "");
        assert_eq!(first_letter_to_uppper_case("a"), "A");
        assert_eq!(first_letter_to_uppper_case("A"), "A");
        assert_eq!(first_letter_to_uppper_case("ab"), "Ab");
        assert_eq!(first_letter_to_uppper_case("Ab"), "Ab");
        assert_eq!(first_letter_to_uppper_case("aB"), "AB");
        assert_eq!(first_letter_to_uppper_case("AB"), "AB");
    }

    #[test]
    fn get_prefixed_key_test() {
        let key = "bar".to_string();
        let prefix = "foo".to_string();
        let prefixed_key = "fooBar".to_string();

        assert_eq!(get_prefixed_key(None, key.clone()), key);
        assert_eq!(get_prefixed_key(Some(prefix), key.clone()), prefixed_key);
    }

    #[test]
    fn map_readable_writable_test() {
        let read_only_1 = map_readable_writable(None, Some(false));
        let expected_read_only_result_1 = (None, Some(true));
        assert_eq!(read_only_1, expected_read_only_result_1);
        let read_only_2 = map_readable_writable(Some(true), Some(false));
        let expected_read_only_result_2 = (Some(false), Some(true));
        assert_eq!(read_only_2, expected_read_only_result_2);

        let write_only_1 = map_readable_writable(Some(false), None);
        let expected_write_only_result_1 = (Some(true), None);
        assert_eq!(write_only_1, expected_write_only_result_1);
        let write_only_2 = map_readable_writable(Some(false), Some(true));
        let expected_write_only_result_2 = (Some(true), Some(false));
        assert_eq!(write_only_2, expected_write_only_result_2);
    }
}
