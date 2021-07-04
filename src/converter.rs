pub mod sdf_to_wot;
pub mod wot_to_sdf;

use super::sdf::definitions::SDFModel;
use super::wot::definitions::Thing;
use std::fs;

/// Conveniance trait that is used to identify structs that implement serialization and 
/// deserialization capabilities provided by `serde`.
pub trait SerializableModel: serde::Serialize + serde::de::DeserializeOwned {}
impl<T: serde::Serialize + serde::de::DeserializeOwned> SerializableModel for T {}

/// Reads a JSON file from a specified path, deserializes it to a supported data type,
/// and returns a formatted `String` as a `Result`.
///
/// # Arguments
///
/// * `path` - The path of the JSON file you want to deserialize
///
fn import_from_json<T: SerializableModel>(path: &str) -> Result<T, serde_json::Error> {
    let json_string = fs::read_to_string(&path).expect("Something went wrong reading the file");
    let definition: T = serde_json::from_str(&json_string)?;
    Ok(definition)
}

fn serialize_json<T: SerializableModel>(model: T) -> Result<String, String> {
    serde_json::to_string_pretty(&model).or(Err("Serialization failed!".to_string()))
}

pub fn serialize_sdf_json(sdf_model: SDFModel) -> Result<String, String> {
    serialize_json::<SDFModel>(sdf_model)
}

pub fn serialize_wot_json(wot_thing: Thing) -> Result<String, String> {
    serialize_json::<Thing>(wot_thing)
}

fn deserialize_json<T: SerializableModel>(path: &str) -> Result<T, String> {
    match import_from_json::<T>(path) {
        Ok(sdf_model) => Ok(sdf_model),
        Err(_) => Err("Deserialization failed!".to_string()),
    }
}

pub fn deserialize_sdf_json(path: &str) -> Result<SDFModel, String> {
    deserialize_json::<SDFModel>(path)
}

pub fn deserialize_wot_json(path: &str) -> Result<Thing, String> {
    deserialize_json::<Thing>(path)
}

fn get_json_string<T: SerializableModel>(path: &str) -> Result<String, String> {
    deserialize_json::<T>(path).and_then(serialize_json)
}

pub fn get_sdf_json_string(path: &str) -> Result<String, String> {
    get_json_string::<SDFModel>(path)
}

pub fn get_wot_json_string(path: &str) -> Result<String, String> {
    get_json_string::<Thing>(path)
}

fn print_definition<T: SerializableModel>(path: &str) -> () {
    match get_json_string::<T>(path) {
        Ok(result) => println!("{}", result),
        Err(e) => println!("{}", e),
    };
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
pub fn print_sdf_definition(path: &str) -> () {
    assert!(path.ends_with("sdf.json"));
    print_definition::<SDFModel>(path)
}

/// Deserializes an WoT TD definition, converts it back into a 
/// JSON string and prints it to the command line.
/// 
/// # Examples
///
/// ```rust
/// use sdf_wot_converter::converter::print_wot_definition;
/// 
/// print_wot_definition("examples/wot/example.td.json");
/// ```
pub fn print_wot_definition(path: &str) -> () {
    assert!(path.ends_with("td.json"));
    print_definition::<Thing>(path)
}

pub fn sdf_to_wot(sdf_model: SDFModel) -> Result<Thing, String> {
    Ok(sdf_to_wot::convert(sdf_model))
}

pub fn sdf_to_wot_from_path(path: &str) -> Result<Thing, String> {
    deserialize_sdf_json(path).and_then(sdf_to_wot)
}

pub fn sdf_to_wot_from_and_to_path(input_path: &str, output_path: &str) {
    let model = sdf_to_wot_from_path(input_path).and_then(serialize_wot_json);

    match model {
        Ok(json_string) => fs::write(output_path, json_string).expect("Writing to file failed!"),
        Err(error) => panic!("{}", error),
    }
}

pub fn convert_sdf_to_wot_file(path: &str) -> Result<Thing, String> {
    deserialize_sdf_json(path).and_then(|thing| Ok(sdf_to_wot::convert(thing)))
}
