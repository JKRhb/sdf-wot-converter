use sdf_wot_converter::{converter, ConverterResult};

use clap::{crate_authors, crate_name, crate_version, App, Arg, ArgGroup};
use std::{env, fs};
use url::Url;

#[derive(Debug, PartialEq)]
enum InputPathType {
    File,
    ValidUrl,
    InvalidUrl,
}

fn determine_path_type(path: &str) -> InputPathType {
    match Url::from_file_path(path) {
        Ok(_) => InputPathType::File,
        Err(_) => match Url::parse(path) {
            Ok(url) => {
                if url.scheme() == "http" || url.scheme() == "https" {
                    InputPathType::ValidUrl
                } else if url.scheme() == "file" {
                    InputPathType::File
                } else {
                    InputPathType::InvalidUrl
                }
            }
            Err(_) => InputPathType::File,
        },
    }
}

fn write_to_file(path: &str, content: String) -> ConverterResult<()> {
    fs::write(path, content).map_err(|e| e.into())
}

fn write_to_another_file(input_path: &str, output_path: &str) -> ConverterResult<()> {
    let content = get_json(input_path)?;
    write_to_file(output_path, content)
}

fn get_json_from_file(path: &str) -> ConverterResult<String> {
    fs::read_to_string(&path).map_err(|e| e.into())
}

fn get_json_from_url(url: &str) -> ConverterResult<String> {
    Ok(reqwest::blocking::get(url)?.text()?)
}

fn get_json(path: &str) -> ConverterResult<String> {
    let path_type = determine_path_type(path);
    match path_type {
        InputPathType::File => get_json_from_file(path),
        InputPathType::ValidUrl => get_json_from_url(path),
        InputPathType::InvalidUrl => Err("Invalid URL or file path!".into()),
    }
}

fn print_model_from_file(
    path: &str,
    print_function: &dyn Fn(String) -> ConverterResult<()>,
) -> ConverterResult<()> {
    let json_string = get_json(path)?;
    print_function(json_string)
}

fn convert(
    input_path: &str,
    output_path: &str,
    conversion_function: &dyn Fn(String) -> ConverterResult<String>,
) -> ConverterResult<()> {
    let input_string = get_json(input_path)?;
    let output_string = conversion_function(input_string)?;
    write_to_file(output_path, output_string)
}

fn main() -> ConverterResult<()> {
    let sdf_input_name = "SDF input file";
    let sdf_output_name = "SDF output file";
    let tm_input_name = "TM input file";
    let tm_output_name = "TM output file";
    let td_input_name = "TD input file";

    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            App::new("print")
                .about("Reads in an SDF or WoT file and prints it in the terminal.")
                .arg(
                    Arg::with_name(sdf_input_name)
                        .long("sdf")
                        .help("Reads in an SDF file.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(tm_input_name)
                        .long("tm")
                        .help("Reads in a WoT Thing Model file.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(td_input_name)
                        .long("td")
                        .help("Reads in a WoT Thing Description file.")
                        .takes_value(true),
                )
                .group(
                    ArgGroup::with_name("input")
                        .args(&[sdf_input_name, tm_input_name, td_input_name])
                        .required(true),
                ),
        )
        .subcommand(
            App::new("convert")
                .about("Reads in an SDF or WoT file and converts it into another format.")
                .arg(
                    Arg::with_name(sdf_input_name)
                        .long("from-sdf")
                        .help("Reads in an SDF file.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(tm_input_name)
                        .long("from-tm")
                        .help("Reads in a WoT Thing Model file.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(tm_output_name)
                        .long("to-tm")
                        .help("Converts to a WoT Thing Model and writes it to a file.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(sdf_output_name)
                        .long("to-sdf")
                        .help("Converts to a WoT Thing Model and writes it to a file.")
                        .takes_value(true),
                )
                .group(
                    ArgGroup::with_name("from")
                        .args(&[sdf_input_name, tm_input_name])
                        .required(true),
                )
                .group(
                    ArgGroup::with_name("to")
                        .args(&[tm_output_name, sdf_output_name])
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(ref matches) = app.subcommand_matches("print") {
        if let Some(input_path) = matches.value_of(sdf_input_name) {
            return print_model_from_file(input_path, &converter::print_sdf_definition);
        } else if let Some(input_path) = matches.value_of(td_input_name) {
            return print_model_from_file(input_path, &converter::print_wot_td_definition);
        } else if let Some(input_path) = matches.value_of(tm_input_name) {
            return print_model_from_file(input_path, &converter::print_wot_tm_definition);
        }
    } else if let Some(ref matches) = app.subcommand_matches("convert") {
        if let Some(input_path) = matches.value_of(sdf_input_name) {
            if let Some(output_path) = matches.value_of(tm_output_name) {
                return convert(input_path, output_path, &converter::convert_sdf_to_wot_tm);
            } else if let Some(output_path) = matches.value_of(sdf_output_name) {
                return write_to_another_file(input_path, output_path);
            }
        } else if let Some(input_path) = matches.value_of(tm_input_name) {
            if let Some(output_path) = matches.value_of(sdf_output_name) {
                return convert(input_path, output_path, &converter::convert_wot_tm_to_sdf);
            } else if let Some(output_path) = matches.value_of(tm_output_name) {
                return write_to_another_file(input_path, output_path);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_dir() {
        let _ = fs::create_dir_all("test_output");
    }

    #[test]
    fn write_to_another_file_test() {
        create_test_dir();
        assert!(
            write_to_another_file("examples/sdf/example.sdf.json", "test_output/barfoo.json")
                .is_ok()
        );
    }

    #[test]
    fn convert_test() {
        create_test_dir();
        let input_path = "examples/sdf/example.sdf.json";
        let output_path = "test_output/foobar.tm.json";
        assert!(convert(input_path, output_path, &converter::convert_sdf_to_wot_tm).is_ok())
    }

    #[test]
    fn determine_path_type_test() {
        assert_eq!(
            InputPathType::ValidUrl,
            determine_path_type("https://example.org")
        );
        assert_eq!(
            InputPathType::InvalidUrl,
            determine_path_type("coap://example.org")
        );
        assert_eq!(InputPathType::File, determine_path_type("file://foobar"));
        assert_eq!(InputPathType::File, determine_path_type("foobar"));
        assert_eq!(InputPathType::File, determine_path_type("./foobar"));
        assert_eq!(InputPathType::File, determine_path_type("~/foobar"));
        if cfg!(windows) {
            assert_eq!(InputPathType::File, determine_path_type("C:\\foobar"));
        }
    }
}
