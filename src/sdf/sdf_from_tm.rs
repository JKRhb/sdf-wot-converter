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

impl From<wot::DataSchema> for sdf::DataQualities {
    fn from(data_schema: wot::DataSchema) -> Self {
        let unit = data_schema.unit.clone();
        let writable = data_schema.read_only.map(|x| !x);
        let readable = data_schema.write_only.map(|x| !x);

        // TODO: Unmapped fields:
        // pub r#type: Option<TypeOrTypeArray<String>>,
        // pub titles: Option<HashMap<String, String>>, // TODO: Consider using a MultiLanguage struct instead
        // pub descriptions: Option<HashMap<String, String>>,
        // #[serde(flatten)]
        // pub data_type: Option<JSONSchemaTypes>,
        // pub r#const: Option<serde_json::Value>,
        // pub one_of: Option<Vec<DataSchema>>,
        // pub r#enum: Option<Vec<serde_json::Value>>,
        // pub format: Option<String>,

        sdf::DataQualities {
            common_qualities: sdf::CommonQualities::from(data_schema),
            jsonschema: None,
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
