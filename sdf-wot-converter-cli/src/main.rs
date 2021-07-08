use sdf_wot_converter::converter;

use clap::{crate_authors, crate_version, App, Arg};
use std::env;
// use url::Url;

fn is_valid_input(input: String) -> Result<(), String> {
    let legal_file_endings = vec!["sdf.json", "td.json", "tm.json"];

    if legal_file_endings.iter().any(|x| input.ends_with(x)) {
        Ok(())
    } else {
        Err(String::from(
            "Illegal file ending! Must be either .sdf.json, td.json, or tm.json!",
        ))
    }
}

fn main() {
    let input_help = "The input file path. Must either end with sdf.json \
                    (for SDF), td.json or tm.json (when \
                    converting to a WoT TD/TM)";
    let output_help = "The output file path. Must either end with sdf.json \
                     (when converting to SDF), td.json or tm.json (when \
                     converting to a WoT TD/TM).";

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
                .about("Reads in an SDF or WoT file and converts it into the other format.")
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
        } else if path.ends_with("td.json") {
            converter::print_wot_td_definition(path);
        } else if path.ends_with("tm.json") {
            converter::print_wot_tm_definition(path);
        }
    } else if let Some(ref matches) = app.subcommand_matches("convert") {
        // TODO: Replace if-else with match
        let input_path = matches.value_of("input").unwrap();
        let output_path = matches.value_of("output").unwrap();
        if input_path.ends_with("sdf.json") {
            if let Err(error) = converter::sdf_to_wot_from_and_to_path(input_path, output_path) {
                println!("{}", error)
            }
        } else if input_path.ends_with("td.json") || input_path.ends_with("tm.json") {
            panic!("TD/TM to SDF conversion is not implemented yet!");
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
