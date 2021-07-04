use sdf_wot_converter::{converter, sdf::definitions::SDFModel, wot::definitions::Thing};

use clap::{crate_authors, crate_version, App, Arg};
use serde_json;
use std::env;
use std::fs;
// use url::Url;

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

fn is_valid_input(input: String) -> Result<(), String> {
  if input.ends_with("sdf.json") || input.ends_with("td.json") {
    Ok(())
  } else {
    Err(String::from("Illegal file ending! Must be either .sdf.json or td.json."))
  }
}

fn main() {
  let input_help = "The input file path. Must either end with sdf.json \
                    (for SDF) or td.json (for WoT TD).";
  let output_help = "The output file path. Must either end with sdf.json \
                     (when converting to SDF) or td.json (when converting \
                      to WoT TD).";

  let app = App::new("sdf-wot-converter")
    .version(crate_version!())
    .author(crate_authors!())
    .subcommand(
      App::new("print")
        .about("Reads in an SDF or WoT file and prints it in the terminal.")
        .arg(
          Arg::with_name("input")
            .help(input_help)
            .index(1)
            .required(true)
            .validator(is_valid_input),
        ),
    )
    .subcommand(
      App::new("convert")
        .about("Reads in an SDF or WoT file and prints it in the terminal.")
        .arg(
          Arg::with_name("input")
            .help(input_help)
            .index(1)
            .required(true)
            .validator(is_valid_input),
        )
        .arg(
          Arg::with_name("output")
            .help(output_help)
            .index(2)
            .required(true)
            .validator(is_valid_input),
        ),
    )
    .get_matches();

  if let Some(ref matches) = app.subcommand_matches("print") {
    let path = matches.value_of("input").unwrap();
    if path.ends_with("sdf.json") {
      print_definition::<SDFModel>(path);
    } else if path.ends_with("td.json") {
      print_definition::<Thing>(path);
    }
  } else if let Some(ref matches) = app.subcommand_matches("convert") {
    // TODO: Replace if-else with match
    let input_path = matches.value_of("input").unwrap();
    let output_path = matches.value_of("output").unwrap();
    if input_path.ends_with("sdf.json") {
      assert!(output_path.ends_with("td.json"));
      let json_string = convert_sdf_to_wot(input_path).and_then(|thing| serde_json::to_string_pretty(&thing));
      match json_string {
        Ok(json_string) => fs::write(output_path, json_string).expect("Unable to write file"),
        Err(error) => panic!("{}", error)
      }
    } else if input_path.ends_with("td.json") {
      panic!("TD to SDF conversion is not implemented yet!");
    }
  }

  // TODO: Implement possibility to use URLs as input
  //
  //   _ => {
  //     // FIXME: Parsing of URLs has to be implemented
  //     let data_url = Url::parse(path.as_str());
  //     match data_url {
  //       Ok(url) => {
  //         if url.scheme() == "http" || url.scheme() == "https" {
  //           println!("The use of URLs as an input is not implemented yet.");
  //         }
  //         Ok(())
  //       }
  //       Err(error) => Err(error),
  //     }
  //   }
  // }
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
