mod sdf;
mod wot;
use sdf::definitions::SDFModel;
use wot::definitions::Thing;
use std::fs;
use serde_json::Result;

fn test_function<T: serde::Serialize + serde::de::DeserializeOwned>(path: &str) -> Result<String> {
  let example = fs::read_to_string(&path)
  .expect("Something went wrong reading the file");

  let sdf_model: T = serde_json::from_str(&example)?;

  let j = serde_json::to_string_pretty(&sdf_model)?;

  return Ok(j);
}

fn sdf_test_function() -> Result<String> {
  return test_function::<SDFModel>("examples/sdf/example.sdf.json");
}

fn wot_test_function() -> Result<String> {
  return test_function::<Thing>("examples/wot/example.td.json");
}


fn main() {
    match sdf_test_function() {
        Ok(result)  => println!("{}", result),
        Err(e) => println!("{}", e),
    };

    match wot_test_function() {
        Ok(result)  => println!("{}", result),
        Err(e) => println!("{}", e),
    };
}
