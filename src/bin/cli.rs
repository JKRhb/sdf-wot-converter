use sdf_wot_converter::converter;

use clap::{crate_authors, crate_version, App, Arg};
use std::env;
// use url::Url;

fn is_valid_input(input: String) -> Result<(), String> {
  if input.ends_with("sdf.json") || input.ends_with("td.json") {
    Ok(())
  } else {
    Err(String::from(
      "Illegal file ending! Must be either .sdf.json or td.json.",
    ))
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
      converter::print_sdf_definition(path);
      println!("{}", converter::get_sdf_json_string(path).unwrap());
    } else if path.ends_with("td.json") {
      converter::print_wot_definition(path);
    }
  } else if let Some(ref matches) = app.subcommand_matches("convert") {
    // TODO: Replace if-else with match
    let input_path = matches.value_of("input").unwrap();
    let output_path = matches.value_of("output").unwrap();
    if input_path.ends_with("sdf.json") {
      assert!(output_path.ends_with("td.json"));
      converter::sdf_to_wot_from_and_to_path(input_path, output_path);
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
