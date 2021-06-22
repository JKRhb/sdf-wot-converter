//! First attempt for a SDF to WoT converter.
//!
//! The code contained in this file should probably moved elsewhere (either to 
//! wot/definitions.rs or into to a separate directory.)

use super::super::sdf::definitions::InfoBlock;
use super::super::sdf::definitions::SDFModel;
use super::definitions::Context;
use super::definitions::SecurityScheme;
use super::definitions::SecuritySchemeCommon;
use super::definitions::Thing;
use super::definitions::TypeOrTypeArray;
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
        let title: String = match infoblock {
            None => no_title,
            Some(infoblock) => match Some(infoblock.title) {
                None => no_title,
                Some(title) => title,
            },
        };

        return Thing {
            context: context,
            title: title,
            titles: None,
            base: None,
            created: None,
            version: None,
            support: None,
            description: None,
            descriptions: None,
            actions: None,
            properties: None,
            events: None,
            forms: None,
            id: None,
            links: None,
            modified: None,
            profile: None,
            schema_definitions: None,
            security: TypeOrTypeArray::Type(String::from("nosec_sc")),
            security_definitions: security_definitions,
            r#type: None,
        };
    }
}
