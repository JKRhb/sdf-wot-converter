pub mod sdf_to_wot;
pub mod wot_to_sdf;

use super::sdf::definitions::SDFModel;
use super::wot::definitions::Thing;
use std::fs;

/// Reads a JSON file from a specified path, deserializes it to a supported data type,
/// and returns a formatted `String` as a `Result`.
///
/// # Arguments
///
/// * `path` - The path of the JSON file you want to deserialize
///
/// # Examples
///
/// ```
/// read_json::<SDFModel>("examples/sdf/example.sdf.json");
/// read_json::<Thing>("examples/wot/example.td.json");
/// ```
pub fn read_json<T: serde::Serialize + serde::de::DeserializeOwned>(
    path: &str,
) -> serde_json::Result<String> {
    let example = fs::read_to_string(&path).expect("Something went wrong reading the file");
    let definition: T = serde_json::from_str(&example)?;
    let j = serde_json::to_string_pretty(&definition)?;
    Ok(j)
}

pub fn convert_sdf_to_wot(path: &str) -> serde_json::Result<Thing> {
    // TODO: Refactor
    let example = fs::read_to_string(&path).expect("Something went wrong reading the file");
    let sdf_model: SDFModel = serde_json::from_str(&example)?;
    let thing = sdf_to_wot::convert(sdf_model);
    Ok(thing)
}
