use crate::model::SerializableModel;
use crate::sdf::definitions::SDFModel;
use crate::wot::definitions::ThingDescription;
use crate::wot::definitions::ThingModel;
use crate::ConverterResult;

/// Deserializes an SDF model, converts it back into a JSON string
/// and prints it to the command line.
///
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::print_sdf_definition_from_path;
///
/// print_sdf_definition_from_path("examples/sdf/example.sdf.json");
///
/// assert!(print_sdf_definition_from_path("examples/sdf/example.sdf.json").is_ok());
/// assert!(print_sdf_definition_from_path("foobar.json").is_err());
/// ```
pub fn print_sdf_definition(json_string: String) -> ConverterResult<()> {
    ThingDescription::print_definition(json_string)
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
///
/// assert!(print_wot_td_definition_from_path("examples/wot/example.td.json").is_ok());
/// assert!(print_wot_td_definition_from_path("foobar.json").is_err());
/// ```
pub fn print_wot_td_definition(json_string: String) -> ConverterResult<()> {
    ThingDescription::print_definition(json_string)
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
///
/// assert!(print_wot_tm_definition_from_path("examples/wot/example.tm.json").is_ok());
/// assert!(print_wot_tm_definition_from_path("foobar.json").is_err());
/// ```
pub fn print_wot_tm_definition(json_string: String) -> ConverterResult<()> {
    ThingModel::print_definition(json_string)
}

/// Deserializes an SDF Model JSON `String` and converts it into an WoT Thing Model
/// JSON `String`.
///
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::convert_sdf_to_wot_tm;
/// use std::fs;
///
/// let json_string = fs::read_to_string("examples/sdf/example.sdf.json").unwrap();
///
/// let result = convert_sdf_to_wot_tm(json_string);
/// assert!(result.is_ok());
/// ```
pub fn convert_sdf_to_wot_tm(json_string: String) -> ConverterResult<String> {
    SDFModel::deserialize_json_string(json_string)
        .and_then(sdf_to_wot_tm)
        .and_then(|m| ThingModel::serialize_json(&m))
}

/// Deserializes a WoT Thing Model JSON `String` and converts it into an SDF Model
/// JSON `String`.
///
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::convert_sdf_to_wot_tm;
/// use std::fs;
///
/// let json_string = fs::read_to_string("examples/sdf/example.sdf.json").unwrap();
///
/// let result = convert_sdf_to_wot_tm(json_string);
/// assert!(result.is_ok());
/// ```
pub fn convert_wot_tm_to_sdf(json_string: String) -> ConverterResult<String> {
    ThingModel::deserialize_json_string(json_string)
        .and_then(wot_tm_to_sdf)
        .and_then(|m| SDFModel::serialize_json(&m))
}

/// Converts an SDF model to a WoT Thing Model.
fn sdf_to_wot_tm(sdf_model: SDFModel) -> ConverterResult<ThingModel> {
    Ok(ThingModel::from(sdf_model))
}

/// Converts a WoT Thing Model to an SDF model.
fn wot_tm_to_sdf(thing_model: ThingModel) -> ConverterResult<SDFModel> {
    Ok(SDFModel::from(thing_model))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdf_to_wot() {
        let output = sdf_to_wot_tm(SDFModel::new_empty_model());
        assert!(output.is_ok());
    }

    #[test]
    fn test_wot_tm_to_sdf() {
        let output = wot_tm_to_sdf(ThingModel::new_empty_model());
        assert!(output.is_ok());
    }
}
