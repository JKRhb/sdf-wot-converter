//! First attempt for a SDF to WoT converter.

use crate::sdf::definitions as sdf;
use crate::wot::definitions as wot;
use std::collections::HashMap;

fn convert_actions(sdf_model: &sdf::SDFModel) -> Option<HashMap<String, wot::ActionAffordance>> {
    let mut actions: HashMap<String, wot::ActionAffordance> = HashMap::new();

    match &sdf_model.sdf_action {
        Some(sdf_actions) => {
            for (key, value) in sdf_actions {
                let title: Option<String> = value.common_qualities.label.clone();
                let description: Option<String> = value.common_qualities.description.clone();

                let wot_action = wot::ActionAffordance {
                    // TODO: Deal with input and output data
                    input: None,
                    output: None,
                    safe: None,
                    idempotent: None,

                    interaction_affordance: wot::InteractionAffordance {
                        title,
                        description,

                        forms: Vec::<wot::Form>::new(),
                        titles: None,
                        descriptions: None,
                        r#type: None,
                        uri_variables: None,
                    },
                };

                actions.insert(key.clone(), wot_action);
            }
        }
        None => (),
    }

    if actions.len() > 0 {
        Some(actions)
    } else {
        None
    }
}

fn convert_properties(
    sdf_model: &sdf::SDFModel,
) -> Option<HashMap<String, wot::PropertyAffordance>> {
    let mut properties: HashMap<String, wot::PropertyAffordance> = HashMap::new();

    match &sdf_model.sdf_property {
        Some(sdf_properties) => {
            for (key, value) in sdf_properties {
                // TODO: Refactor mapping of common qualities
                // TODO: How should nullable be mapped?
                // TODO: How should contentFormat be mapped?
                let title: Option<String> = value.common_qualities.label.clone();
                let description: Option<String> = value.common_qualities.description.clone();

                let write_only;
                let read_only;
                let writable = value.writable.unwrap_or(true);
                let readable = value.readable.unwrap_or(true);
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
                let wot_property = wot::PropertyAffordance {
                    observable: value.observable.clone(),

                    data_schema: wot::DataSchema {
                        write_only,
                        read_only,

                        r#enum: None,    // Still TODO
                        r#const: None,   // Still TODO
                        data_type: None, // Still TODO
                        one_of: None,    // TODO: Can this be mapped using sdfChoice?

                        unit: value.unit.clone(), // TODO: Check if this kind of mapping is appropriate

                        title: None,       // Set to None to avoid duplication
                        description: None, // Set to None to avoid duplication
                        titles: None,
                        descriptions: None,
                        format: None, // TODO: Can this be mapped?
                        r#type: None,
                    },

                    interaction_affordance: wot::InteractionAffordance {
                        title,
                        description,

                        forms: Vec::<wot::Form>::new(),
                        titles: None,
                        descriptions: None,
                        r#type: None,
                        uri_variables: None,
                    },
                };

                properties.insert(key.clone(), wot_property);
            }
        }
        None => (),
    }

    if properties.len() > 0 {
        Some(properties)
    } else {
        None
    }
}

fn convert_events(sdf_model: &sdf::SDFModel) -> Option<HashMap<String, wot::EventAffordance>> {
    let mut events: HashMap<String, wot::EventAffordance> = HashMap::new();

    match &sdf_model.sdf_event {
        Some(sdf_events) => {
            for (key, value) in sdf_events {
                let title: Option<String> = value.common_qualities.label.clone();
                let description: Option<String> = value.common_qualities.description.clone();

                let wot_event = wot::EventAffordance {
                    subscription: None, // Still TODO
                    data: None,         // Still TODO
                    cancellation: None, // Still TODO

                    // TODO: Refactor generation of InteractionAffordance
                    interaction_affordance: wot::InteractionAffordance {
                        title,
                        description,

                        forms: Vec::<wot::Form>::new(),
                        titles: None,
                        descriptions: None,
                        r#type: None,
                        uri_variables: None,
                    },
                };

                events.insert(key.clone(), wot_event);
            }
        }
        None => (),
    }

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
