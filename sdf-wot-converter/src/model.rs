use crate::TResult;
use std::fs;

fn write_to_path(path: &str, content: String) -> TResult<()> {
    fs::write(path, content).map_err(|e| e.into())
}

/// Conveniance trait that is used to identify structs that implement serialization and
/// deserialization capabilities provided by `serde`.
pub trait SerializableModel: serde::Serialize + serde::de::DeserializeOwned {
    fn path_is_valid(path: &str) -> bool;

    fn print(&self) -> TResult<()> {
        self.serialize_json().map(|j| println!("{}", j))
    }

    /// Deserializes a `SerializableModel`, converts it back into
    /// a JSON string and prints it to the command line.
    fn print_definition_from_path(path: &str) -> TResult<()> {
        Self::deserialize_json_from_path(path).and_then(|m| m.print())
    }

    /// Reads a JSON file from a specified path, deserializes it to a supported data type,
    /// and returns an instance of the data type as a `Result`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the JSON file you want to deserialize
    ///
    fn import_json_from_path(path: &str) -> TResult<String> {
        if !Self::path_is_valid(&path) {
            return Err("Invalid input path given!".into());
        }
        fs::read_to_string(&path).map_err(|e| e.into())
    }

    fn serialize_json(&self) -> TResult<String> {
        serde_json::to_string_pretty(&self).map_err(|e| e.into())
    }

    fn deserialize_json_string(json_string: String) -> TResult<Self> {
        serde_json::from_str(json_string.as_str()).map_err(|e| e.into())
    }

    fn deserialize_json_from_path(path: &str) -> TResult<Self> {
        Self::import_json_from_path(path).and_then(Self::deserialize_json_string)
    }

    fn write_json_to_path(&self, path: &str) -> TResult<()> {
        self.serialize_json()
            .and_then(|json_string| write_to_path(path, json_string))
    }
}
