use std::fs;

fn write_to_path(path: &str, content: String) -> Result<(), String> {
    match fs::write(path, content) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error writing to path".to_string()),
    }
}

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

    /// Deserializes a `SerializableModel`, converts it back into
    /// a JSON string and prints it to the command line.
    fn print_definition_from_path(path: &str) {
        match Self::deserialize_json_from_path(path) {
            Ok(model) => model.print(),
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
        fs::read_to_string(&path).map_err(|_| "Something went wrong reading the file".to_string())
    }

    fn serialize_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self).map_err(|_| "Serialization failed!".to_string())
    }

    fn deserialize_json_string(json_string: String) -> Result<Self, String> {
        match serde_json::from_str(json_string.as_str()) {
            Ok(model) => Ok(model),
            Err(_) => Err("Deserialization failed!".to_string()),
        }
    }

    fn deserialize_json_from_path(path: &str) -> Result<Self, String> {
        Self::import_json_from_path(path).and_then(Self::deserialize_json_string)
    }

    fn write_json_to_path(&self, path: &str) -> Result<(), String> {
        self.serialize_json()
            .and_then(|json_string| write_to_path(path, json_string))
    }
}
