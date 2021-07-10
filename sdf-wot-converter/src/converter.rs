use crate::model::SerializableModel;
use crate::sdf::definitions::SDFModel;
use crate::wot::definitions::ThingDescription;
use crate::wot::definitions::ThingModel;

/// Deserializes an SDF model, converts it back into a JSON string
/// and prints it to the command line.
///
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::print_sdf_definition_from_path;
///
/// print_sdf_definition_from_path("examples/sdf/example.sdf.json");
/// ```
pub fn print_sdf_definition_from_path(path: &str) {
    SDFModel::print_definition_from_path(path);
}

/// Deserializes a WoT TD definition, converts it back into a
/// JSON string and prints it to the command line.
///
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::print_wot_td_definition_from_path;
///
/// print_wot_td_definition_from_path("examples/wot/example.td.json");
/// ```
pub fn print_wot_td_definition_from_path(path: &str) {
    ThingDescription::print_definition_from_path(path);
}

/// Deserializes a WoT TM definition, converts it back into a
/// JSON string and prints it to the command line.
///
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::print_wot_tm_definition_from_path;
///
/// print_wot_tm_definition_from_path("examples/wot/example.tm.json");
/// ```
pub fn print_wot_tm_definition_from_path(path: &str) {
    ThingModel::print_definition_from_path(path);
}

/// Converts an SDF model to a WoT Thing Model.
fn sdf_to_wot(sdf_model: SDFModel) -> Result<ThingModel, String> {
    Ok(ThingModel::from(sdf_model))
}

/// Deserializes an WoT TM definition, converts it back into a
/// JSON string and prints it to the command line.
///
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::print_wot_tm_definition_from_path;
///
/// print_wot_tm_definition_from_path("examples/wot/example.tm.json");
/// ```
pub fn sdf_to_wot_from_path(path: &str) -> Result<ThingModel, String> {
    SDFModel::deserialize_json_from_path(path).and_then(sdf_to_wot)
}

pub fn sdf_to_wot_from_and_to_path(input_path: &str, output_path: &str) -> Result<(), String> {
    if !output_path.ends_with("tm.json") {
        return Err("The output filename has to end with tm.json!".to_string());
    }

    sdf_to_wot_from_path(input_path).and_then(|x| x.write_json_to_path(output_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdf_to_wot() {
        let sdf_model = SDFModel {
            info: None,
            namespace: None,
            default_namespace: None,
            sdf_data: None,
            sdf_object: None,
            sdf_thing: None,
            sdf_product: None,
            sdf_action: None,
            sdf_property: None,
            sdf_event: None,
        };

        let output = sdf_to_wot(sdf_model);
        assert!(output.is_ok());
    }
}
