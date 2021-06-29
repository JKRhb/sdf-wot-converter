mod converter;
mod sdf;
mod wot;

use sdf::definitions::SDFModel;
use serde_json;
use std::env;
use std::fs;
use url::Url;
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
fn read_json<T: serde::Serialize + serde::de::DeserializeOwned>(
  path: &str,
) -> serde_json::Result<String> {
  let example = fs::read_to_string(&path).expect("Something went wrong reading the file");

  let definition: T = serde_json::from_str(&example)?;

  let j = serde_json::to_string_pretty(&definition)?;

  Ok(j)
}

fn print_definition<T: serde::Serialize + serde::de::DeserializeOwned>(path: &str) -> () {
  match read_json::<T>(path) {
    Ok(result) => println!("{}", result),
    Err(e) => println!("{}", e),
  };
}

fn convert_sdf_to_wot(path: &str) -> serde_json::Result<Thing> {
  // TODO: Refactor
  let example = fs::read_to_string(&path).expect("Something went wrong reading the file");
  let sdf_model: SDFModel = serde_json::from_str(&example)?;
  let thing = converter::sdf_to_wot::convert(sdf_model);

  Ok(thing)
}

fn main() -> Result<(), url::ParseError> {
  let args: Vec<String> = env::args().collect();
  let command = &args[1];
  let path = &args[2];

  // TODO: This is only a first rough attempt that has to be reworked
  match command.as_str() {
    "print" => {
      if path.ends_with("sdf.json") {
        print_definition::<SDFModel>(path.as_str());
      } else if path.ends_with("td.json") {
        print_definition::<Thing>(path.as_str());
      } else {
        panic!("Illegal file ending! Must be either .sdf.json or td.json.");
      }
      Ok(())
    }
    "convert" => {
      if path.ends_with("sdf.json") {
        match convert_sdf_to_wot(path) {
          Ok(thing) => {
            let json_string = serde_json::to_string_pretty(&thing);
            match json_string {
              Ok(json_string) => {
                if args.len() >= 4 {
                  fs::write(args[3].as_str(), json_string).expect("Unable to write file");
                } else {
                  println!("{}", json_string);
                }
              }
              Err(error) => println!("{}", error),
            }
          }
          Err(error) => println!("{}", error),
        };
      } else if path.ends_with("td.json") {
        panic!("TD to SDF conversion is not implemented yet!");
      } else {
        panic!("Illegal file ending! Must be either .sdf.json or td.json.");
      }
      Ok(())
    }
    _ => {
      // FIXME: Parsing of URLs has to be implemented
      let data_url = Url::parse(path.as_str());
      match data_url {
        Ok(url) => {
          if url.scheme() == "http" || url.scheme() == "https" {
            println!("The use of URLs as an input is not implemented yet.");
          }
          Ok(())
        }
        Err(error) => Err(error),
      }
    }
  }
}

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
