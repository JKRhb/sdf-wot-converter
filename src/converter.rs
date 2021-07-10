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
pub fn print_sdf_definition_from_path(path: &str) -> ConverterResult<()> {
    SDFModel::print_definition_from_path(path)
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
pub fn print_wot_td_definition_from_path(path: &str) -> ConverterResult<()> {
    ThingDescription::print_definition_from_path(path)
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
pub fn print_wot_tm_definition_from_path(path: &str) -> ConverterResult<()> {
    ThingModel::print_definition_from_path(path)
}

/// Converts an SDF model to a WoT Thing Model.
fn sdf_to_wot(sdf_model: SDFModel) -> ConverterResult<ThingModel> {
    Ok(ThingModel::from(sdf_model))
}

/// Converts a WoT Thing Model to an SDF model.
fn wot_tm_to_sdf(thing_model: ThingModel) -> ConverterResult<SDFModel> {
    Ok(SDFModel::from(thing_model))
}

/// Deserializes an SDF definition from a given file path and converts
/// it into a WoT Thing Model.
///
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::sdf_to_wot_from_path;
///
/// let result = sdf_to_wot_from_path("examples/sdf/example.sdf.json");
/// assert!(result.is_ok());
///
/// assert!(sdf_to_wot_from_path("foobar.json").is_err());
/// ```
pub fn sdf_to_wot_from_path(path: &str) -> ConverterResult<ThingModel> {
    SDFModel::deserialize_json_from_path(path).and_then(sdf_to_wot)
}

/// Deserializes a WoT Thing Model from a given file path and converts
/// it into an SDF Model.
///
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::wot_tm_to_sdf_from_path;
///
/// let result = wot_tm_to_sdf_from_path("examples/wot/example.tm.json");
/// assert!(result.is_ok());
///
/// assert!(wot_tm_to_sdf_from_path("foobar.json").is_err());
/// ```
pub fn wot_tm_to_sdf_from_path(path: &str) -> ConverterResult<SDFModel> {
    ThingModel::deserialize_json_from_path(path).and_then(wot_tm_to_sdf)
}

pub fn sdf_to_wot_from_and_to_path(input_path: &str, output_path: &str) -> ConverterResult<()> {
    if !output_path.ends_with("tm.json") {
        return Err("The output filename has to end with tm.json!".into());
    }

    sdf_to_wot_from_path(input_path).and_then(|x| x.write_json_to_path(output_path))
}

pub fn wot_tm_to_sdf_from_and_to_path(input_path: &str, output_path: &str) -> ConverterResult<()> {
    if !output_path.ends_with("sdf.json") {
        return Err("The output filename has to end with sdf.json!".into());
    }

    wot_tm_to_sdf_from_path(input_path).and_then(|x| x.write_json_to_path(output_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdf_to_wot() {
        let output = sdf_to_wot(SDFModel::new_empty_model());
        assert!(output.is_ok());
    }

    #[test]
    fn test_wot_tm_to_sdf() {
        let output = wot_tm_to_sdf(ThingModel::new_empty_model());
        assert!(output.is_ok());
    }
}
