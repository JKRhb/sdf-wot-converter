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
        let sdf_property = None;
        let sdf_action = None;
        let sdf_event = None;
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
