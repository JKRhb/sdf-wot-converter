//! First attempt for a SDF to WoT converter.

use crate::sdf::definitions as sdf;
use crate::wot::definitions as wot;
use std::collections::HashMap;

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
    let infoblock: Option<sdf::InfoBlock> = sdf_model.info;
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
            title = infoblock.title;
            version = Some(wot::VersionInfo {
                // TODO: Revisit use of "instance" and "model"
                instance: infoblock.version,
                model: None,
            });
            description = Some(infoblock.copyright);
            links = Some(vec![wot::Link {
                rel: Some("license".to_string()),
                href: infoblock.license,
                r#type: None,
                anchor: None,
                sizes: None,
            }]);
        }
    };

    match sdf_model.namespace {
        Some(namespace) => {
            context_entries.push(wot::ContextEntry::Map(namespace.clone()));
        }
        None => {}
    };

    return wot::Thing {
        context: wot::Context::Array(context_entries),
        title,
        description,
        security: wot::TypeOrTypeArray::Type(String::from("nosec_sc")),
        security_definitions: security_definitions,
        version,
        actions: None,
        properties: None,
        events: None,
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