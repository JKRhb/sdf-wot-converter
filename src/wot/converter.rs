//! First attempt for a SDF to WoT converter.
//!
//! The code contained in this file should probably be moved elsewhere (either to
//! wot/definitions.rs or into to a separate directory.)

use super::super::sdf::definitions::InfoBlock;
use super::super::sdf::definitions::SDFModel;
use super::definitions::Context;
use super::definitions::SecurityScheme;
use super::definitions::SecuritySchemeCommon;
use super::definitions::Thing;
use super::definitions::TypeOrTypeArray;
use super::definitions::VersionInfo;
use std::collections::HashMap;
use std::convert::From;

impl From<SDFModel> for Thing {
    fn from(sdf_model: SDFModel) -> Self {
        let context = Context::String(String::from("https://www.w3.org/2019/wot/td/v1"));
        let nosec_sc = SecurityScheme::Nosec {
            common: SecuritySchemeCommon {
                r#type: None,
                description: None,
                descriptions: None,
                proxy: None,
            },
        };
        let mut security_definitions: HashMap<String, SecurityScheme> = HashMap::new();
        security_definitions.insert(String::from("nosec_sc"), nosec_sc);

        let no_title = "No Title given.".to_string();
        let infoblock: Option<InfoBlock> = sdf_model.info;
        let title: String;
        let description: Option<String>;
        let version: Option<VersionInfo>;
        match infoblock {
            None => {
                title = no_title;
                version = None;
                description = None;
            }
            Some(infoblock) => {
                title = infoblock.title;
                version = Some(VersionInfo {
                    // TODO: Revisit use of "instance" and "model"
                    instance: infoblock.version,
                    model: None,
                });
                description = Some(infoblock.copyright);
            }
        };

        return Thing {
            context,
            title,
            description,
            security: TypeOrTypeArray::Type(String::from("nosec_sc")),
            security_definitions: security_definitions,
            version,
            actions: None,
            properties: None,
            events: None,

            // Not covered by SDF yet:
            r#type: None,
            titles: None,
            descriptions: None,
            id: None,
            links: None,
            forms: None,
            modified: None,
            profile: None,
            schema_definitions: None,
            base: None,
            created: None,
            support: None,
        };
    }
}
