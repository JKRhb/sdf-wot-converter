use sdf_wot_converter::{converter, ConverterResult};

use clap::{crate_authors, crate_name, crate_version, App, Arg, ArgGroup};
use std::{env, fs};
// use url::Url;

fn is_valid_path(path: String, ending: &str) -> Result<(), String> {
    if path.ends_with(ending) {
        Ok(())
    } else {
        Err(format!(
            "Illegal file ending of {}! Must end with {}",
            path, ending
        ))
    }
}

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

fn get_json_from_file(path: &str) -> ConverterResult<String> {
    fs::read_to_string(&path).map_err(|e| e.into())
}

fn print_model_from_file(path: &str) -> ConverterResult<()> {
    let json_string = get_json_from_file(path)?;
    if path.ends_with("sdf.json") {
        converter::print_sdf_definition(json_string)
    } else if path.ends_with("td.json") {
        converter::print_wot_td_definition(json_string)
    } else if path.ends_with("tm.json") {
        converter::print_wot_tm_definition(json_string)
    } else {
        Err("Illegal path ending!".into())
    }
}

fn convert_sdf_to_wot_tm(input_path: &str, output_path: &str) -> ConverterResult<()> {
    let input_string = get_json_from_file(input_path)?;
    let output_string = converter::convert_sdf_to_wot_tm(input_string)?;
    fs::write(output_path, output_string).map_err(|e| e.into())
}

fn main() -> ConverterResult<()> {
    let input_help = "The input file path. Must either end with sdf.json \
                    (for SDF), td.json or tm.json (when \
                    converting to a WoT TD/TM)";

    let sdf_input_name = "SDF input file";
    let tm_output_name = "TM output file";

    let app = App::new(crate_name!())
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
                .about("Reads in an SDF or WoT file and converts it into another format.")
                .arg(
                    Arg::with_name(sdf_input_name)
                        .long("from-sdf")
                        .help("Reads in an SDF file. Must end with sdf.json")
                        .validator(|p| is_valid_path(p, "sdf.json"))
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(tm_output_name)
                        .long("to-tm")
                        .help("Converts to a WoT Thing Model and writes it to a file.")
                        .takes_value(true),
                )
                .group(
                    ArgGroup::with_name("from")
                        .args(&[sdf_input_name])
                        .required(true),
                )
                .group(
                    ArgGroup::with_name("to")
                        .args(&[tm_output_name])
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(ref matches) = app.subcommand_matches("print") {
        if let Some(path) = matches.value_of("input") {
            print_model_from_file(path)?
        }
    } else if let Some(ref matches) = app.subcommand_matches("convert") {
        if let Some(input_path) = matches.value_of(sdf_input_name) {
            if let Some(output_path) = matches.value_of(tm_output_name) {
                return convert_sdf_to_wot_tm(input_path, output_path);
            }
        }
    }

    Ok(())

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
mod tests {
    use super::*;

    fn get_legal_inputs() -> Vec<&'static str> {
        vec![
            "examples/sdf/example.sdf.json",
            "examples/wot/example.td.json",
            "examples/wot/example.tm.json",
        ]
    }

    fn get_illegal_inputs() -> Vec<&'static str> {
        vec!["examples/foobar", "examples/foobar.json"]
    }

    #[test]
    fn print_model_from_legal_path_test() {
        assert!(get_legal_inputs()
            .iter()
            .all(|f| print_model_from_file(f).is_ok()));
    }

    #[test]
    fn print_model_from_illegal_path_test() {
        assert!(get_illegal_inputs()
            .iter()
            .all(|f| print_model_from_file(f).is_err()));
    }

    #[test]
    fn is_valid_input_test() {
        assert!(get_legal_inputs()
            .iter()
            .all(|f| is_valid_input(f.to_string()).is_ok()));
        assert!(get_illegal_inputs()
            .iter()
            .all(|f| is_valid_input(f.to_string()).is_err()));
    }
}
