mod sdf;
mod wot;
use sdf::definitions::SDFModel;
use serde_json::Result;
use std::fs;
use wot::definitions::Thing;

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
fn read_json<T: serde::Serialize + serde::de::DeserializeOwned>(path: &str) -> Result<String> {
  let example = fs::read_to_string(&path).expect("Something went wrong reading the file");

  let definition: T = serde_json::from_str(&example)?;

  let j = serde_json::to_string_pretty(&definition)?;

  return Ok(j);
}

fn print_definition<T: serde::Serialize + serde::de::DeserializeOwned>(path: &str) -> () {
  match read_json::<T>(path) {
    Ok(result) => println!("{}", result),
    Err(e) => println!("{}", e),
  };
}

fn convert_sdf_to_wot(path: &str) -> Result<Thing> {
  // TODO: Refactor
  let example = fs::read_to_string(&path).expect("Something went wrong reading the file");
  let sdf_model: SDFModel = serde_json::from_str(&example)?;
  let thing = Thing::from(sdf_model);

  Ok(thing)
}

fn main() {
  print_definition::<Thing>("examples/wot/example.td.json");
  print_definition::<SDFModel>("examples/sdf/example.sdf.json");

  match convert_sdf_to_wot("examples/sdf/example.sdf.json") {
    Ok(thing) => {
      let j = serde_json::to_string_pretty(&thing);
      match j {
        Ok(json_string) => println!("{}", json_string),
        Err(error) => println!("{}", error)
      }
      
    }
    Err(error) => println!("{}", error),
  };

#[cfg(test)]
mod test {
  use super::*;

  fn test_function<T: serde::Serialize + serde::de::DeserializeOwned>(path: &str) -> () {
    match read_json::<T>(path) {
      Ok(result) => println!("{}", result),
      Err(error) => panic!("{}", error),
    };
  }

  #[test]
  fn test_sdf() {
    // TODO: Add assertions
    test_function::<SDFModel>("examples/sdf/example.sdf.json");
  }

  #[test]
  fn test_wot() {
    // TODO: Add assertions
    test_function::<Thing>("examples/wot/example.td.json");
  }
}
