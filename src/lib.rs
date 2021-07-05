//! This crate provides a converter from
//! [SDF](https://datatracker.ietf.org/doc/html/draft-ietf-asdf-sdf-05)
//! (including protocol bindings) to [WoT TD](https://www.w3.org/TR/wot-thing-description/).
//!
//! The converter is both usable as a library that can be built upon in other WoT
//! and SDF related projects as well as a tool for the command line.

pub mod converter;
pub mod sdf;
pub mod wot;

use std::fs;

/// Conveniance trait that is used to identify structs that implement serialization and
/// deserialization capabilities provided by `serde`.
pub trait SerializableModel: serde::Serialize + serde::de::DeserializeOwned {
    fn path_is_valid(path: &str) -> bool;

    fn print(&self) {
        match self.serialize_json() {
            Ok(result) => println!("{}", result),
            Err(error) => println!("{}", error),
        }
    }

    /// Reads a JSON file from a specified path, deserializes it to a supported data type,
    /// and returns an instance of the data type as a `Result`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the JSON file you want to deserialize
    ///
    fn import_json_from_path(path: &str) -> Result<String, String> {
        if !Self::path_is_valid(&path) {
            return Err("Invalid input path given!".to_string());
        }
        fs::read_to_string(&path).or(Err("Something went wrong reading the file".to_string()))
    }

    fn serialize_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self).or(Err("Serialization failed!".to_string()))
    }

    fn deserialize_json(path: &str) -> Result<Self, String> {
        Self::import_json_from_path(path)
            .and_then(|x| serde_json::from_str(x.as_str()).map_err(|e| e.to_string()))
            .map_err(|e| e.to_string())
    }

    fn write_json_to_path(&self, path: &str) -> Result<(), String> {
        self.serialize_json()
            .and_then(|x| fs::write(path, x).map_err(|e| e.to_string()))
    }
}
