pub mod sdf_to_wot;
pub mod wot_to_sdf;

use crate::sdf::definitions::SDFModel;
use crate::wot::definitions::ThingDescription;
use crate::wot::definitions::ThingModel;
use crate::SerializableModel;

/// Deserializes a `SerializableModel`, converts it back into
/// a JSON string and prints it to the command line.
fn print_definition<T: SerializableModel>(path: &str) {
    match T::deserialize_json_from_path(path) {
        Ok(model) => model.print(),
        Err(error) => println!("{}", error),
    }
}

/// Deserializes an SDF model, converts it back into a JSON string
/// and prints it to the command line.
///
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::print_sdf_definition;
///
/// print_sdf_definition("examples/sdf/example.sdf.json");
/// ```
pub fn print_sdf_definition(path: &str) {
    print_definition::<SDFModel>(path)
}

/// Deserializes an WoT TD definition, converts it back into a
/// JSON string and prints it to the command line.
///
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::print_wot_td_definition;
///
/// print_wot_td_definition("examples/wot/example.td.json");
/// ```
pub fn print_wot_td_definition(path: &str) {
    print_definition::<ThingDescription>(path)
}

/// Deserializes an WoT TM definition, converts it back into a
/// JSON string and prints it to the command line.
///
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::print_wot_tm_definition;
///
/// print_wot_tm_definition("examples/wot/example.tm.json");
/// ```
pub fn print_wot_tm_definition(path: &str) {
    print_definition::<ThingModel>(path)
}

/// Converts an SDF model to a WoT Thing Model.
fn sdf_to_wot(sdf_model: SDFModel) -> Result<ThingModel, String> {
    Ok(sdf_to_wot::convert(sdf_model))
}

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

    #[test]
    fn test_print_definition() {
        print_definition::<SDFModel>("examples/sdf/example.sdf.json");
        print_definition::<ThingModel>("examples/wot/example.td.json");
    }
}
