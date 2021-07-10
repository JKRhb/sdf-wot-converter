use crate::ConverterResult;

/// Conveniance trait that is used to identify structs that implement serialization and
/// deserialization capabilities provided by `serde`.
pub trait SerializableModel: serde::Serialize + serde::de::DeserializeOwned {
    fn new_empty_model() -> Self;

    fn print(&self) -> ConverterResult<()> {
        Self::serialize_json(self).map(|j| println!("{}", j))
    }

    fn print_definition(json_string: String) -> ConverterResult<()> {
        let model = Self::deserialize_json_string(json_string)?;
        model.print()
    }

    fn serialize_json(model: &Self) -> ConverterResult<String> {
        serde_json::to_string_pretty(&model).map_err(|e| e.into())
    }

    fn deserialize_json_string(json_string: String) -> ConverterResult<Self> {
        serde_json::from_str(json_string.as_str()).map_err(|e| e.into())
    }
}
