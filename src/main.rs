mod sdf;
mod wot;
use sdf::definitions::SDFModel;
use serde_json::Result;
use std::fs;
use wot::definitions::Thing;

fn read_json<T: serde::Serialize + serde::de::DeserializeOwned>(path: &str) -> Result<String> {
  let example = fs::read_to_string(&path).expect("Something went wrong reading the file");

  let definition: T = serde_json::from_str(&example)?;

  let j = serde_json::to_string_pretty(&definition)?;

  return Ok(j);
}

fn test_function<T: serde::Serialize + serde::de::DeserializeOwned>(path: &str) -> () {
  match read_json::<T>(path) {
    Ok(result) => println!("{}", result),
    Err(e) => println!("{}", e),
  };
}

fn main() {
  test_function::<SDFModel>("examples/sdf/example.sdf.json");
  test_function::<Thing>("examples/wot/example.td.json");
}
