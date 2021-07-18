use crate::sdf::definitions::SDFModel;
use crate::wot::definitions::ThingDescription;
use crate::wot::definitions::ThingModel;
use crate::ConverterResult;

fn print<T: serde::Serialize + serde::de::DeserializeOwned>(model: T) -> ConverterResult<()> {
    serialize_json::<T>(model).map(|j| println!("{}", j))
}

fn print_definition<T: serde::Serialize + serde::de::DeserializeOwned>(
    json_string: String,
) -> ConverterResult<()> {
    let model = deserialize_json_string::<T>(json_string)?;
    print(model)
}

fn serialize_json<T: serde::Serialize + serde::de::DeserializeOwned>(
    model: T,
) -> ConverterResult<String> {
    serde_json::to_string_pretty(&model).map_err(|e| e.into())
}

fn deserialize_json_string<T: serde::Serialize + serde::de::DeserializeOwned>(
    json_string: String,
) -> ConverterResult<T> {
    serde_json::from_str(json_string.as_str()).map_err(|e| e.into())
}

/// Deserializes an SDF model, converts it back into a JSON string
/// and prints it to the command line.
///
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::print_sdf_definition;
/// use std::fs;
///
/// let json_string = fs::read_to_string("examples/sdf/example.sdf.json").unwrap();
///
/// let result = print_sdf_definition(json_string);
///
/// assert!(result.is_ok());
/// ```
pub fn print_sdf_definition(json_string: String) -> ConverterResult<()> {
    print_definition::<SDFModel>(json_string)
}

/// Deserializes a WoT TD definition, converts it back into a
/// JSON string and prints it to the command line.
///
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::print_wot_td_definition;
/// use std::fs;
///
/// let json_string = fs::read_to_string("examples/wot/example.td.json").unwrap();
///
/// let result = print_wot_td_definition(json_string);
///
/// assert!(result.is_ok());
/// ```
pub fn print_wot_td_definition(json_string: String) -> ConverterResult<()> {
    print_definition::<ThingDescription>(json_string)
}

/// Deserializes a WoT TM definition, converts it back into a
/// JSON string and prints it to the command line.
///
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::print_wot_tm_definition;
/// use std::fs;
///
/// let json_string = fs::read_to_string("examples/wot/example.tm.json").unwrap();
///
/// let result = print_wot_tm_definition(json_string);
///
/// assert!(result.is_ok());
/// ```
pub fn print_wot_tm_definition(json_string: String) -> ConverterResult<()> {
    print_definition::<ThingModel>(json_string)
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
    deserialize_json_string::<SDFModel>(json_string)
        .and_then(sdf_to_wot_tm)
        .and_then(serialize_json::<ThingModel>)
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
    deserialize_json_string::<ThingModel>(json_string)
        .and_then(wot_tm_to_sdf)
        .and_then(serialize_json::<SDFModel>)
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
    fn test_sdf_to_wot_tm() {
        let output = sdf_to_wot_tm(SDFModel::default());
        assert!(output.is_ok());
    }

    #[test]
    fn test_wot_tm_to_sdf() {
        let output = wot_tm_to_sdf(ThingModel::default());
        assert!(output.is_ok());
    }
}
