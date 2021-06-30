//! First attempt for a SDF to WoT converter.

use crate::sdf::definitions as sdf;
use crate::wot::definitions as wot;
use std::collections::HashMap;

fn first_letter_to_uppper_case(s1: &String) -> String {
    let mut c = s1.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn get_prefixed_key(prefix: &Option<String>, key: String) -> String {
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
) -> wot::InteractionAffordance {
    let title: Option<String> = common_qualities.label.clone();
    let description: Option<String> = common_qualities.description.clone();

    wot::InteractionAffordance {
        title,
        description,

        forms: Vec::<wot::Form>::new(),
        titles: None,
        descriptions: None,
        r#type: None,
        uri_variables: None,
    }
}

fn convert_action(sdf_action: &sdf::ActionQualities) -> wot::ActionAffordance {
    wot::ActionAffordance {
        // TODO: Deal with input and output data
        input: None,
        output: None,
        safe: None,
        idempotent: None,

        interaction_affordance: create_interaction_affordance(&sdf_action.common_qualities),
    }
}

fn convert_actions(sdf_model: &sdf::SDFModel) -> Option<HashMap<String, wot::ActionAffordance>> {
    let mut actions_map: HashMap<String, wot::ActionAffordance> = HashMap::new();

    convert_sdf_actions(&sdf_model, &mut actions_map, &None, &None);
    convert_sdf_object_actions(&sdf_model, &mut actions_map, &sdf_model.sdf_object, &None);

    if actions_map.len() > 0 {
        Some(actions_map)
    } else {
        None
    }
}

fn convert_property(sdf_property: &sdf::PropertyQualities) -> wot::PropertyAffordance {
    // TODO: How should nullable be mapped?
    // TODO: How should contentFormat be mapped?

    let write_only;
    let read_only;
    let writable = sdf_property.writable.unwrap_or(true);
    let readable = sdf_property.readable.unwrap_or(true);
    if !readable && writable {
        write_only = Some(true);
        read_only = None;
    } else if !writable && readable {
        write_only = None;
        read_only = Some(true);
    } else {
        // TODO: How do you map a property that is neither writable nor readable?
        write_only = None;
        read_only = None;
    }

    // TODO: Refactor as sdfProperty is an alias for sdfData
    wot::PropertyAffordance {
        observable: sdf_property.observable.clone(),

        data_schema: wot::DataSchema {
            write_only,
            read_only,

            r#enum: None,    // Still TODO
            r#const: None,   // Still TODO
            data_type: None, // Still TODO
            one_of: None,    // TODO: Can this be mapped using sdfChoice?

            unit: sdf_property.unit.clone(), // TODO: Check if this kind of mapping is appropriate

            title: None,       // Set to None to avoid duplication
            description: None, // Set to None to avoid duplication
            titles: None,
            descriptions: None,
            format: None, // TODO: Can this be mapped?
            r#type: None,
        },

        interaction_affordance: create_interaction_affordance(&sdf_property.common_qualities),
    }
}

fn convert_properties(
    sdf_model: &sdf::SDFModel,
) -> Option<HashMap<String, wot::PropertyAffordance>> {
    let mut properties: HashMap<String, wot::PropertyAffordance> = HashMap::new();

    convert_sdf_properties(&sdf_model, &mut properties, &None, &None);
    convert_sdf_object_properties(&sdf_model, &mut properties, &sdf_model.sdf_object, &None);

    if properties.len() > 0 {
        Some(properties)
    } else {
        None
    }
}

macro_rules! conversion_function {
    ($wot_type:ty, $sdf_type:ty, $function_name:ident, $function_call:ident $(, $field_name:ident)?) => {
        fn $function_name(
            _sdf_model: &sdf::SDFModel, // Might be used later for resolving references
            wot_definitions: &mut HashMap<String, $wot_type>,
            sdf_definitions: &Option<HashMap<String, $sdf_type>>,
            prefix: &Option<String>,
        ) -> () {
            match sdf_definitions {
                Some(sdf_definitions) => {
                    for (key, value) in sdf_definitions {
                        let prefixed_key = get_prefixed_key(prefix, key.to_string());

                        macro_rules! inner_function {
                            ($inner_function_call:ident) => {
                                wot_definitions.insert(prefixed_key, $inner_function_call(&value));
                            };
                            ($inner_function_call:ident, $inner_field_name:ident) => {
                                $inner_function_call(
                                    _sdf_model,
                                    wot_definitions,
                                    &value.$inner_field_name,
                                    &Some(prefixed_key),
                                );
                            };
                        }

                        inner_function!($function_call $(, $field_name)?);
                    }
                }
                None => (),
            }
        }
    };
}

conversion_function!(wot::PropertyAffordance, sdf::PropertyQualities, convert_sdf_properties, convert_property);
conversion_function!(wot::ActionAffordance, sdf::ActionQualities, convert_sdf_actions, convert_action);
conversion_function!(wot::EventAffordance, sdf::EventQualities, convert_sdf_events, convert_event);
conversion_function!(wot::PropertyAffordance, sdf::ObjectQualities, convert_sdf_object_properties, convert_sdf_properties, sdf_property);
conversion_function!(wot::ActionAffordance, sdf::ObjectQualities, convert_sdf_object_actions, convert_sdf_actions, sdf_action);
conversion_function!(wot::EventAffordance, sdf::ObjectQualities, convert_sdf_object_events, convert_sdf_events, sdf_event);

fn convert_event(sdf_event: &sdf::EventQualities) -> wot::EventAffordance {
    wot::EventAffordance {
        subscription: None, // Still TODO
        data: None,         // Still TODO
        cancellation: None, // Still TODO

        interaction_affordance: create_interaction_affordance(&sdf_event.common_qualities),
    }
}

fn convert_events(sdf_model: &sdf::SDFModel) -> Option<HashMap<String, wot::EventAffordance>> {
    let mut events: HashMap<String, wot::EventAffordance> = HashMap::new();

    convert_sdf_events(&sdf_model, &mut events, &None, &None);
    convert_sdf_object_events(&sdf_model, &mut events, &sdf_model.sdf_object, &None);

    if events.len() > 0 {
        Some(events)
    } else {
        None
    }
}

pub fn convert(sdf_model: sdf::SDFModel) -> wot::Thing {
    let mut context_entries: Vec<wot::ContextEntry> = vec![wot::ContextEntry::String(
        "https://www.w3.org/2019/wot/td/v1".to_string(),
    )];
    let nosec_sc = wot::SecurityScheme::Nosec {
        common: wot::SecuritySchemeCommon {
            r#type: None,
            description: None,
            descriptions: None,
            proxy: None,
        },
    };
    let mut security_definitions: HashMap<String, wot::SecurityScheme> = HashMap::new();
    security_definitions.insert(String::from("nosec_sc"), nosec_sc);
    let links;

    let no_title = "No Title given.".to_string();
    let infoblock: &Option<sdf::InfoBlock> = &sdf_model.info;
    let title: String;
    let description: Option<String>;
    let version: Option<wot::VersionInfo>;
    match infoblock {
        None => {
            title = no_title;
            version = None;
            description = None;
            links = None;
        }
        Some(infoblock) => {
            title = infoblock.title.clone();
            version = Some(wot::VersionInfo {
                // TODO: Revisit use of "instance" and "model"
                instance: infoblock.version.clone(),
                model: None,
            });
            description = Some(infoblock.copyright.clone());
            links = Some(vec![wot::Link {
                rel: Some("license".to_string()),
                href: infoblock.license.clone(),
                r#type: None,
                anchor: None,
                sizes: None,
            }]);
        }
    };

    match sdf_model.namespace.clone() {
        Some(namespace) => {
            context_entries.push(wot::ContextEntry::Map(namespace));
        }
        None => {}
    };

    return wot::Thing {
        context: wot::Context::Array(context_entries),
        title,
        description,
        security: wot::TypeOrTypeArray::Type(String::from("nosec_sc")),
        security_definitions,
        version,
        actions: convert_actions(&sdf_model),
        properties: convert_properties(&sdf_model),
        events: convert_events(&sdf_model),
        links,

        // Not covered by SDF yet:
        r#type: None,
        titles: None,
        descriptions: None,
        id: None,
        forms: None,
        modified: None,
        profile: None,
        schema_definitions: None,
        base: None,
        created: None,
        support: None,
    };
}
