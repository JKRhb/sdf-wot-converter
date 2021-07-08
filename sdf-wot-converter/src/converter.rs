pub mod sdf_to_wot;
pub mod wot_to_sdf;

use crate::deserialize_json_from_path;
use crate::sdf::definitions::SDFModel;
use crate::wot::definitions::ThingDescription;
use crate::wot::definitions::ThingModel;
use crate::SerializableModel;

fn print_definition<T: SerializableModel>(path: &str) {
    match deserialize_json_from_path::<T>(path) {
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

pub fn sdf_to_wot(sdf_model: SDFModel) -> Result<ThingModel, String> {
    Ok(sdf_to_wot::convert(sdf_model))
}

pub fn sdf_to_wot_from_path(path: &str) -> Result<ThingModel, String> {
    deserialize_json_from_path::<SDFModel>(path).and_then(sdf_to_wot)
}

pub fn sdf_to_wot_from_and_to_path(input_path: &str, output_path: &str) -> Result<(), String> {
    if !output_path.ends_with("tm.json") {
        return Err("The output filename has to end with tm.json!".to_string());
    }

    sdf_to_wot_from_path(input_path).and_then(|x| x.write_json_to_path(output_path))
}
